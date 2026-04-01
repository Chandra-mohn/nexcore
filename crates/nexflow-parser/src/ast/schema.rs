// NexCore -- Nexflow Parser: SchemaDSL AST
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Typed AST for `.schema` files (SchemaDSL v2 grammar).

use serde::{Deserialize, Serialize};

use super::common::ImportPath;

/// Top-level schema definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaDefinition {
    pub imports: Vec<ImportPath>,
    pub name: String,
    pub patterns: Vec<MutationPattern>,
    pub targets: Vec<String>,
    pub version: Option<VersionInfo>,
    pub compatibility: Option<CompatibilityMode>,
    pub retention: Option<Duration>,
    pub identity: Vec<FieldDecl>,
    pub streaming: Option<StreamingBlock>,
    pub serialization: Option<SerializationBlock>,
    pub fields: Vec<FieldDecl>,
    pub nested_objects: Vec<NestedObject>,
    pub computed: Vec<ComputedField>,
    pub constraints: Vec<ConstraintDecl>,
    pub immutable: Option<bool>,
    pub state_machine: Option<StateMachineBlock>,
    pub parameters: Vec<ParameterDecl>,
    pub entries: Vec<EntryDecl>,
    pub rules: Vec<RuleBlock>,
    pub migration: Vec<MigrationStatement>,
}

/// Type alias block (top-level, not inside a schema).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAliasBlock {
    pub aliases: Vec<TypeAlias>,
}

/// A single type alias.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub field_type: Option<FieldType>,
    pub fields: Vec<FieldDecl>,
}

/// Top-level program containing schemas and type alias blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaProgram {
    pub imports: Vec<ImportPath>,
    pub schemas: Vec<SchemaDefinition>,
    pub type_aliases: Vec<TypeAliasBlock>,
}

// -- Mutation patterns --

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MutationPattern {
    MasterData,
    ImmutableLedger,
    VersionedConfiguration,
    OperationalParameters,
    EventLog,
    StateMachine,
    TemporalData,
    ReferenceData,
    BusinessLogic,
    Command,
    Response,
    Aggregate,
    Document,
    AuditEvent,
}

// -- Version --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionInfo {
    pub number: String,
    pub compatibility: Option<CompatibilityMode>,
    pub previous_version: Option<String>,
    pub deprecation: Option<DeprecationInfo>,
    pub migration_guide: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityMode {
    Backward,
    Forward,
    Full,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeprecationInfo {
    pub message: String,
    pub since: Option<String>,
    pub removal_version: Option<String>,
}

// -- Duration --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Duration {
    pub value: i64,
    pub unit: String,
}

// -- Fields --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldDecl {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub optional: bool,
    pub unique: bool,
    pub cannot_change: bool,
    pub encrypted: bool,
    pub pii: Option<PiiModifier>,
    pub default: Option<String>,
    pub deprecated: Option<String>,
    pub removal: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    /// Base types: string, integer, decimal, uuid, etc.
    Base { name: String, params: Vec<i64> },
    /// Collection: list(T), set(T), map(K, V)
    Collection { kind: CollectionKind, element_types: Vec<FieldType> },
    /// Inline object: object { field1 type1, field2 type2 }
    InlineObject { fields: Vec<FieldDecl> },
    /// Custom type reference (lowercase identifier)
    Custom(String),
    /// Type alias reference (uppercase identifier)
    Alias(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectionKind {
    List,
    Set,
    Map,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PiiModifier {
    pub profile: Option<String>,
}

// -- Nested objects --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NestedObject {
    pub name: String,
    pub is_list: bool,
    pub fields: Vec<FieldDecl>,
    pub nested: Vec<NestedObject>,
}

// -- Computed fields --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputedField {
    pub name: String,
    /// Raw expression text (parsed to string for now; full expression AST later)
    pub expression: String,
}

// -- Constraints --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConstraintDecl {
    Enum {
        field: String,
        values: Vec<String>,
    },
    Range {
        field: String,
        min: String,
        max: String,
    },
    Pattern {
        field: String,
        regex: String,
    },
    Length {
        field: String,
        min: i64,
        max: Option<i64>,
    },
    BusinessRule {
        condition: String,
        message: String,
    },
}

// -- Streaming --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamingBlock {
    pub key_fields: Vec<String>,
    pub time_field: Option<String>,
    pub time_semantics: Option<TimeSemantics>,
    pub watermark_delay: Option<Duration>,
    pub watermark_strategy: Option<String>,
    pub max_out_of_orderness: Option<Duration>,
    pub watermark_interval: Option<Duration>,
    pub watermark_field: Option<String>,
    pub late_data_handling: Option<String>,
    pub late_data_stream: Option<String>,
    pub allowed_lateness: Option<Duration>,
    pub idle_timeout: Option<Duration>,
    pub idle_behavior: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeSemantics {
    EventTime,
    ProcessingTime,
    IngestionTime,
}

// -- Serialization --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SerializationBlock {
    pub format: Option<String>,
    pub compatibility: Option<CompatibilityMode>,
    pub subject: Option<String>,
    pub registry: Option<String>,
}

// -- State machine --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateMachineBlock {
    pub for_entity: Option<String>,
    pub states: Vec<StateDecl>,
    pub initial_state: Option<String>,
    pub transitions: Vec<TransitionDecl>,
    pub on_transition: Vec<TransitionAction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateDecl {
    pub name: String,
    pub qualifier: Option<StateQualifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateQualifier {
    Initial,
    Terminal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionDecl {
    /// Original: from state [target1, target2]
    From { state: String, targets: Vec<String> },
    /// Arrow: state -> state: trigger
    Arrow { from: String, to: String, trigger: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionAction {
    pub to_state: String,
    pub action_name: String,
    pub args: Vec<String>,
}

// -- Parameters (operational_parameters pattern) --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterDecl {
    pub name: String,
    pub field_type: FieldType,
    pub default: Option<String>,
    pub range: Option<(String, String)>,
    pub can_schedule: Option<bool>,
    pub change_frequency: Option<String>,
}

// -- Entries (reference_data pattern) --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryDecl {
    pub name: String,
    pub fields: Vec<EntryField>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryField {
    pub key: String,
    pub value: String,
}

// -- Rules (business_logic pattern) --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleBlock {
    pub name: String,
    pub given: Vec<RuleFieldDecl>,
    pub calculate: Vec<Calculation>,
    pub returns: Vec<RuleFieldDecl>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuleFieldDecl {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Calculation {
    pub name: String,
    /// Raw expression text
    pub expression: String,
}

// -- Migration --

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MigrationStatement {
    /// Raw text of the migration statement
    pub raw: String,
}
