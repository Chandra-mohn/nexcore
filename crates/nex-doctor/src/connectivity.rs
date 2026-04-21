use crate::types::{
    DiagnosticIssue, IssueSeverity, NodeMetrics, NodeStatus, NodeType, PipelineNode,
    ServiceStatus,
};
use nex_kafka::{KafkaClient, KafkaConfig};
use nex_registry::{RegistryClient, RegistryConfig};

/// Check connectivity to all configured services.
///
/// Checks run regardless of whether matching topology nodes exist --
/// connectivity status is useful even with an empty topology.
/// When nodes exist, their metrics are updated with the result.
pub async fn check_connectivity(
    nodes: &[PipelineNode],
    kafka_config: Option<&KafkaConfig>,
    registry_config: Option<&RegistryConfig>,
    flink_url: Option<&str>,
    connect_url: Option<&str>,
) -> (Vec<NodeMetrics>, Vec<DiagnosticIssue>, Vec<ServiceStatus>) {
    let mut metrics = Vec::new();
    let mut issues = Vec::new();
    let mut services = Vec::new();
    let now = chrono_now();

    // -- Kafka --
    if let Some(config) = kafka_config {
        let config = config.clone();
        let result = tokio::task::spawn_blocking(move || {
            let client = KafkaClient::new(&config)?;
            client.test_connection()
        })
        .await;

        match result {
            Ok(Ok(msg)) => {
                for node in nodes.iter().filter(|n| {
                    n.node_type == NodeType::KafkaTopic
                        || n.node_type == NodeType::DeadLetterQueue
                }) {
                    metrics.push(NodeMetrics {
                        node_id: node.id.clone(),
                        status: NodeStatus::Healthy,
                        message: msg.clone(),
                        last_checked: now.clone(),
                    });
                }
                services.push(ServiceStatus {
                    name: "Kafka".into(),
                    status: "ok".into(),
                    message: msg,
                });
            }
            Ok(Err(e)) => {
                let err_msg = e.to_string();
                mark_nodes_failed(nodes, &[NodeType::KafkaTopic, NodeType::DeadLetterQueue], "Kafka broker unreachable", &now, &mut metrics);
                issues.push(DiagnosticIssue {
                    severity: IssueSeverity::Critical,
                    category: "connectivity".into(),
                    title: "Kafka broker unreachable".into(),
                    description: err_msg,
                    affected_nodes: collect_node_ids(nodes, &[NodeType::KafkaTopic, NodeType::DeadLetterQueue]),
                    suggestions: vec![
                        "Check that Kafka is running and accessible".into(),
                        "Verify bootstrap servers in project settings".into(),
                    ],
                });
            }
            Err(e) => {
                mark_nodes_failed(nodes, &[NodeType::KafkaTopic, NodeType::DeadLetterQueue], "Kafka health check failed", &now, &mut metrics);
                issues.push(DiagnosticIssue {
                    severity: IssueSeverity::Critical,
                    category: "connectivity".into(),
                    title: "Kafka broker unreachable".into(),
                    description: e.to_string(),
                    affected_nodes: collect_node_ids(nodes, &[NodeType::KafkaTopic, NodeType::DeadLetterQueue]),
                    suggestions: vec![
                        "Check that Kafka is running and accessible".into(),
                        "Verify bootstrap servers in project settings".into(),
                    ],
                });
            }
        }
    }

    // -- Schema Registry --
    if let Some(config) = registry_config {
        match RegistryClient::new(config) {
            Ok(client) => match client.test_connection(config).await {
                Ok(msg) => {
                    services.push(ServiceStatus {
                        name: "Schema Registry".into(),
                        status: "ok".into(),
                        message: msg,
                    });
                }
                Err(e) => {
                    issues.push(DiagnosticIssue {
                        severity: IssueSeverity::Critical,
                        category: "connectivity".into(),
                        title: "Schema Registry unreachable".into(),
                        description: e.to_string(),
                        affected_nodes: vec![],
                        suggestions: vec![
                            "Check that Schema Registry is running".into(),
                            "Verify URL in project settings".into(),
                        ],
                    });
                }
            },
            Err(e) => {
                issues.push(DiagnosticIssue {
                    severity: IssueSeverity::Critical,
                    category: "connectivity".into(),
                    title: "Schema Registry unreachable".into(),
                    description: e.to_string(),
                    affected_nodes: vec![],
                    suggestions: vec!["Verify registry URL in project settings".into()],
                });
            }
        }
    }

    // -- Flink REST API --
    if let Some(url) = flink_url {
        match check_flink_connectivity(url).await {
            Ok(msg) => {
                for node in nodes.iter().filter(|n| n.node_type == NodeType::FlinkJob) {
                    metrics.push(NodeMetrics {
                        node_id: node.id.clone(),
                        status: NodeStatus::Healthy,
                        message: msg.clone(),
                        last_checked: now.clone(),
                    });
                }
                services.push(ServiceStatus {
                    name: "Flink".into(),
                    status: "ok".into(),
                    message: msg,
                });
            }
            Err(e) => {
                mark_nodes_failed(nodes, &[NodeType::FlinkJob], "Flink cluster unreachable", &now, &mut metrics);
                issues.push(DiagnosticIssue {
                    severity: IssueSeverity::Critical,
                    category: "connectivity".into(),
                    title: "Flink cluster unreachable".into(),
                    description: e,
                    affected_nodes: collect_node_ids(nodes, &[NodeType::FlinkJob]),
                    suggestions: vec![
                        "Check that Flink is running".into(),
                        "Verify Flink URL in Doctor settings".into(),
                    ],
                });
            }
        }
    }

    // -- Kafka Connect REST API --
    if let Some(url) = connect_url {
        match check_connect_connectivity(url).await {
            Ok(msg) => {
                for node in nodes
                    .iter()
                    .filter(|n| n.node_type == NodeType::KafkaConnector)
                {
                    metrics.push(NodeMetrics {
                        node_id: node.id.clone(),
                        status: NodeStatus::Healthy,
                        message: msg.clone(),
                        last_checked: now.clone(),
                    });
                }
                services.push(ServiceStatus {
                    name: "Kafka Connect".into(),
                    status: "ok".into(),
                    message: msg,
                });
            }
            Err(e) => {
                mark_nodes_failed(nodes, &[NodeType::KafkaConnector], "Kafka Connect unreachable", &now, &mut metrics);
                issues.push(DiagnosticIssue {
                    severity: IssueSeverity::Warning,
                    category: "connectivity".into(),
                    title: "Kafka Connect unreachable".into(),
                    description: e,
                    affected_nodes: collect_node_ids(nodes, &[NodeType::KafkaConnector]),
                    suggestions: vec![
                        "Check that Kafka Connect is running".into(),
                        "Verify Connect URL in Doctor settings".into(),
                    ],
                });
            }
        }
    }

    // MongoDB nodes -- mark as unknown for Phase 1
    for node in nodes.iter().filter(|n| n.node_type == NodeType::MongoDB) {
        metrics.push(NodeMetrics {
            node_id: node.id.clone(),
            status: NodeStatus::Unknown,
            message: "MongoDB checks available in Phase 4".into(),
            last_checked: now.clone(),
        });
    }

    // Parquet nodes -- mark as unknown for Phase 1
    for node in nodes.iter().filter(|n| n.node_type == NodeType::Parquet) {
        metrics.push(NodeMetrics {
            node_id: node.id.clone(),
            status: NodeStatus::Unknown,
            message: "Parquet checks available in Phase 4".into(),
            last_checked: now.clone(),
        });
    }

    (metrics, issues, services)
}

