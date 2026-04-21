use crate::connectivity::{check_connectivity, check_data_flow};
use crate::types::{DiagnosticReport, DoctorConfig, PipelineTopology};
use nex_kafka::KafkaConfig;
use nex_registry::RegistryConfig;

/// Run a full diagnostic pass: connectivity + data flow checks.
pub async fn run_diagnostics(
    topology: &PipelineTopology,
    kafka_config: Option<&KafkaConfig>,
    registry_config: Option<&RegistryConfig>,
    doctor_config: &DoctorConfig,
) -> DiagnosticReport {
    let start = std::time::Instant::now();

    let (mut metrics, mut issues, services) = check_connectivity(
        &topology.nodes,
        kafka_config,
        registry_config,
        doctor_config.flink_url.as_deref(),
        doctor_config.connect_url.as_deref(),
    )
    .await;

    // Phase 2: data flow checks (DLQ + consumer lag) -- only if Kafka is connected
    let kafka_connected = services.iter().any(|s| s.name == "Kafka" && s.status == "ok");
    if kafka_connected {
        if let Some(kc) = kafka_config {
            let kc = kc.clone();
            let nodes = topology.nodes.clone();
            let dlq_prefix = doctor_config.dlq_topic_prefix.clone();
            let lag_warning = 1000i64; // TODO: read from operations config
            let lag_critical = 10000i64;
            let dlq_threshold = 1i64;

            // Run blocking Kafka operations on a separate thread
            let (mut df_metrics, mut df_issues) = tokio::task::spawn_blocking(move || {
                let mut m = Vec::new();
                let mut i = Vec::new();
                check_data_flow(
                    &nodes, &kc, &dlq_prefix, lag_warning, lag_critical, dlq_threshold,
                    &mut m, &mut i,
                );
                (m, i)
            })
            .await
            .unwrap_or_default();

            metrics.append(&mut df_metrics);
            issues.append(&mut df_issues);
        }
    }

    let duration_ms = start.elapsed().as_millis() as u64;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    DiagnosticReport {
        topology: topology.clone(),
        metrics,
        issues,
        services,
        timestamp: format!("{now}"),
        duration_ms,
    }
}
