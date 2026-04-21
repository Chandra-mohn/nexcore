pub mod connectivity;
pub mod diagnostics;
pub mod error;
pub mod topology;
pub mod types;

pub use diagnostics::run_diagnostics;
pub use error::DoctorError;
pub use topology::{build_fallback_topology, build_topology_from_proc_dir};
pub use types::{
    DiagnosticIssue, DiagnosticReport, DoctorConfig, IssueSeverity, NodeMetrics, NodeStatus,
    NodeType, PipelineEdge, PipelineNode, PipelineTopology, ServiceStatus,
};
