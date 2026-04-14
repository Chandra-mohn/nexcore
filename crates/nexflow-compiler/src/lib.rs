// NexCore -- Nexflow Compiler
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

//! Cross-grammar validation and linking for Nexflow DSLs.
//!
//! The compiler validates consistency across `.api`, `.service`, and `.schema`
//! files. It checks:
//!
//! - **Schema references**: API endpoint request/response/error types resolve to
//!   existing schema definitions
//! - **Service contracts**: Service `implements` references match an API name,
//!   handler names match API endpoint names
//! - **Type consistency**: Field types in schemas are valid base types or
//!   references to other schemas
//! - **Completeness**: Every API endpoint has a corresponding service handler

pub mod diagnostics;
pub mod resolve;
pub mod validate;

pub use diagnostics::{Diagnostic, DiagnosticLevel, ValidationResult};
pub use nexflow_parser::ParseError;
pub use resolve::{load_files, load_project, Project};
pub use validate::validate;
