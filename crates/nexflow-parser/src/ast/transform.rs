// NexCore -- Nexflow Parser: TransformDSL AST
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Typed AST for `.xform` files (TransformDSL grammar).

use serde::{Deserialize, Serialize};

use super::common::ImportPath;

/// Top-level program containing transforms and transform blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformProgram {
    pub imports: Vec<ImportPath>,
    pub transforms: Vec<TransformDef>,
    pub transform_blocks: Vec<TransformBlockDef>,
}

/// A field-level or expression-level transform definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformDef {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub previous_version: Option<String>,
    pub compatibility: Option<String>,
    pub pure: Option<bool>,
    pub idempotent: Option<bool>,
    pub cache: Option<CacheDecl>,
    pub inputs: Vec<FieldSpec>,
    pub outputs: Vec<FieldSpec>,
    pub lookup: Option<String>,
    pub lookups: Vec<LookupFieldDecl>,
    pub state: Option<String>,
    pub params: Vec<ParamDecl>,
    pub validate_input: Vec<ValidationRule>,
    pub apply: Vec<Statement>,
    pub validate_output: Vec<ValidationRule>,
    pub on_error: Option<ErrorBlock>,
}

/// A block-level transform (multi-field mapping).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformBlockDef {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub uses: Vec<String>,
    pub inputs: Vec<FieldSpec>,
    pub outputs: Vec<FieldSpec>,
    pub validate_input: Vec<ValidationRule>,
    pub invariants: Vec<ValidationRule>,
    pub mappings: Vec<Mapping>,
    pub compose: Option<ComposeBlock>,
    pub validate_output: Vec<ValidationRule>,
    pub on_change: Option<OnChangeBlock>,
    pub on_error: Option<ErrorBlock>,
}

/// An input or output field specification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSpec {
    pub name: Option<String>,
    pub field_type: TransformFieldType,
    pub nullable: bool,
    pub required: bool,
    pub default: Option<String>,
}

/// Type system for transform fields (mirrors grammar's fieldType rule).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransformFieldType {
    /// Base type: string, integer, decimal, boolean, date, timestamp, uuid, bytes
    Base { name: String, constraints: Vec<FieldConstraint> },
    /// Collection: list<T>, set<T>, map<K,V>
    Collection { kind: TransformCollectionKind, element_types: Vec<TransformFieldType> },
    /// Schema reference (lowercase identifier)
    Reference(String),
    /// Type alias reference (uppercase identifier)
    AliasRef(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransformCollectionKind {
    List,
    Set,
    Map,
}

/// Constraint on a field type (from grammar's constraintSpec rule).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldConstraint {
    Range { min: Option<String>, max: Option<String> },
    Length { min: Option<i64>, max: Option<i64> },
    Pattern(String),
    Values(Vec<String>),
    Precision { precision: i64, scale: Option<i64> },
}

/// A lookup field declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LookupFieldDecl {
    pub name: String,
    pub source: String,
}

/// A parameter declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamDecl {
    pub name: String,
    pub field_type: TransformFieldType,
    pub required: bool,
    pub optional: bool,
    pub default: Option<String>,
}

/// A validation rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Raw expression text.
    pub expression: String,
    /// Error message.
    pub message: Option<String>,
}

/// An assignment statement in apply block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    pub target: String,
    pub expression: String,
    pub is_let: bool,
}

/// A mapping in mappings block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mapping {
    pub target: String,
    pub expression: String,
}

/// Compose block for transform composition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComposeBlock {
    pub compose_type: Option<String>,
    pub refs: Vec<ComposeRef>,
    pub then: Option<ThenBlock>,
}

/// A reference in a compose block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComposeRef {
    Simple(String),
    Conditional { condition: String, transform: String },
    Otherwise(String),
}

/// A then block after compose.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThenBlock {
    pub compose_type: Option<String>,
    pub refs: Vec<ComposeRef>,
}

/// Cache declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheDecl {
    pub ttl: Option<String>,
    pub key: Vec<String>,
}

/// Error handling block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorBlock {
    pub action: Option<String>,
    pub default: Option<String>,
    pub log_level: Option<String>,
    pub emit_to: Option<String>,
    pub error_code: Option<String>,
}

/// On-change block for recalculation triggers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OnChangeBlock {
    pub watch_fields: Vec<String>,
    pub recalculate: Vec<Statement>,
}
