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

/// Error returned when parsing a Nexflow DSL source fails.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// ANTLR4 parser failed to produce a parse tree.
    #[error("parse error in {grammar}: {message}")]
    Grammar {
        /// Which DSL grammar was being parsed (e.g., "ApiDSL", "SchemaDSL").
        grammar: &'static str,
        /// Human-readable error description.
        message: String,
    },

    /// Parse tree was produced but AST construction failed (missing required node).
    #[error("{grammar}: {message}")]
    Ast {
        /// Which DSL grammar was being parsed.
        grammar: &'static str,
        /// What went wrong during AST construction.
        message: String,
    },
}

impl ParseError {
    /// Create a grammar-level parse error.
    ///
    /// # Errors
    ///
    /// This is an error constructor; it always returns `ParseError::Grammar`.
    pub fn grammar(grammar: &'static str, message: impl Into<String>) -> Self {
        Self::Grammar { grammar, message: message.into() }
    }

    /// Create an AST construction error.
    ///
    /// # Errors
    ///
    /// This is an error constructor; it always returns `ParseError::Ast`.
    pub fn ast(grammar: &'static str, message: impl Into<String>) -> Self {
        Self::Ast { grammar, message: message.into() }
    }
}
