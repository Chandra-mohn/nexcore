// NexCore -- Nexflow Codegen: Flink Job Class Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates `*Job.java` -- the main Flink streaming job class.
//!
//! Produces:
//! - `main()` entry point with StreamExecutionEnvironment
//! - `buildPipeline()` that wires sources -> operators -> sinks
//! - Checkpoint configuration from execution block
//! - Configuration constants (Kafka bootstrap, parallelism, etc.)

use std::fmt::Write;

use nexflow_parser::ast::proc::{ProcessDef, ProcessStatement};

use crate::java::naming::{to_camel_case, to_pascal_case};

use super::operator;
use super::sink;
use super::source;

const DEFAULT_PACKAGE: &str = "com.nexflow.proc";

/// Generate a Flink Job Java class from a `ProcessDef`.
///
/// Returns `(filename, content)`.
pub fn generate_job(process: &ProcessDef) -> (String, String) {
    let class_name = format!("{}Job", to_pascal_case(&process.name));
    let filename = format!("{class_name}.java");
    let parallelism = process
        .execution
        .as_ref()
        .and_then(|e| e.parallelism)
        .unwrap_or(1);

    let mut out = String::with_capacity(16384);

    // Package + imports
    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();
    write_imports(&mut out);
    writeln!(out).unwrap();

    // Class
    writeln!(out, "/**").unwrap();
    writeln!(out, " * Flink streaming job: {}", process.name).unwrap();
    writeln!(
        out,
        " * Generated from {}.proc -- DO NOT EDIT.",
        process.name
    )
    .unwrap();
    writeln!(out, " */").unwrap();
    writeln!(out, "public class {class_name} {{").unwrap();
    writeln!(out).unwrap();

    // Constants
    writeln!(
        out,
        "    private static final String JOB_NAME = \"{}\";",
        process.name
    )
    .unwrap();
    writeln!(
        out,
        "    private static final int DEFAULT_PARALLELISM = {parallelism};"
    )
    .unwrap();
    writeln!(
        out,
        "    private static final String KAFKA_BOOTSTRAP_SERVERS = System.getenv().getOrDefault(\"KAFKA_BOOTSTRAP_SERVERS\", \"localhost:9092\");"
    )
    .unwrap();
    writeln!(out).unwrap();

    // main()
    writeln!(
        out,
        "    public static void main(String[] args) throws Exception {{"
    )
    .unwrap();
    writeln!(
        out,
        "        StreamExecutionEnvironment env = StreamExecutionEnvironment.getExecutionEnvironment();"
    )
    .unwrap();
    writeln!(
        out,
        "        env.setParallelism(DEFAULT_PARALLELISM);"
    )
    .unwrap();

    // Checkpoint config
    if let Some(exec) = &process.execution {
        if let Some(interval) = &exec.checkpoint_interval {
            let ms = super::operator::parse_duration_to_ms_pub(interval);
            writeln!(
                out,
                "        env.enableCheckpointing({ms}L, org.apache.flink.streaming.api.CheckpointingMode.EXACTLY_ONCE);"
            )
            .unwrap();
        }
    }

    writeln!(out).unwrap();
    writeln!(
        out,
        "        new {class_name}().buildPipeline(env);"
    )
    .unwrap();
    writeln!(out, "        env.execute(JOB_NAME);").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // buildPipeline()
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Build the streaming pipeline.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public void buildPipeline(StreamExecutionEnvironment env) {{"
    )
    .unwrap();

    // Process all statements
    let mut current_stream = "input".to_string();
    let mut step_idx = 0;

    for stmt in &process.body {
        match stmt {
            ProcessStatement::Receive {
                name,
                source_type,
                source: src,
                schema,
                key,
                ..
            } => {
                let stream_var = to_camel_case(name);
                if source_type == "kafka" {
                    source::write_kafka_source(
                        &mut out,
                        name,
                        src,
                        schema.as_deref(),
                        key.as_deref(),
                        &stream_var,
                    );
                } else {
                    source::write_generic_source(
                        &mut out,
                        name,
                        source_type,
                        src,
                        &stream_var,
                    );
                }
                current_stream = stream_var;
            }
            ProcessStatement::Emit {
                name,
                sink_type,
                sink: snk,
                schema,
                ..
            } => {
                if sink_type == "kafka" {
                    sink::write_kafka_sink(
                        &mut out,
                        name,
                        snk,
                        schema.as_deref(),
                        &current_stream,
                    );
                } else {
                    sink::write_generic_sink(
                        &mut out,
                        name,
                        sink_type,
                        snk,
                        &current_stream,
                    );
                }
            }
            other => {
                current_stream = operator::write_operator(
                    &mut out,
                    other,
                    &current_stream,
                    step_idx,
                );
                step_idx += 1;
            }
        }
    }

    writeln!(out, "    }}").unwrap();

    // Close class
    writeln!(out, "}}").unwrap();

    (filename, out)
}

fn write_imports(out: &mut String) {
    writeln!(
        out,
        "import org.apache.flink.streaming.api.environment.StreamExecutionEnvironment;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.streaming.api.datastream.DataStream;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.streaming.api.datastream.SingleOutputStreamOperator;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.api.common.eventtime.WatermarkStrategy;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.connector.kafka.source.KafkaSource;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.connector.kafka.sink.KafkaSink;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.streaming.api.functions.ProcessFunction;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.util.Collector;"
    )
    .unwrap();
    writeln!(
        out,
        "import org.apache.flink.util.OutputTag;"
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::proc::*;

    fn make_order_process() -> ProcessDef {
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
                    into: "normalized_orders".to_string(),
                },
                ProcessStatement::Transform {
                    input: "normalized_orders".to_string(),
                    using: "enrich_order".to_string(),
                    into: "enriched_orders".to_string(),
                },
                ProcessStatement::Emit {
                    name: "processed_orders".to_string(),
                    sink_type: "kafka".to_string(),
                    sink: "orders-processed".to_string(),
                    schema: Some("processed_order_event".to_string()),
                    key: Some("order_id".to_string()),
                },
            ],
            phases: Vec::new(),
            state: None,
            metrics: None,
            resilience: None,
        }
    }

    #[test]
    fn test_job_filename() {
        let (filename, _) = generate_job(&make_order_process());
        assert_eq!(filename, "OrderProcessingJob.java");
    }

    #[test]
    fn test_job_class_structure() {
        let (_, content) = generate_job(&make_order_process());

        assert!(content.contains("public class OrderProcessingJob {"));
        assert!(content.contains("JOB_NAME = \"order_processing\""));
        assert!(content.contains("DEFAULT_PARALLELISM = 4"));
        assert!(content.contains("public static void main(String[] args)"));
        assert!(content.contains("public void buildPipeline(StreamExecutionEnvironment env)"));
    }

    #[test]
    fn test_checkpoint_config() {
        let (_, content) = generate_job(&make_order_process());
        assert!(content.contains("enableCheckpointing(60000L"));
        assert!(content.contains("EXACTLY_ONCE"));
    }

    #[test]
    fn test_source_wiring() {
        let (_, content) = generate_job(&make_order_process());
        assert!(content.contains("KafkaSource<OrderEvent>"));
        assert!(content.contains("\"orders-inbound\""));
        assert!(content.contains("keyBy("));
    }

    #[test]
    fn test_transform_chain() {
        let (_, content) = generate_job(&make_order_process());
        assert!(content.contains("NormalizeOrderFunction"));
        assert!(content.contains("EnrichOrderFunction"));
    }

    #[test]
    fn test_sink_wiring() {
        let (_, content) = generate_job(&make_order_process());
        assert!(content.contains("KafkaSink<ProcessedOrderEvent>"));
        assert!(content.contains("\"orders-processed\""));
    }

    #[test]
    fn test_env_from_config() {
        let (_, content) = generate_job(&make_order_process());
        assert!(content.contains("KAFKA_BOOTSTRAP_SERVERS"));
        assert!(content.contains("System.getenv()"));
    }
}
