//! Error types for the cobol-data crate.

use thiserror::Error;

/// Errors that can occur during data intelligence operations.
#[derive(Debug, Error, miette::Diagnostic)]
pub enum DataError {
    /// No dataset has been opened in the viewer session.
    #[error("no dataset open")]
    NoDataset,

    /// No schema (copybook) has been attached to the session.
    #[error("no schema attached")]
    NoSchema,

    /// Record index out of range.
    #[error("record index {index} out of range (total: {total})")]
    OutOfRange {
        /// Requested index.
        index: usize,
        /// Total number of records.
        total: usize,
    },

    /// Error decoding a field from binary data.
    #[error("decode error at field '{field}' offset {offset}: {reason}")]
    DecodeError {
        /// Field name.
        field: String,
        /// Byte offset in record.
        offset: usize,
        /// Description of the error.
        reason: String,
    },

    /// Invalid PIC clause.
    #[error("invalid PIC clause '{clause}': {reason}")]
    InvalidPic {
        /// The PIC string.
        clause: String,
        /// Why it is invalid.
        reason: String,
    },

    /// Parse error from the transpiler.
    #[error("parse error: {0}")]
    ParseError(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, DataError>;
