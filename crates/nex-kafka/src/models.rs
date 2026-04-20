use serde::{Deserialize, Serialize};

/// Connection configuration for a Kafka cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub security_protocol: String,
    pub sasl_mechanism: Option<String>,
    pub sasl_username: Option<String>,
    pub sasl_password: Option<String>,
}

/// Metadata about a Kafka topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicInfo {
    pub name: String,
    pub partitions: i32,
    pub replication_factor: i32,
    pub message_count_estimate: Option<i64>,
}

/// Watermark offsets for a single partition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionOffsets {
    pub partition: i32,
    pub low: i64,
    pub high: i64,
}

/// Options for consuming messages from a topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumeOptions {
    pub topic: String,
    pub limit: usize,
    pub offset: String, // "earliest", "latest", or numeric string
    pub partition: Option<i32>,
    pub timeout_ms: u64,
    pub filter: Option<MessageFilter>,
}

/// Simple field-level filter for messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFilter {
    pub field: String,
    pub op: String, // "eq", "neq", "gt", "lt", "gte", "lte", "contains"
    pub value: String,
}

/// A raw message as consumed from Kafka (before decoding).
#[derive(Debug, Clone)]
pub struct RawMessage {
    pub offset: i64,
    pub partition: i32,
    pub timestamp: Option<i64>,
    pub key: Option<Vec<u8>>,
    pub value: Option<Vec<u8>>,
    pub headers: Vec<(String, Vec<u8>)>,
}

/// A decoded, serializable message ready for the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedMessage {
    pub offset: i64,
    pub partition: i32,
    pub timestamp: Option<i64>,
    pub key: Option<String>,
    pub value: String,
    pub headers: Vec<(String, String)>,
    pub value_format: String, // "json", "avro", "raw"
    pub size_bytes: usize,
}

/// PII masking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiConfig {
    pub patterns: Vec<PiiPatternDef>,
    pub field_names: Vec<String>,
}

/// A named regex pattern for PII detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PiiPatternDef {
    pub name: String,
    pub pattern: String,
    pub enabled: bool,
}
