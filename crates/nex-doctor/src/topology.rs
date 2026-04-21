use std::collections::HashMap;
use std::path::Path;

use crate::error::DoctorError;
use crate::types::{
    EdgeType, NodeConfig, NodeType, PipelineEdge, PipelineNode, PipelineTopology,
};

/// Build a pipeline topology from .proc files in the given directory.
///
/// Parses each .proc file, extracts source/sink declarations, and builds
/// a directed graph of pipeline nodes and edges.
pub fn build_topology_from_proc_dir(proc_dir: &str) -> Result<PipelineTopology, DoctorError> {
    let dir = Path::new(proc_dir);
    if !dir.is_dir() {
        return Ok(PipelineTopology {
            nodes: vec![],
            edges: vec![],
        });
    }

    let mut proc_files: Vec<std::path::PathBuf> = Vec::new();
    collect_proc_files(dir, &mut proc_files)?;

    if proc_files.is_empty() {
        return Ok(PipelineTopology {
            nodes: vec![],
            edges: vec![],
        });
    }

    let mut builder = TopologyBuilder::new();

    for path in &proc_files {
        let source = std::fs::read_to_string(path)?;
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        match nexflow_parser::parse_proc(&source) {
            Ok(program) => {
                for process in &program.processes {
                    builder.add_process(process, &file_name);
                }
            }
            Err(e) => {
                tracing::warn!(file = %file_name, error = %e, "failed to parse .proc file");
            }
        }
    }

    Ok(builder.build())
}

/// Build a minimal fallback topology from just the configured Kafka topics.
///
/// Used when no .proc files exist yet.
pub fn build_fallback_topology(topic_names: &[String]) -> PipelineTopology {
    let nodes: Vec<PipelineNode> = topic_names
        .iter()
        .map(|name| PipelineNode {
            id: format!("topic:{name}"),
            node_type: NodeType::KafkaTopic,
            label: name.clone(),
            source_file: None,
            config: NodeConfig::KafkaTopic {
                topic_name: name.clone(),
                partitions: None,
                schema_subject: None,
            },
        })
        .collect();

    PipelineTopology {
        nodes,
        edges: vec![],
    }
}

struct TopologyBuilder {
    nodes: HashMap<String, PipelineNode>,
    edges: Vec<PipelineEdge>,
    edge_counter: usize,
}