fn mark_nodes_failed(
    nodes: &[PipelineNode],
    types: &[NodeType],
    message: &str,
    now: &str,
    metrics: &mut Vec<NodeMetrics>,
) {
    for node in nodes.iter().filter(|n| types.contains(&n.node_type)) {
        metrics.push(NodeMetrics {
            node_id: node.id.clone(),
            status: NodeStatus::Failed,
            message: message.into(),
            last_checked: now.to_string(),
        });
    }
}

fn collect_node_ids(nodes: &[PipelineNode], types: &[NodeType]) -> Vec<String> {
    nodes
        .iter()
        .filter(|n| types.contains(&n.node_type))
        .map(|n| n.id.clone())
        .collect()
}

async fn check_flink_connectivity(base_url: &str) -> Result<String, String> {
    let url = format!("{base_url}/overview");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        let slots = body
            .get("slots-total")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let running = body
            .get("jobs-running")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        Ok(format!("Connected -- {slots} slots, {running} jobs running"))
    } else {
        Err(format!("Flink returned HTTP {}", resp.status()))
    }
}

async fn check_connect_connectivity(base_url: &str) -> Result<String, String> {
    let url = format!("{base_url}/connectors");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let connectors: Vec<String> = resp.json().await.map_err(|e| e.to_string())?;
        Ok(format!(
            "Connected -- {} connectors",
            connectors.len()
        ))
    } else {
        Err(format!("Kafka Connect returned HTTP {}", resp.status()))
    }
}

fn chrono_now() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{now}")
}
