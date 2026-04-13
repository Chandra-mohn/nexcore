//! Target architecture configuration (`.cobol2rust-target.toml`).
//!
//! Captures architectural decisions that Tier 4 rules consume to perform
//! deterministic structural transformations. The config decouples creative
//! decisions (human/AI architect) from mechanical code transforms.

use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::error::RustifyError;

/// Default config file name.
pub const TARGET_CONFIG_FILE: &str = ".cobol2rust-target.toml";

/// Load a `.cobol2rust-target.toml` from the given directory, if it exists.
///
/// Returns the default config if the file does not exist.
pub fn load_target_config(dir: &Path) -> Result<TargetConfig, RustifyError> {
    let config_path = dir.join(TARGET_CONFIG_FILE);
    if !config_path.exists() {
        return Ok(TargetConfig::default());
    }

    let content = std::fs::read_to_string(&config_path)?;
    let config: TargetConfig = toml::from_str(&content)
        .map_err(|e| RustifyError::Config(format!("{}: {e}", config_path.display())))?;

    Ok(config)
}

/// Resolve the effective config for a specific program, merging per-program
/// overrides on top of the base config.
pub fn resolve_for_program(base: &TargetConfig, program_id: &str) -> TargetConfig {
    let mut resolved = base.clone();

    if let Some(overrides) = base.program.get(program_id) {
        if let Some(ref cf) = overrides.control_flow {
            merge_control_flow(&mut resolved.control_flow, cf);
        }
        if let Some(ref dm) = overrides.data_model {
            merge_data_model(&mut resolved.data_model, dm);
        }
        if let Some(ref eh) = overrides.error_handling {
            merge_error_handling(&mut resolved.error_handling, eh);
        }
        if let Some(ref io) = overrides.io {
            merge_io(&mut resolved.io, io);
        }
        if let Some(ref svc) = overrides.services {
            merge_services(&mut resolved.services, svc);
        }
        if let Some(ref a) = overrides.r#async {
            merge_async(&mut resolved.r#async, a);
        }
    }

    resolved
}

// ---------------------------------------------------------------------------
// Top-level config
// ---------------------------------------------------------------------------

/// Root target architecture config.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TargetConfig {
    /// Dispatcher elimination strategy.
    pub control_flow: ControlFlowConfig,
    /// WorkingStorage decomposition strategy.
    pub data_model: DataModelConfig,
    /// Error handling strategy.
    pub error_handling: ErrorHandlingConfig,
    /// File I/O modernization strategy.
    pub io: IoConfig,
    /// Service decomposition strategy.
    pub services: ServicesConfig,
    /// Concurrency model.
    pub r#async: AsyncConfig,
    /// Test generation settings.
    pub testing: TestingConfig,
    /// Target dependency preferences.
    pub dependencies: DependenciesConfig,
    /// Per-program overrides.
    #[serde(default)]
    pub program: HashMap<String, ProgramOverride>,
}


// ---------------------------------------------------------------------------
// [control_flow]
// ---------------------------------------------------------------------------

/// Dispatcher elimination strategy.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ControlFlowConfig {
    /// How to handle the dispatch loop.
    pub strategy: ControlFlowStrategy,
    /// Max body lines for inline eligibility.
    pub inline_threshold: usize,
    /// Collapse single-call chains (A -> B -> C becomes A_B_C).
    pub collapse_chains: bool,
    /// How to handle detected cycles.
    pub cycle_handling: CycleHandling,
    /// Generate structured error propagation from paragraphs.
    pub error_propagation: ErrorPropagation,
}

impl Default for ControlFlowConfig {
    fn default() -> Self {
        Self {
            strategy: ControlFlowStrategy::Inline,
            inline_threshold: 30,
            collapse_chains: true,
            cycle_handling: CycleHandling::Preserve,
            error_propagation: ErrorPropagation::None,
        }
    }
}

/// Dispatch loop transformation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ControlFlowStrategy {
    /// Inline leaf paragraphs at call sites.
    Inline,
    /// Extract each paragraph as a standalone function.
    Functions,
    /// Keep dispatch but use a typed enum state machine.
    StateMachine,
    /// Convert to async functions with structured concurrency.
    Async,
}

/// Cycle handling strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CycleHandling {
    /// Keep cycles as-is (loop).
    Preserve,
    /// Convert to trampoline pattern.
    Trampoline,
    /// Allow direct recursion (if depth is bounded).
    Recursive,
}

