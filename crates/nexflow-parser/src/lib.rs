// NexCore -- Nexflow Parser
// Copyright (c) 2024-2026 Mphasis Corporation. All Rights Reserved.
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

//! Nexflow DSL parsers and typed ASTs.
//!
//! This crate provides ANTLR4-generated parsers and strongly-typed AST
//! representations for all 7 Nexflow DSL grammars:
//!
//! - **SchemaDSL** (.schema) -- Data types, structures, constraints
//! - **RulesDSL** (.rules) -- Business decisions, validation, decision tables
//! - **TransformDSL** (.xform) -- Data transformations, field mappings
//! - **ProcDSL** (.proc) -- Stream/batch pipeline orchestration
//! - **ApiDSL** (.api) -- Service contracts, endpoints, events
//! - **ServiceDSL** (.service) -- Microservice request/response orchestration
//! - **ScreenDSL** (.screen) -- UI contracts (future)

// Modules will be added as grammars are implemented:
// pub mod api;
// pub mod service;
// pub mod schema;
// pub mod rules;
// pub mod transform;
// pub mod proc;
