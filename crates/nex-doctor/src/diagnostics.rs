use crate::connectivity::check_connectivity;
use crate::types::{DiagnosticReport, DoctorConfig, PipelineTopology};
use nex_kafka::KafkaConfig;
use nex_registry::RegistryConfig;

/// Run a full diagnostic pass: connectivity checks + rule evaluation.
///
/// Phase 1: connectivity only. Future phases add data flow and Flink deep inspection.
pub async fn run_diagnostics(
    topology: &PipelineTopology,
    kafka_config: Option<&KafkaConfig>,
    registry_config: Option<&RegistryConfig>,
    doctor_config: &DoctorConfig,
) -> DiagnosticReport {
    let start = std::time::Instant::now();

    let (metrics, issues, services) = check_connectivity(
        &topology.nodes,
        kafka_config,
        registry_config,
        doctor_config.flink_url.as_deref(),
        doctor_config.connect_url.as_deref(),
    )
    .await;

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