/// Error propagation strategy for paragraph functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorPropagation {
    /// No change to paragraph signatures.
    None,
    /// Paragraphs return Result<(), ProgramError>.
    Result,
    /// Use anyhow::Result.
    Anyhow,
}

// ---------------------------------------------------------------------------
// [data_model]
// ---------------------------------------------------------------------------

/// WorkingStorage decomposition strategy.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DataModelConfig {
    /// How to restructure WorkingStorage.
    pub strategy: DataModelStrategy,
    /// Minimum fields in a prefix group to trigger extraction.
    pub min_group_size: usize,
    /// Generate accessor methods vs pub fields.
    pub use_accessors: bool,
    /// Derive level for generated structs.
    pub derive_level: DeriveLevel,
    /// Field naming strategy.
    pub naming: NamingStrategy,
    /// Custom sub-struct definitions (override auto-detection).
    #[serde(default)]
    pub structs: HashMap<String, Vec<String>>,
}

impl Default for DataModelConfig {
    fn default() -> Self {
        Self {
            strategy: DataModelStrategy::Flat,
            min_group_size: 3,
            use_accessors: false,
            derive_level: DeriveLevel::Standard,
            naming: NamingStrategy::Preserve,
            structs: HashMap::new(),
        }
    }
}

/// WorkingStorage restructuring strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataModelStrategy {
    /// Keep flat struct, just rename fields.
    Flat,
    /// Extract sub-structs based on prefix groups + co-access.
    Nested,
    /// Split into multiple independent structs.
    Split,
    /// Full domain type extraction (most aggressive).
    Domain,
}

/// Derive trait level for generated structs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeriveLevel {
    /// Debug only.
    Minimal,
    /// Debug, Clone.
    Standard,
    /// Debug, Clone, Serialize, Deserialize.
    Full,
}

/// Field naming transformation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NamingStrategy {
    /// Keep COBOL names (ws_acct_number).
    Preserve,
    /// Remove ws_ prefix (acct_number).
    StripPrefix,
    /// Full Rust naming (account_number) -- requires mapping.
    RustIdiom,
}

// ---------------------------------------------------------------------------
// [error_handling]
// ---------------------------------------------------------------------------

/// Error handling strategy.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ErrorHandlingConfig {
    /// Overall error handling approach.
    pub strategy: ErrorStrategy,
    /// How to handle 2-value status fields.
    pub binary_status: BinaryStatusTarget,
    /// How to handle 3+ value status fields.
    pub multi_status: MultiStatusTarget,
    /// Generate a top-level program error enum.
    pub generate_program_error: bool,
    /// Name pattern for generated error types.
    pub error_type_pattern: String,
    /// Custom status-to-variant mappings.
    #[serde(default)]
    pub mappings: HashMap<String, StatusMapping>,
}

impl Default for ErrorHandlingConfig {
    fn default() -> Self {
        Self {
            strategy: ErrorStrategy::StatusQuo,
            binary_status: BinaryStatusTarget::Result,
            multi_status: MultiStatusTarget::Enum,
            generate_program_error: true,
            error_type_pattern: "{Program}Error".to_string(),
            mappings: HashMap::new(),
        }
    }
}

/// Overall error strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorStrategy {
    /// Keep numeric status fields as-is.
    StatusQuo,
    /// Convert to Result<T, E> with generated types.
    Result,
    /// Use thiserror-derived error enums.
    Thiserror,
    /// Use anyhow for all errors.
    Anyhow,
}

/// Target for binary (2-value) status fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BinaryStatusTarget {
    Bool,
    Result,
    Option,
}

/// Target for multi-value (3+) status fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MultiStatusTarget {
    Enum,
    Result,
    I32,
}

/// Custom mapping from status values to variant names.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusMapping {
    /// Value -> variant name.
    pub values: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// [io]
// ---------------------------------------------------------------------------

/// I/O modernization strategy.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct IoConfig {
    /// Default backend for all file handles.
    pub backend: IoBackend,
    /// Database for "database" backend.
    pub database: DatabaseTarget,
    /// Message broker for "stream" backend.
    pub broker: BrokerTarget,
    /// Per-handle backend overrides.
    #[serde(default)]
    pub handles: HashMap<String, HandleConfig>,
    /// File format settings.
    #[serde(default)]
    pub formats: FormatConfig,
}

