//! Error types for the rustify crate.
//!
//! Backtrace: set `RUST_BACKTRACE=1` for stack traces via miette diagnostics.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during rustification.
///
/// # Backtrace
///
/// Set `RUST_BACKTRACE=1` to capture backtraces via miette diagnostics.
#[derive(Error, Debug)]
pub enum RustifyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("source directory not found: {0}")]
    SourceNotFound(PathBuf),

    #[error("output directory already exists with patches; use --force or --preserve-patches: {0}")]
    PatchesDetected(PathBuf),

    #[error("source and output directories must be different")]
    SameDirectory,

    #[error("syn parse error in {file}: {message}")]
    ParseError { file: PathBuf, message: String },

    #[error("hints.json not found (required for tier 2+): {0}")]
    HintsNotFound(PathBuf),

    #[error("target config error: {0}")]
    Config(String),
}
