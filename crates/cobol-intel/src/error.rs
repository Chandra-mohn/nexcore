use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
pub enum IntelError {
    #[error("graph build failed: {reason}")]
    BuildFailed { reason: String },

    #[error("node not found: {kind} '{name}'")]
    NodeNotFound { kind: String, name: String },

    #[error("storage error: {reason}")]
    Storage { reason: String },

    #[error("encryption error: {reason}")]
    Encryption { reason: String },

    #[error("decryption failed -- invalid license key or corrupted file")]
    DecryptionFailed,

    #[error("invalid .nxg file: {reason}")]
    InvalidFormat { reason: String },

    #[error("parse error: {reason}")]
    ParseError { reason: String },

    #[error("query execution error: {reason}")]
    QueryError { reason: String },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type IntelResult<T> = Result<T, IntelError>;