impl Default for IoConfig {
    fn default() -> Self {
        Self {
            backend: IoBackend::File,
            database: DatabaseTarget::Sqlite,
            broker: BrokerTarget::Kafka,
            handles: HashMap::new(),
            formats: FormatConfig::default(),
        }
    }
}

/// I/O backend target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IoBackend {
    File,
    Database,
    Api,
    Stream,
}

/// Database target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DatabaseTarget {
    Sqlite,
    Postgres,
    Duckdb,
}

/// Message broker target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BrokerTarget {
    Kafka,
    Nats,
    Rabbitmq,
}

/// Per-handle configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct HandleConfig {
    /// Backend for this handle.
    pub backend: Option<IoBackend>,
    /// Format for file backend.
    pub format: Option<FileFormat>,
    /// Table name for database backend.
    pub table: Option<String>,
    /// Topic name for stream backend.
    pub topic: Option<String>,
    /// API endpoint for API backend.
    pub endpoint: Option<String>,
}

/// File format settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct FormatConfig {
    /// Default file format.
    pub default: FileFormat,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            default: FileFormat::Fixed,
        }
    }
}

/// File record format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FileFormat {
    Fixed,
    Csv,
    Json,
    Parquet,
}

// ---------------------------------------------------------------------------
// [services]
// ---------------------------------------------------------------------------

/// Service decomposition strategy.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ServicesConfig {
    /// Decomposition granularity.
    pub granularity: ServiceGranularity,
    /// How to define service boundaries.
    pub boundary: ServiceBoundary,
    /// Communication pattern between services.
    pub ipc: IpcStrategy,
    /// Web/RPC framework.
    pub framework: FrameworkTarget,
    /// Custom service boundary definitions.
    #[serde(default)]
    pub boundaries: Vec<ServiceBoundaryDef>,
}

impl Default for ServicesConfig {
    fn default() -> Self {
        Self {
            granularity: ServiceGranularity::Monolith,
            boundary: ServiceBoundary::Program,
            ipc: IpcStrategy::Direct,
            framework: FrameworkTarget::Axum,
            boundaries: Vec::new(),
        }
    }
}

/// Service granularity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceGranularity {
    Monolith,
    Modules,
    Libraries,
    Services,
}

/// Service boundary detection strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ServiceBoundary {
    Program,
    Section,
    ParagraphGroup,
    Custom,
}

/// Inter-process communication strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IpcStrategy {
    Direct,
    Grpc,
    Rest,
    Events,
}

/// Web/RPC framework target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FrameworkTarget {
    Axum,
    Actix,
    Tonic,
}

/// Custom service boundary definition.
#[derive(Debug, Clone, Deserialize)]
pub struct ServiceBoundaryDef {
    /// Service name.
    pub name: String,
    /// Paragraph patterns (glob-style).
    #[serde(default)]
    pub paragraphs: Vec<String>,
    /// I/O handles owned by this service.
    #[serde(default)]
    pub io_handles: Vec<String>,
}

// ---------------------------------------------------------------------------
// [async]
// ---------------------------------------------------------------------------

/// Concurrency model configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AsyncConfig {
    /// Async runtime.
    pub runtime: AsyncRuntime,
    /// Which operations to make async.
    pub scope: AsyncScope,
    /// Structured concurrency pattern.
    pub concurrency: ConcurrencyPattern,
}

impl Default for AsyncConfig {
    fn default() -> Self {
        Self {
            runtime: AsyncRuntime::Sync,
            scope: AsyncScope::IoOnly,
            concurrency: ConcurrencyPattern::None,
        }
    }
}

/// Async runtime target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AsyncRuntime {
    Sync,
    Tokio,
    AsyncStd,
}

/// Scope of async transformation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AsyncScope {
    IoOnly,
    All,
    Selective,
}

/// Structured concurrency pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConcurrencyPattern {
    None,
    Join,
    Select,
}

// ---------------------------------------------------------------------------
// [testing]
// ---------------------------------------------------------------------------

/// Test generation settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct TestingConfig {
    /// What tests to generate.
    pub generate: TestGenerate,
    /// Test framework.
    pub framework: TestFramework,
    /// Generate property-based tests for numeric transforms.
    pub property_tests: bool,
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
            generate: TestGenerate::Unit,
            framework: TestFramework::Builtin,
            property_tests: false,
        }
    }
}

/// Test generation level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TestGenerate {
    None,
    Unit,
    Integration,
    Both,
}

