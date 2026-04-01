// NexCore -- Nexflow Parser: Parse Module
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Parse functions that convert source text into typed ASTs.
//!
//! Each parse function:
//! 1. Lexes + parses source text using ANTLR4-generated parser
//! 2. Walks the parse tree using context accessors (compositional)
//! 3. Returns a typed AST node
//!
//! Pattern: each build_* function takes a parse tree context and returns
//! a typed AST node. This is the visitor pattern implemented manually
//! because antlr-rust visitors return () rather than typed values.

pub mod api_builder;
pub mod nexquery_builder;
pub mod proc_builder;
pub mod rules_builder;
pub mod schema_builder;
pub mod service_builder;
pub mod transform_builder;

mod helpers;
