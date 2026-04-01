// NexCore -- Nexflow Parser: ServiceDSL AST
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Typed AST for `.service` files (ServiceDSL grammar).

use serde::{Deserialize, Serialize};

use super::common::{
    ConfigDirective, ImportPath, QualifiedRef, SchemaRef, ValueOrCfg,
};

/// Top-level service definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceDefinition {
    pub imports: Vec<ImportPath>,
    pub name: String,
    pub description: Option<String>,
    pub implements: Vec<String>,
    pub consumes: Vec<String>,
    pub config: Vec<ConfigDirective>,
    pub handlers: Vec<Handler>,
    pub cache_entries: Vec<CacheEntry>,
    pub health: Option<HealthBlock>,
    pub ready_path: Option<String>,
}

/// A request handler (per-endpoint implementation pipeline).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Handler {
    pub name: String,
    pub statements: Vec<HandlerStatement>,
}

/// A statement within a handler.
/// 8 primitives + 4 composition operators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandlerStatement {
    Authorize {
        scope: String,
    },
    Validate {
        expression: Expression,
        using: QualifiedRef,
    },
    Lookup {
        target: String,
        from: String,
        where_clauses: Vec<WhereClause>,
    },
    Transform {
        sources: Vec<Expression>,
        using: QualifiedRef,
        into: String,
    },
    Persist {
        target: Expression,
        assignment: Option<Expression>,
        to: String,
    },
    Call {
        service_endpoint: QualifiedRef,
        with_args: Vec<WithArg>,
        for_each: Option<ForEachClause>,
        into: Option<String>,
    },
    Publish {
        event_name: String,
    },
    Respond {
        status_code: i64,
        schema: Option<SchemaRef>,
    },
    OnError {
        cases: Vec<ErrorCase>,
    },
    Transaction {
        statements: Vec<HandlerStatement>,
        on_rollback: Vec<HandlerStatement>,
    },
    Saga {
        steps: Vec<SagaStep>,
    },
    Parallel {
        statements: Vec<HandlerStatement>,
    },
}

/// An expression (dotted path, literal, or cfg. reference).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Dotted path: `request.account_id`, `source.balance`
    Path(Vec<String>),
    /// Literal value: integer, string, boolean, null
    Literal(LiteralValue),
    /// Qualified annotation: `cfg.timeout`, `pii.ssn`
    QualifiedAnnotation { namespace: String, key: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LiteralValue {
    Integer(i64),
    Decimal(f64),
    String(String),
    Boolean(bool),
    Null,
}

/// A where clause in a lookup statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhereClause {
    pub left: Expression,
    pub comparator: Comparator,
    pub right: Expression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comparator {
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

/// A `with` argument in a call statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithArg {
    pub key: String,
    pub value: Expression,
}

/// A `for each` clause in a call statement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForEachClause {
    pub variable: String,
    pub collection: Expression,
}

/// An error case in an on_error block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorCase {
    /// `404 when account is null`
    WhenPredicate {
        status_code: i64,
        predicate: Predicate,
    },
    /// `fallback balance to default_balance`
    Fallback {
        expression: Expression,
        to: String,
    },
}

/// A predicate in an error case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Predicate {
    /// `account is null`
    IsNull(Expression),
    /// `source.balance < request.amount`
    Comparison {
        left: Expression,
        comparator: Comparator,
        right: Expression,
    },
    /// `product_details contains null`
    Contains {
        expression: Expression,
        value: String,
    },
}

/// A saga step with compensation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SagaStep {
    pub name: String,
    pub statements: Vec<HandlerStatement>,
    pub compensate: Vec<HandlerStatement>,
}

/// A cache entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheEntry {
    pub handler_name: String,
    pub ttl: ValueOrCfg,
    pub ttl_unit: String,
    pub invalidate_on: Vec<String>,
}

/// Health check block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthBlock {
    pub path: String,
    pub checks: Vec<HealthCheck>,
}

/// A health check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheck {
    pub kind: String,
    pub resource: String,
    pub timeout: ValueOrCfg,
    pub timeout_unit: String,
}