impl TopologyBuilder {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            edge_counter: 0,
        }
    }

    fn add_process(
        &mut self,
        process: &nexflow_parser::ast::proc::ProcessDef,
        source_file: &str,
    ) {
        let job_id = format!("job:{}", process.name);

        // Collect operators from the process body
        let mut operators = Vec::new();
        for stmt in &process.body {
            match stmt {
                nexflow_parser::ast::proc::ProcessStatement::Transform { using, .. } => {
                    operators.push(format!("transform:{using}"));
                }
                nexflow_parser::ast::proc::ProcessStatement::Join { join_type, .. } => {
                    operators.push(format!("join:{join_type}"));
                }
                nexflow_parser::ast::proc::ProcessStatement::Window { window_type, .. } => {
                    operators.push(format!("window:{window_type}"));
                }
                nexflow_parser::ast::proc::ProcessStatement::Route { .. } => {
                    operators.push("route".into());
                }
                nexflow_parser::ast::proc::ProcessStatement::Enrich { .. } => {
                    operators.push("enrich".into());
                }
                nexflow_parser::ast::proc::ProcessStatement::Aggregate { .. } => {
                    operators.push("aggregate".into());
                }
                _ => {}
            }
        }

        let parallelism = process
            .execution
            .as_ref()
            .and_then(|e| e.parallelism)
            .map(|p| p as i32);

        // Add the Flink job node
        self.nodes.entry(job_id.clone()).or_insert_with(|| PipelineNode {
            id: job_id.clone(),
            node_type: NodeType::FlinkJob,
            label: process.name.clone(),
            source_file: Some(source_file.to_string()),
            config: NodeConfig::FlinkJob {
                job_name: process.name.clone(),
                operators,
                parallelism,
            },
        });

        // Process all statements for sources and sinks
        let all_stmts = self.collect_statements(process);
        for stmt in &all_stmts {
            match stmt {
                nexflow_parser::ast::proc::ProcessStatement::Receive {
                    source_type,
                    source,
                    schema,
                    ..
                } => {
                    if source_type == "kafka" && !source.is_empty() {
                        let topic_id = format!("topic:{source}");
                        self.ensure_topic_node(&topic_id, source, schema.as_deref(), source_file);
                        self.add_edge(&topic_id, &job_id, EdgeType::DataFlow);
                    }
                }
                nexflow_parser::ast::proc::ProcessStatement::Emit {
                    sink_type,
                    sink,
                    schema,
                    ..
                } => {
                    match sink_type.as_str() {
                        "" => {} // No sink type -- batch process, skip
                        "kafka" if !sink.is_empty() => {
                            let topic_id = format!("topic:{sink}");
                            self.ensure_topic_node(&topic_id, sink, schema.as_deref(), source_file);
                            self.add_edge(&job_id, &topic_id, EdgeType::DataFlow);
                        }
                        "mongodb" if !sink.is_empty() => {
                            // sink format: "db.collection"
                            let parts: Vec<&str> = sink.splitn(2, '.').collect();
                            let (db, coll) = if parts.len() == 2 {
                                (parts[0].to_string(), parts[1].to_string())
                            } else {
                                (sink.clone(), "default".to_string())
                            };
                            let mongo_id = format!("mongo:{sink}");
                            self.nodes.entry(mongo_id.clone()).or_insert_with(|| PipelineNode {
                                id: mongo_id.clone(),
                                node_type: NodeType::MongoDB,
                                label: sink.clone(),
                                source_file: Some(source_file.to_string()),
                                config: NodeConfig::MongoDB {
                                    database: db,
                                    collection: coll,
                                },
                            });
                            self.add_edge(&job_id, &mongo_id, EdgeType::DataFlow);
                        }
                        "parquet" if !sink.is_empty() => {
                            let parquet_id = format!("parquet:{sink}");
                            self.nodes.entry(parquet_id.clone()).or_insert_with(|| PipelineNode {
                                id: parquet_id.clone(),
                                node_type: NodeType::Parquet,
                                label: sink.clone(),
                                source_file: Some(source_file.to_string()),
                                config: NodeConfig::Parquet {
                                    output_path: sink.clone(),
                                },
                            });
                            self.add_edge(&job_id, &parquet_id, EdgeType::DataFlow);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Check resilience block for dead_letter references
        if let Some(resilience) = &process.resilience {
            // Simple heuristic: look for "dead_letter" keyword in raw resilience text
            if let Some(dlq_topic) = extract_dead_letter_topic(&resilience.raw) {
                let dlq_id = format!("dlq:{dlq_topic}");
                self.nodes.entry(dlq_id.clone()).or_insert_with(|| PipelineNode {
                    id: dlq_id.clone(),
                    node_type: NodeType::DeadLetterQueue,
                    label: dlq_topic.clone(),
                    source_file: Some(source_file.to_string()),
                    config: NodeConfig::DeadLetterQueue {
                        topic_name: dlq_topic,
                        source_connector: Some(process.name.clone()),
                    },
                });
                self.add_edge(&job_id, &dlq_id, EdgeType::ErrorFlow);
            }
        }
    }

    fn collect_statements<'a>(
        &self,
        process: &'a nexflow_parser::ast::proc::ProcessDef,
    ) -> Vec<&'a nexflow_parser::ast::proc::ProcessStatement> {
        let mut stmts: Vec<&nexflow_parser::ast::proc::ProcessStatement> = process.body.iter().collect();
        for phase in &process.phases {
            stmts.extend(phase.statements.iter());
        }
        stmts
    }

    fn ensure_topic_node(
        &mut self,
        topic_id: &str,
        topic_name: &str,
        schema: Option<&str>,
        source_file: &str,
    ) {
        self.nodes.entry(topic_id.to_string()).or_insert_with(|| PipelineNode {
            id: topic_id.to_string(),
            node_type: NodeType::KafkaTopic,
            label: topic_name.to_string(),
            source_file: Some(source_file.to_string()),
            config: NodeConfig::KafkaTopic {
                topic_name: topic_name.to_string(),
                partitions: None,
                schema_subject: schema.map(String::from),
            },
        });
    }

    fn add_edge(&mut self, source: &str, target: &str, edge_type: EdgeType) {
        self.edge_counter += 1;
        self.edges.push(PipelineEdge {
            id: format!("e{}", self.edge_counter),
            source: source.to_string(),
            target: target.to_string(),
            edge_type,
        });
    }

    fn build(self) -> PipelineTopology {
        PipelineTopology {
            nodes: self.nodes.into_values().collect(),
            edges: self.edges,
        }
    }
}

/// Recursively collect all .proc files under a directory.
fn collect_proc_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> Result<(), std::io::Error> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_proc_files(&path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("proc") {
            out.push(path);
        }
    }
    Ok(())
}

/// Extract a dead_letter topic name from a raw resilience block.
///
/// Looks for patterns like `dead_letter "topic-name"` or `dead_letter topic-name`.
fn extract_dead_letter_topic(raw: &str) -> Option<String> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("dead_letter") {
            let topic = rest.trim().trim_matches('"').trim().to_string();
            if !topic.is_empty() {
                return Some(topic);
            }
        }
    }
    None
}
