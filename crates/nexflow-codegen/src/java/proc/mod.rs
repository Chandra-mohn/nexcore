// NexCore -- Nexflow Codegen: Flink Pipeline Generator (L1)
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Flink streaming job classes from ProcDSL ASTs.
//!
//! **L1 Generator** -- depends on L0 (runtime), L2 (schemas), L3 (transforms), L4 (rules).
//!
//! For each `ProcessDef`, produces a `*Job.java` with:
//! - `main()` entry point with `StreamExecutionEnvironment`
//! - `buildPipeline()` that wires sources -> operators -> sinks
//! - Kafka source/sink connectors with Avro serialization
//! - Pipeline operators: transform (L3), route (side outputs), window, join, enrich, aggregate

pub mod job;
pub mod operator;
pub mod sink;
pub mod source;

use std::collections::HashMap;

use nexflow_parser::ast::proc::ProcProgram;

use crate::GeneratedProject;

/// Configuration for Flink pipeline generation.
#[derive(Debug, Clone)]
pub struct ProcGenConfig {
    /// Java package name (default: `com.nexflow.proc`).
    pub package: String,
    /// Output directory prefix (default: `src/main/java/com/nexflow/proc`).
    pub java_dir: String,
}

impl Default for ProcGenConfig {
    fn default() -> Self {
        Self {
            package: "com.nexflow.proc".to_string(),
            java_dir: "src/main/java/com/nexflow/proc".to_string(),
        }
    }
}

/// Generate all Flink job classes for a proc program.
pub fn generate_java_proc(
    program: &ProcProgram,
    config: &ProcGenConfig,
) -> Result<GeneratedProject, String> {
    let mut files = HashMap::new();

    for process in &program.processes {
        let (filename, content) = job::generate_job(process);
        files.insert(format!("{}/{filename}", config.java_dir), content);
    }

    Ok(GeneratedProject { files })
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::proc::*;

    fn make_program() -> ProcProgram {
        ProcProgram {
            imports: Vec::new(),
            processes: vec![
                ProcessDef {
                    name: "order_processing".to_string(),
                    execution: Some(ExecutionBlock {
                        mode: Some("stream".to_string()),
                        parallelism: Some(4),
                        checkpoint_interval: Some("60s".to_string()),
                    }),
                    business_date: None,
                    processing_date: None,
                    markers: Vec::new(),
                    state_machine: None,
                    body: vec![
                        ProcessStatement::Receive {
                            name: "orders".to_string(),
                            source_type: "kafka".to_string(),
                            source: "orders-inbound".to_string(),
                            schema: Some("order_event".to_string()),
                            key: Some("order_id".to_string()),
                            options: Vec::new(),
                        },
                        ProcessStatement::Transform {
                            input: "orders".to_string(),
                            using: "normalize_order".to_string(),
                            into: "normalized".to_string(),
                        },
                        ProcessStatement::Emit {
                            name: "output".to_string(),
                            sink_type: "kafka".to_string(),
                            sink: "orders-processed".to_string(),
                            schema: Some("processed_order".to_string()),
                            key: Some("order_id".to_string()),
                        },
                    ],
                    phases: Vec::new(),
                    state: None,
                    metrics: None,
                    resilience: None,
                },
                ProcessDef {
                    name: "daily_settlement".to_string(),
                    execution: None,
                    business_date: None,
                    processing_date: None,
                    markers: Vec::new(),
                    state_machine: None,
                    body: vec![ProcessStatement::Receive {
                        name: "txns".to_string(),
                        source_type: "kafka".to_string(),
                        source: "daily-txns".to_string(),
                        schema: Some("transaction".to_string()),
                        key: None,
                        options: Vec::new(),
                    }],
                    phases: Vec::new(),
                    state: None,
                    metrics: None,
                    resilience: None,
                },
            ],
        }
    }

    #[test]
    fn test_generates_all_jobs() {
        let program = make_program();
        let config = ProcGenConfig::default();
        let project = generate_java_proc(&program, &config).unwrap();
        let dir = &config.java_dir;

        assert_eq!(project.files.len(), 2);
        assert!(project
            .files
            .contains_key(&format!("{dir}/OrderProcessingJob.java")));
        assert!(project
            .files
            .contains_key(&format!("{dir}/DailySettlementJob.java")));
    }

    #[test]
    fn test_job_has_pipeline() {
        let program = make_program();
        let config = ProcGenConfig::default();
        let project = generate_java_proc(&program, &config).unwrap();
        let dir = &config.java_dir;

        let content = &project.files[&format!("{dir}/OrderProcessingJob.java")];
        // Source -> Transform -> Sink pipeline
        assert!(content.contains("KafkaSource<OrderEvent>"));
        assert!(content.contains("NormalizeOrderFunction"));
        assert!(content.contains("KafkaSink<ProcessedOrder>"));
    }

    #[test]
    fn test_second_job_independent() {
        let program = make_program();
        let config = ProcGenConfig::default();
        let project = generate_java_proc(&program, &config).unwrap();
        let dir = &config.java_dir;

        let content = &project.files[&format!("{dir}/DailySettlementJob.java")];
        assert!(content.contains("public class DailySettlementJob"));
        assert!(content.contains("DEFAULT_PARALLELISM = 1")); // No execution block -> default
    }
}
