// NexCore -- Nexflow Codegen: Flink Sink Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Flink sink connectors from `ProcessStatement::Emit`.

use std::fmt::Write;

use crate::java::naming::to_pascal_case;

/// Write a Kafka sink connector.
pub fn write_kafka_sink(
    out: &mut String,
    name: &str,
    topic: &str,
    schema: Option<&str>,
    stream_var: &str,
) {
    let schema_class = schema
        .map(|s| to_pascal_case(s))
        .unwrap_or_else(|| "org.apache.avro.generic.GenericRecord".to_string());

    writeln!(out, "        // Sink: {name} to Kafka topic \"{topic}\"").unwrap();
    writeln!(
        out,
        "        KafkaSink<{schema_class}> {name}Sink = KafkaSink"
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
        "            .setRecordSerializer("
    )
    .unwrap();
    writeln!(
        out,
        "                org.apache.flink.connector.kafka.sink.KafkaRecordSerializationSchema.<{schema_class}>builder()"
    )
    .unwrap();
    writeln!(
        out,
        "                    .setTopic(\"{topic}\")"
    )
    .unwrap();
    writeln!(
        out,
        "                    .setValueSerializationSchema(org.apache.flink.formats.avro.AvroSerializationSchema.forSpecific({schema_class}.class))"
    )
    .unwrap();
    writeln!(out, "                    .build()").unwrap();
    writeln!(out, "            )").unwrap();
    writeln!(
        out,
        "            .setDeliveryGuarantee(org.apache.flink.connector.base.DeliveryGuarantee.AT_LEAST_ONCE)"
    )
    .unwrap();
    writeln!(out, "            .build();").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "        {stream_var}.sinkTo({name}Sink).name(\"sink-{topic}\");"
    )
    .unwrap();
    writeln!(out).unwrap();
}

/// Write a generic sink (non-Kafka).
pub fn write_generic_sink(
    out: &mut String,
    name: &str,
    sink_type: &str,
    sink: &str,
    stream_var: &str,
) {
    writeln!(
        out,
        "        // Sink: {name} ({sink_type}: \"{sink}\")"
    )
    .unwrap();
    writeln!(
        out,
        "        // TODO: Implement {sink_type} sink connector for \"{sink}\""
    )
    .unwrap();
    writeln!(
        out,
        "        {stream_var}.print().name(\"sink-{name}\"); // Placeholder"
    )
    .unwrap();
    writeln!(out).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_sink() {
        let mut out = String::new();
        write_kafka_sink(
            &mut out,
            "processedOrders",
            "orders-processed",
            Some("processed_order_event"),
            "processedStream",
        );

        assert!(out.contains("KafkaSink<ProcessedOrderEvent>"));
        assert!(out.contains("\"orders-processed\""));
        assert!(out.contains("AvroSerializationSchema.forSpecific(ProcessedOrderEvent.class)"));
        assert!(out.contains("AT_LEAST_ONCE"));
        assert!(out.contains("processedStream.sinkTo(processedOrdersSink)"));
    }
}
