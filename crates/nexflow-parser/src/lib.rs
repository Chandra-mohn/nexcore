// NexCore -- Nexflow Parser
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

//! Nexflow DSL parsers and typed ASTs.
//!
//! This crate provides ANTLR4-generated parsers and strongly-typed AST
//! representations for Nexflow DSL grammars. Currently implemented:
//!
//! - **ApiDSL** (.api) -- Service contracts, endpoints, events
//! - **ServiceDSL** (.service) -- Microservice request/response orchestration
//! - **SchemaDSL** (.schema) -- Schema registry, data types, mutation patterns
//! - **TransformDSL** (.xform) -- Data transformations, field mappings
//! - **RulesDSL** (.rules) -- Decision tables, procedural business rules
//! - **ProcDSL** (.proc) -- Process orchestration, streaming pipelines

pub mod ast;
pub mod generated;
pub mod parse;

// Re-export the main parse functions and error type for convenience.
pub use parse::api_builder::parse_api;
pub use parse::nexquery_builder::parse_nexquery;
pub use parse::proc_builder::parse_proc;
pub use parse::rules_builder::parse_rules;
pub use parse::schema_builder::parse_schema;
pub use parse::service_builder::parse_service;
pub use parse::transform_builder::parse_transform;
pub use parse::ParseError;
