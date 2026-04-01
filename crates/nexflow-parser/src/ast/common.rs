// NexCore -- Nexflow Parser: Shared AST Types
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Types shared across ApiDSL and ServiceDSL ASTs.

use serde::{Deserialize, Serialize};

/// A reference to a value that is either a literal or a cfg. indirection.
/// Examples: `1000`, `cfg.rate_limit`, `cfg.timeout`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValueOrCfg {
    Integer(i64),
    Decimal(f64),
    String(String),
    /// Config indirection: namespace.key (e.g., cfg.rate_limit)
    CfgRef { namespace: String, key: String },
}

/// A qualified reference: `Identifier.Identifier`
/// Used for: `AuditAPI.logChange`, `AddressValidation.rules`, `FormatBalance.xform`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualifiedRef {
    pub qualifier: String,
    pub name: String,
}

/// A reference to a schema type (bare name or qualified).
/// Examples: `AccountDetail`, `AccountNotFound`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaRef {
    pub name: String,
    pub qualifier: Option<String>,
}

/// A generic key-value config directive.
/// Examples: `tracing_provider "opentelemetry"`, `pool_size 10`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigDirective {
    pub key: String,
    pub values: Vec<ConfigValue>,
}

/// A config value: integer, decimal, string, identifier, or boolean.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConfigValue {
    Integer(i64),
    Decimal(f64),
    String(String),
    Identifier(String),
    Boolean(bool),
}

/// An import path.
/// Example: `./schemas/account.schema`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportPath {
    pub raw: String,
}

/// HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}
