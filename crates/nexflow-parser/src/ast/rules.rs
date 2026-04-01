// NexCore -- Nexflow Parser: RulesDSL AST
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Typed AST for `.rules` files (RulesDSL grammar).

use serde::{Deserialize, Serialize};

use super::common::ImportPath;

/// Top-level rules program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulesProgram {
    pub imports: Vec<ImportPath>,
    pub services: Vec<ServiceDecl>,
    pub actions: Vec<ActionDecl>,
    pub decision_tables: Vec<DecisionTableDef>,
    pub procedural_rules: Vec<ProceduralRuleDef>,
}

// -- Service declarations --

/// External service declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDecl {
    pub name: String,
    pub service_type: ServiceType,
    pub class_name: String,
    pub method_name: String,
    pub params: Vec<ServiceParam>,
    pub return_type: String,
    pub timeout: Option<String>,
    pub fallback: Option<String>,
    pub retry: Option<i64>,
}

/// Service invocation type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServiceType {
    Sync,
    Async,
    Cached(String),
}

/// Service parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceParam {
    pub name: String,
    pub param_type: String,
}

// -- Action declarations --

/// Action method declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionDecl {
    pub name: String,
    pub params: Vec<ServiceParam>,
    pub target: ActionTarget,
}

/// Action target (where the action routes to).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionTarget {
    /// emit to stream_name
    Emit { stream: String },
    /// state state_name.operation(arg)
    State {
        state_name: String,
        operation: String,
        arg: Option<String>,
    },
    /// audit
    Audit,
    /// call ClassName.method
    Call { class: String, method: String },
}

// -- Decision tables --

/// A decision table definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionTableDef {
    pub name: String,
    pub hit_policy: Option<HitPolicy>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub inputs: Vec<InputParam>,
    pub columns: Vec<String>,
    pub rows: Vec<TableRow>,
    pub returns: Vec<ReturnParam>,
    pub execute: Option<String>,
    pub post_calculate: Vec<PostCalculateStmt>,
    pub aggregate: Vec<AggregateStmt>,
}

/// Hit policy for decision tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HitPolicy {
    FirstMatch,
    SingleHit,
    MultiHit,
    CollectAll,
}

/// An input parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputParam {
    pub name: String,
    pub param_type: String,
}

/// A table row with typed cells.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableRow {
    pub priority: Option<i64>,
    pub cells: Vec<CellContent>,
}

/// Typed cell content (condition or action).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellContent {
    /// * (wildcard -- any value / no action)
    Wildcard,
    /// A condition expression (for input columns)
    Condition(ConditionExpr),
    /// An action expression (for output columns)
    Action(ActionExpr),
}

/// Condition types in decision table cells.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConditionExpr {
    /// Exact match: literal value
    ExactMatch(String),
    /// Range: 700 to 799
    Range { from: String, to: String },
    /// In set: in (a, b, c) or not in (x, y)
    InSet { values: Vec<String>, negated: bool },
    /// Pattern: matches "regex", starts_with "x", ends_with "y", contains "z"
    Pattern { kind: String, pattern: String },
    /// Null check: is null, is not null
    NullCheck { is_null: bool },
    /// Comparison: >= 700, != "closed", < 0.3
    Comparison { operator: String, value: String },
    /// Complex boolean expression in parens
    Expression(String),
    /// Marker state: marker eod_1 fired
    MarkerState { marker: String, state: String },
}

/// Action types in decision table cells.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionExpr {
    /// Literal assignment
    Assign(String),
    /// Arithmetic/expression calculation
    Calculate(String),
    /// lookup(table, key, default: value)
    Lookup {
        table: String,
        args: Vec<String>,
        default: Option<String>,
    },
    /// function(args)
    Call { function: String, args: Vec<String> },
    /// emit to stream
    Emit { target: String },
}

/// A return parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnParam {
    pub name: String,
    pub param_type: String,
}

/// Post-calculate statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCalculateStmt {
    pub name: String,
    pub expression: String,
    pub is_let: bool,
}

/// Aggregate statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateStmt {
    pub name: String,
    pub expression: String,
}

// -- Procedural rules --

/// A procedural rule definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProceduralRuleDef {
    pub name: String,
    pub description: Option<String>,
    pub body: Vec<BlockItem>,
}

/// A block item in a procedural rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockItem {
    /// if-then-elseif-else chain
    IfThenElse {
        condition: String,
        then_block: Vec<BlockItem>,
        elseif_blocks: Vec<(String, Vec<BlockItem>)>,
        else_block: Vec<BlockItem>,
    },
    /// set variable = expression
    Set { name: String, expression: String },
    /// let variable = expression
    Let { name: String, expression: String },
    /// action_name(args)
    ActionCall {
        name: String,
        args: Vec<String>,
    },
    /// return statement
    Return,
}
