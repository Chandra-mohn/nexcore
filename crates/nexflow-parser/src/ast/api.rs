// NexCore -- Nexflow Parser: ApiDSL AST
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Typed AST for `.api` files (ApiDSL grammar).

use serde::{Deserialize, Serialize};

use super::common::{
    ConfigDirective, HttpMethod, ImportPath, QualifiedRef, SchemaRef, ValueOrCfg,
};

/// Top-level API definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDefinition {
    pub imports: Vec<ImportPath>,
    pub name: String,
    pub version: Option<String>,
    pub base_path: Option<String>,
    pub description: Option<String>,
    pub targets: Vec<String>,
    pub auth_default: Option<AuthScheme>,
    pub content_type: Option<String>,
    pub rate_limit: Option<RateLimit>,
    pub pagination: Option<Pagination>,
    pub config: Vec<ConfigDirective>,
    pub cors: Option<CorsConfig>,
    pub endpoints: Vec<Endpoint>,
    pub events: Vec<EventContract>,
    pub dependencies: Vec<Dependency>,
    pub health_path: Option<String>,
    pub ready_path: Option<String>,
}

/// An auth scheme declaration.
/// Grammar accepts any identifier as scheme name; compiler validates.
/// Examples: `bearer scope "accounts:read"`, `api_key header "X-API-Key"`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthScheme {
    pub scheme: String,
    pub scope: Option<String>,
    pub header: Option<String>,
}

/// Rate limit declaration.
/// Examples: `rate_limit 1000 per minute burst 50`
///           `rate_limit cfg.rate per minute burst cfg.burst`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimit {
    pub limit: ValueOrCfg,
    pub per: String,
    pub burst: Option<ValueOrCfg>,
}

/// Pagination declaration.
/// Examples: `pagination cursor default_size 50 max_size 200`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pagination {
    pub style: String,
    pub default_size: i64,
    pub max_size: i64,
}

/// CORS configuration block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorsConfig {
    pub origins: Vec<ValueOrCfg>,
    pub methods: Vec<HttpMethod>,
    pub headers: Vec<String>,
    pub max_age: Option<ValueOrCfg>,
}

/// An endpoint declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    pub name: String,
    pub method: Option<HttpMethod>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub params: Vec<ParamDecl>,
    pub query: Vec<ParamDecl>,
    pub headers: Vec<ParamDecl>,
    pub request: Option<SchemaRef>,
    pub response: Option<ResponseDecl>,
    pub errors: Vec<ErrorMapping>,
    pub auth: Option<AuthScheme>,
    pub validate: Option<QualifiedRef>,
    pub rate_limit: Option<RateLimit>,
    pub timeout: Option<TimeoutDecl>,
    pub cache: Option<CacheDecl>,
    pub idempotent_key: Option<String>,
    pub is_async: bool,
    pub is_deprecated: bool,
    pub sunset: Option<String>,
    pub replacement: Option<String>,
}

/// Parameter declaration (used in params, query, headers blocks).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamDecl {
    pub name: String,
    pub type_ref: String,
    pub required: bool,
    pub optional: bool,
    pub default: Option<String>,
}

/// Response declaration with optional modifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseDecl {
    pub modifier: Option<ResponseModifier>,
    pub schema: SchemaRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseModifier {
    Paginated,
    List,
}

/// Error code -> schema mapping.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorMapping {
    pub status_code: i64,
    pub schema: SchemaRef,
}

/// Timeout declaration: value + unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeoutDecl {
    pub value: ValueOrCfg,
    pub unit: String,
}

/// Cache declaration: duration + unit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheDecl {
    pub duration: ValueOrCfg,
    pub unit: String,
}

/// Event contract declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventContract {
    pub name: String,
    pub topic: Option<String>,
    pub payload: Option<SchemaRef>,
    pub partitioned_by: Option<String>,
}

/// Dependency on another API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dependency {
    pub api_name: String,
    pub version: String,
}