/// Test framework.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TestFramework {
    Builtin,
    Rstest,
}

// ---------------------------------------------------------------------------
// [dependencies]
// ---------------------------------------------------------------------------

/// Target dependency preferences.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DependenciesConfig {
    pub serde: bool,
    pub serde_json: bool,
    pub sqlx: bool,
    pub rusqlite: bool,
    pub tokio: bool,
    pub thiserror: bool,
    pub anyhow: bool,
    pub tracing: bool,
    pub log: bool,
}

impl Default for DependenciesConfig {
    fn default() -> Self {
        Self {
            serde: false,
            serde_json: false,
            sqlx: false,
            rusqlite: false,
            tokio: false,
            thiserror: true,
            anyhow: false,
            tracing: false,
            log: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Per-program overrides
// ---------------------------------------------------------------------------

/// Per-program config overrides. All fields are optional -- only set fields
/// override the base config.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ProgramOverride {
    pub control_flow: Option<ControlFlowConfig>,
    pub data_model: Option<DataModelConfig>,
    pub error_handling: Option<ErrorHandlingConfig>,
    pub io: Option<IoConfig>,
    pub services: Option<ServicesConfig>,
    pub r#async: Option<AsyncConfig>,
}

// ---------------------------------------------------------------------------
// Merge helpers (override only if the override section is present)
// ---------------------------------------------------------------------------

fn merge_control_flow(base: &mut ControlFlowConfig, over: &ControlFlowConfig) {
    *base = over.clone();
}

fn merge_data_model(base: &mut DataModelConfig, over: &DataModelConfig) {
    *base = over.clone();
}

fn merge_error_handling(base: &mut ErrorHandlingConfig, over: &ErrorHandlingConfig) {
    *base = over.clone();
}

fn merge_io(base: &mut IoConfig, over: &IoConfig) {
    // Merge handles additively -- override's handles take precedence.
    let mut merged = base.clone();
    merged.backend = over.backend;
    merged.database = over.database;
    merged.broker = over.broker;
    merged.formats = over.formats.clone();
    for (k, v) in &over.handles {
        merged.handles.insert(k.clone(), v.clone());
    }
    *base = merged;
}

fn merge_services(base: &mut ServicesConfig, over: &ServicesConfig) {
    *base = over.clone();
}

fn merge_async(base: &mut AsyncConfig, over: &AsyncConfig) {
    *base = over.clone();
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Validation warning (non-fatal).
#[derive(Debug, Clone)]
pub struct ConfigWarning {
    pub section: String,
    pub message: String,
}

/// Validate config consistency. Returns warnings, not errors.
pub fn validate_config(config: &TargetConfig) -> Vec<ConfigWarning> {
    let mut warnings = Vec::new();

    // Async runtime requires tokio dependency.
    if config.r#async.runtime == AsyncRuntime::Tokio && !config.dependencies.tokio {
        warnings.push(ConfigWarning {
            section: "async + dependencies".to_string(),
            message: "async.runtime = \"tokio\" but dependencies.tokio = false".to_string(),
        });
    }

    // gRPC framework requires tonic.
    if config.services.ipc == IpcStrategy::Grpc
        && config.services.framework != FrameworkTarget::Tonic
    {
        warnings.push(ConfigWarning {
            section: "services".to_string(),
            message: "services.ipc = \"grpc\" usually pairs with framework = \"tonic\""
                .to_string(),
        });
    }

    // Thiserror strategy requires thiserror dependency.
    if config.error_handling.strategy == ErrorStrategy::Thiserror && !config.dependencies.thiserror {
        warnings.push(ConfigWarning {
            section: "error_handling + dependencies".to_string(),
            message:
                "error_handling.strategy = \"thiserror\" but dependencies.thiserror = false"
                    .to_string(),
        });
    }

    // Anyhow strategy requires anyhow dependency.
    if config.error_handling.strategy == ErrorStrategy::Anyhow && !config.dependencies.anyhow {
        warnings.push(ConfigWarning {
            section: "error_handling + dependencies".to_string(),
            message: "error_handling.strategy = \"anyhow\" but dependencies.anyhow = false"
                .to_string(),
        });
    }

    // Database backend requires a database dependency.
    if config.io.backend == IoBackend::Database
        && !config.dependencies.sqlx
        && !config.dependencies.rusqlite
    {
        warnings.push(ConfigWarning {
            section: "io + dependencies".to_string(),
            message: "io.backend = \"database\" but no database dependency enabled".to_string(),
        });
    }

    // Service decomposition with async scope=all but sync runtime.
    if config.services.granularity == ServiceGranularity::Services
        && config.r#async.runtime == AsyncRuntime::Sync
        && config.services.ipc != IpcStrategy::Direct
    {
        warnings.push(ConfigWarning {
            section: "services + async".to_string(),
            message: "services with network IPC typically requires an async runtime".to_string(),
        });
    }

    // Full derive level requires serde dependency.
    if config.data_model.derive_level == DeriveLevel::Full && !config.dependencies.serde {
        warnings.push(ConfigWarning {
            section: "data_model + dependencies".to_string(),
            message:
                "data_model.derive_level = \"full\" requires Serialize/Deserialize but dependencies.serde = false"
                    .to_string(),
        });
    }

    warnings
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_parses() {
        let config: TargetConfig = toml::from_str("").unwrap();
        assert_eq!(config.control_flow.strategy, ControlFlowStrategy::Inline);
        assert_eq!(config.data_model.strategy, DataModelStrategy::Flat);
        assert_eq!(config.error_handling.strategy, ErrorStrategy::StatusQuo);
        assert_eq!(config.io.backend, IoBackend::File);
        assert_eq!(
            config.services.granularity,
            ServiceGranularity::Monolith
        );
        assert_eq!(config.r#async.runtime, AsyncRuntime::Sync);
    }

    #[test]
    fn cloud_native_preset() {
        let toml_str = r#"
[control_flow]
strategy = "functions"
error_propagation = "result"

[data_model]
strategy = "nested"
naming = "strip-prefix"
derive_level = "full"

[error_handling]
strategy = "thiserror"

[io]
backend = "database"
database = "postgres"

[services]
granularity = "services"
ipc = "grpc"
framework = "tonic"

[async]
runtime = "tokio"
scope = "all"

[testing]
generate = "both"

[dependencies]
serde = true
tokio = true
thiserror = true
sqlx = true
tracing = true
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.control_flow.strategy, ControlFlowStrategy::Functions);
        assert_eq!(
            config.control_flow.error_propagation,
            ErrorPropagation::Result
        );
        assert_eq!(config.data_model.strategy, DataModelStrategy::Nested);
        assert_eq!(config.data_model.naming, NamingStrategy::StripPrefix);
        assert_eq!(config.error_handling.strategy, ErrorStrategy::Thiserror);
        assert_eq!(config.io.backend, IoBackend::Database);
        assert_eq!(config.io.database, DatabaseTarget::Postgres);
        assert_eq!(
            config.services.granularity,
            ServiceGranularity::Services
        );
        assert_eq!(config.services.ipc, IpcStrategy::Grpc);
        assert_eq!(config.r#async.runtime, AsyncRuntime::Tokio);
        assert_eq!(config.testing.generate, TestGenerate::Both);
        assert!(config.dependencies.serde);
        assert!(config.dependencies.tokio);
    }

    #[test]
    fn lift_and_shift_preset() {
        let toml_str = r#"
[control_flow]
strategy = "inline"
inline_threshold = 20

[data_model]
strategy = "flat"
naming = "strip-prefix"

[error_handling]
strategy = "result"
binary_status = "bool"
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.control_flow.strategy, ControlFlowStrategy::Inline);
        assert_eq!(config.control_flow.inline_threshold, 20);
        assert_eq!(config.data_model.strategy, DataModelStrategy::Flat);
        assert_eq!(config.data_model.naming, NamingStrategy::StripPrefix);
        assert_eq!(config.error_handling.strategy, ErrorStrategy::Result);
        assert_eq!(
            config.error_handling.binary_status,
            BinaryStatusTarget::Bool
        );
    }

    #[test]
    fn per_handle_io_config() {
        let toml_str = r#"
[io]
backend = "file"

[io.handles.master_file]
backend = "database"
table = "master_records"

[io.handles.output_file]
backend = "stream"
topic = "processed-records"
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.io.backend, IoBackend::File);

        let master = &config.io.handles["master_file"];
        assert_eq!(master.backend, Some(IoBackend::Database));
        assert_eq!(master.table.as_deref(), Some("master_records"));

        let output = &config.io.handles["output_file"];
        assert_eq!(output.backend, Some(IoBackend::Stream));
        assert_eq!(output.topic.as_deref(), Some("processed-records"));
    }

    #[test]
    fn custom_struct_overrides() {
        let toml_str = r#"
[data_model]
strategy = "nested"

[data_model.structs]
AccountInfo = ["ws_acct_number", "ws_acct_type", "ws_acct_balance"]
TransactionData = ["ws_txn_date", "ws_txn_amount", "ws_txn_code"]
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.data_model.structs.len(), 2);
        assert_eq!(config.data_model.structs["AccountInfo"].len(), 3);
        assert_eq!(config.data_model.structs["TransactionData"].len(), 3);
    }

