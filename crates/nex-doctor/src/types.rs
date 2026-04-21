use serde::{Deserialize, Serialize};

// -- Topology graph --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineTopology {
    pub nodes: Vec<PipelineNode>,
    pub edges: Vec<PipelineEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineNode {
    pub id: String,
    pub node_type: NodeType,
    pub label: String,
    pub source_file: Option<String>,
    pub config: NodeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    KafkaTopic,
    FlinkJob,
    KafkaConnector,
    MongoDB,
    Parquet,
    DeadLetterQueue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeConfig {
    KafkaTopic {
        topic_name: String,
        partitions: Option<i32>,
        schema_subject: Option<String>,
    },
    FlinkJob {
        job_name: String,
        operators: Vec<String>,
        parallelism: Option<i32>,
    },
    KafkaConnector {
        connector_name: String,
        connector_class: String,
        target_database: Option<String>,
        target_collection: Option<String>,
    },
    #[serde(rename = "mongodb")]
    MongoDB {
        database: String,
        collection: String,
    },
    Parquet {
        output_path: String,
    },
    DeadLetterQueue {
        topic_name: String,
        source_connector: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    DataFlow,
    ErrorFlow,
    SchemaBinding,
}

// -- Live metrics --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub node_id: String,
    pub status: NodeStatus,
    pub message: String,
    pub last_checked: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Healthy,
    Warning,
    Failed,
    Unknown,
    NotRunning,
}

// -- Diagnostic issues --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticIssue {
    pub severity: IssueSeverity,
    pub category: String,
    pub title: String,
    pub description: String,
    pub affected_nodes: Vec<String>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IssueSeverity {
    Critical,
    Warning,
    Info,
}

// -- Connectivity summary (not an issue -- just status) --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String, // "ok" or "fail"
    pub message: String,
}

// -- Diagnostic report --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    pub topology: PipelineTopology,
    pub metrics: Vec<NodeMetrics>,
    pub issues: Vec<DiagnosticIssue>,
    pub services: Vec<ServiceStatus>,
    pub timestamp: String,
    pub duration_ms: u64,
}

// -- Config passed from frontend --

#[derive(Debug, Clone, Deserialize)]
pub struct DoctorConfig {
    pub flink_url: Option<String>,
    pub connect_url: Option<String>,
    pub mongo_uri: Option<String>,
    pub parquet_dirs: Vec<String>,
    pub dlq_topic_prefix: String,
}
