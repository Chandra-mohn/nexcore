//! Error types for JCL parsing.

use miette::Diagnostic;
use thiserror::Error;

/// JCL parsing errors.
#[derive(Debug, Error, Diagnostic)]
pub enum JclError {
    /// Missing JOB card in execution JCL.
    #[error("no JOB card found in JCL source")]
    #[diagnostic(code(jcl::missing_job))]
    MissingJob,

    /// Missing EXEC statement in a step.
    #[error("line {line}: step has no EXEC statement")]
    #[diagnostic(code(jcl::missing_exec))]
    MissingExec { line: usize },

    /// Invalid DISP= value.
    #[error("line {line}: invalid DISP value: {value}")]
    #[diagnostic(code(jcl::invalid_disp))]
    InvalidDisp { line: usize, value: String },

    /// Invalid COND= parameter.
    #[error("line {line}: invalid COND parameter: {value}")]
    #[diagnostic(code(jcl::invalid_cond))]
    InvalidCond { line: usize, value: String },

    /// Unexpected token or syntax.
    #[error("line {line}: {message}")]
    #[diagnostic(code(jcl::syntax))]
    Syntax { line: usize, message: String },

    /// I/O error reading JCL file.
    #[error("cannot read JCL file: {0}")]
    #[diagnostic(code(jcl::io))]
    Io(#[from] std::io::Error),
}

/// Convenience Result type.
pub type Result<T> = std::result::Result<T, JclError>;