    #[test]
    fn custom_error_mappings() {
        let toml_str = r#"
[error_handling]
strategy = "result"

[error_handling.mappings.ws_status]
values = { "0" = "Success", "1" = "InvalidInput", "2" = "NotFound" }
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        let mapping = &config.error_handling.mappings["ws_status"];
        assert_eq!(mapping.values["0"], "Success");
        assert_eq!(mapping.values["1"], "InvalidInput");
        assert_eq!(mapping.values["2"], "NotFound");
    }

    #[test]
    fn service_boundaries() {
        let toml_str = r#"
[services]
granularity = "services"
boundary = "custom"

[[services.boundaries]]
name = "account-service"
paragraphs = ["process_account_*", "validate_account_*"]
io_handles = ["master_file"]

[[services.boundaries]]
name = "report-service"
paragraphs = ["generate_report_*", "print_*"]
io_handles = ["report_file"]
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.services.boundaries.len(), 2);
        assert_eq!(config.services.boundaries[0].name, "account-service");
        assert_eq!(config.services.boundaries[0].paragraphs.len(), 2);
        assert_eq!(config.services.boundaries[1].name, "report-service");
    }

    #[test]
    fn per_program_overrides() {
        let toml_str = r#"
[control_flow]
strategy = "inline"

[data_model]
strategy = "flat"

[program.ACCT0100.control_flow]
strategy = "functions"

[program.ACCT0100.data_model]
strategy = "domain"

[program.ACCT0100.io.handles.master_file]
backend = "database"
table = "accounts"
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();

        // Base config
        assert_eq!(config.control_flow.strategy, ControlFlowStrategy::Inline);
        assert_eq!(config.data_model.strategy, DataModelStrategy::Flat);

        // Resolved for ACCT0100
        let resolved = resolve_for_program(&config, "ACCT0100");
        assert_eq!(
            resolved.control_flow.strategy,
            ControlFlowStrategy::Functions
        );
        assert_eq!(resolved.data_model.strategy, DataModelStrategy::Domain);
        assert_eq!(
            resolved.io.handles["master_file"].backend,
            Some(IoBackend::Database)
        );

        // Unaffected program gets base config
        let other = resolve_for_program(&config, "OTHER");
        assert_eq!(other.control_flow.strategy, ControlFlowStrategy::Inline);
        assert_eq!(other.data_model.strategy, DataModelStrategy::Flat);
    }

    #[test]
    fn validation_catches_inconsistencies() {
        let toml_str = r#"
[async]
runtime = "tokio"

[error_handling]
strategy = "thiserror"

[io]
backend = "database"

[data_model]
derive_level = "full"

[dependencies]
tokio = false
thiserror = false
serde = false
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        let warnings = validate_config(&config);

        // Should catch: tokio dep missing, thiserror dep missing,
        // database dep missing, serde dep missing
        assert!(warnings.len() >= 4, "got {} warnings", warnings.len());

        let messages: Vec<&str> = warnings.iter().map(|w| w.message.as_str()).collect();
        assert!(messages.iter().any(|m| m.contains("tokio")));
        assert!(messages.iter().any(|m| m.contains("thiserror")));
        assert!(messages.iter().any(|m| m.contains("database")));
        assert!(messages.iter().any(|m| m.contains("serde")));
    }

    #[test]
    fn validation_passes_for_consistent_config() {
        let config = TargetConfig::default();
        let warnings = validate_config(&config);
        assert!(warnings.is_empty(), "default config should have no warnings");
    }

    #[test]
    fn state_machine_strategy() {
        let toml_str = r#"
[control_flow]
strategy = "state-machine"
cycle_handling = "trampoline"
"#;
        let config: TargetConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(
            config.control_flow.strategy,
            ControlFlowStrategy::StateMachine
        );
        assert_eq!(
            config.control_flow.cycle_handling,
            CycleHandling::Trampoline
        );
    }
}
