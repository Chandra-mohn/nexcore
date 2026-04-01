// NexCore -- Nexflow Parser: NexQueryDSL AST
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Typed AST for `.nxq` files (NexQueryDSL grammar).
//!
//! This AST is compatible with the hand-written NexQuery AST in cobol-intel.
//! It can serve as a drop-in replacement when the ANTLR4 parser is adopted.

use serde::{Deserialize, Serialize};

/// A script is one or more statements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NexQueryScript {
    pub statements: Vec<NexQueryStatement>,
}

/// A statement is a pipeline of clauses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NexQueryStatement {
    pub clauses: Vec<NexQueryClause>,
}

/// A clause is traverse, expand, or verb.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NexQueryClause {
    Traverse(TraverseClause),
    Expand(ExpandClause),
    Verb(VerbClause),
}

/// `<node-type> <traversal-verb> <target> <filter?>`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraverseClause {
    pub node_type: NodeType,
    pub verb: TraversalVerb,
    pub target: Target,
    pub filter: Option<Filter>,
}

/// `<node-type> <filter?>`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExpandClause {
    pub node_type: NodeType,
    pub filter: Option<Filter>,
}

/// `<domain-verb> <target?> <modifier*>`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerbClause {
    pub verb: DomainVerb,
    pub target: Option<Target>,
    pub modifiers: Vec<Modifier>,
}

/// The 7 node types in the code graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Programs,
    Paragraphs,
    Fields,
    Copybooks,
    Files,
    Tables,
    Rules,
}

/// The 14 traversal verbs (7 forward + 7 reverse).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraversalVerb {
    Calling,
    CalledBy,
    Performing,
    PerformedBy,
    Reading,
    ReadBy,
    Writing,
    WrittenBy,
    Using,
    UsedBy,
    Accessing,
    AccessedBy,
    Containing,
    Within,
}

/// The 11 domain verbs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainVerb {
    Trace,
    Rank,
    Similar,
    FindDead,
    Deps,
    Impact,
    Compare,
    DiscoverProcesses,
    EstimateCost,
    Save,
    Run,
}

/// A query target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Target {
    Identifier(String),
    Each,
    List(Vec<String>),
}

/// A filter (parenthesized predicate expression).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    pub expr: FilterExpr,
}

/// A filter expression (predicate tree).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FilterExpr {
    Predicate(Predicate),
    And(Box<FilterExpr>, Box<FilterExpr>),
    Or(Box<FilterExpr>, Box<FilterExpr>),
    Not(Box<FilterExpr>),
}

/// A single predicate: field op value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Predicate {
    pub field: String,
    pub op: CompareOp,
    pub value: QueryValue,
}

/// Comparison operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompareOp {
    Eq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
    Glob,
    Regex,
    In,
    Has,
}

/// A value in a predicate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryValue {
    String(String),
    Number(f64),
    List(Vec<String>),
}

/// A modifier (keyword argument for domain verbs).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Modifier {
    pub keyword: ModifierKeyword,
    pub value: ModifierValue,
}

/// Modifier keywords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifierKeyword {
    By,
    Top,
    In,
    With,
    Depth,
    Level,
    Order,
    Scope,
    Threshold,
    As,
}

/// Modifier values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModifierValue {
    Identifier(String),
    Number(f64),
    NodeType(NodeType),
    String(String),
}
