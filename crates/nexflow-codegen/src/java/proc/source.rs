// NexCore -- Nexflow Codegen: Flink Source Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Flink source connectors from `ProcessStatement::Receive`.

use std::fmt::Write;

use crate::java::naming::to_pascal_case;

/// Write a Kafka source connector.
pub fn write_kafka_source(
    out: &mut String,
    name: &str,
    topic: &str,
    schema: Option<&str>,
    key: Option<&str>,
    stream_var: &str,
) {
    let schema_class = schema
        .map(|s| to_pascal_case(s))
        .unwrap_or_else(|| "org.apache.avro.generic.GenericRecord".to_string());

    writeln!(out, "        // Source: {name} from Kafka topic \"{topic}\"").unwrap();
    writeln!(
        out,
        "        KafkaSource<{schema_class}> {stream_var}Source = KafkaSource"
    )
    .unwrap();
    writeln!(
        out,
        "            .<{schema_class}>builder()"
    )
    .unwrap();
    writeln!(
        out,
        "            .setBootstrapServers(KAFKA_BOOTSTRAP_SERVERS)"
    )
    .unwrap();
    writeln!(
        out,
        "            .setTopics(\"{topic}\")"
    )
    .unwrap();
    writeln!(
        out,
        "            .setGroupId(\"{name}-consumer\")"
    )
    .unwrap();
    writeln!(
        out,
        "            .setStartingOffsets(org.apache.flink.connector.kafka.source.enumerator.initializer.OffsetsInitializer.latest())"
    )
    .unwrap();
    writeln!(
        out,
        "            .setValueOnlyDeserializer(org.apache.flink.formats.avro.AvroDeserializationSchema.forSpecific({schema_class}.class))"
    )
    .unwrap();
    writeln!(out, "            .build();").unwrap();
    writeln!(out).unwrap();

    // Watermark strategy
    writeln!(
        out,
        "        DataStream<{schema_class}> {stream_var} = env.fromSource("
    )
    .unwrap();
    writeln!(
        out,
        "            {stream_var}Source,"
    )
    .unwrap();
    writeln!(
        out,
        "            WatermarkStrategy.forBoundedOutOfOrderness(java.time.Duration.ofSeconds(5)),"
    )
    .unwrap();
    writeln!(
        out,
        "            \"{name}\""
    )
    .unwrap();
    writeln!(out, "        );").unwrap();

    // Key-by if specified
    if let Some(key_field) = key {
        let getter = format!("get{}()", to_pascal_case(key_field));
        writeln!(out).unwrap();
        writeln!(
            out,
            "        // Keyed by {key_field}"
        )
        .unwrap();
        writeln!(
            out,
            "        var {stream_var}Keyed = {stream_var}.keyBy(record -> record.{getter} != null ? record.{getter}.toString() : \"\");"
        )
        .unwrap();
    }

    writeln!(out).unwrap();
}

/// Write a generic source (non-Kafka).
pub fn write_generic_source(
    out: &mut String,
    name: &str,
    source_type: &str,
    source: &str,
    stream_var: &str,
) {
    writeln!(
        out,
        "        // Source: {name} ({source_type}: \"{source}\")"
    )
    .unwrap();
    writeln!(
        out,
        "        // TODO: Implement {source_type} source connector for \"{source}\""
    )
    .unwrap();
    writeln!(
        out,
        "        DataStream<org.apache.avro.generic.GenericRecord> {stream_var} = env"
    )
    .unwrap();
    writeln!(
        out,
        "            .fromElements() // Placeholder"
    )
    .unwrap();
    writeln!(
        out,
        "            .name(\"{name}\");"
    )
    .unwrap();
    writeln!(out).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_source() {
        let mut out = String::new();
        write_kafka_source(
            &mut out,
            "orders",
            "orders-inbound",
            Some("order_event"),
            Some("order_id"),
            "rawOrders",
        );

        assert!(out.contains("KafkaSource<OrderEvent>"));
        assert!(out.contains("\"orders-inbound\""));
        assert!(out.contains("\"orders-consumer\""));
        assert!(out.contains("AvroDeserializationSchema.forSpecific(OrderEvent.class)"));
        assert!(out.contains("WatermarkStrategy.forBoundedOutOfOrderness"));
        assert!(out.contains("keyBy(record -> record.getOrderId()"));
    }

    #[test]
    fn test_kafka_source_no_key() {
        let mut out = String::new();
        write_kafka_source(
            &mut out,
            "events",
            "events-topic",
            Some("event_log"),
            None,
            "rawEvents",
        );

        assert!(out.contains("KafkaSource<EventLog>"));
        assert!(!out.contains("keyBy"));
    }

    #[test]
    fn test_generic_source() {
        let mut out = String::new();
        write_generic_source(&mut out, "files", "parquet", "/data/input", "rawFiles");

        assert!(out.contains("parquet: \"/data/input\""));
        assert!(out.contains("TODO"));
    }
}
