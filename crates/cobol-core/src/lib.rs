//! cobol-core: Foundation traits, configuration, and error types.
//!
//! This crate defines the canonical trait hierarchy (`CobolField`, `CobolNumeric`, etc.),
//! runtime configuration, and shared error types used by all other runtime crates.

pub mod category;
pub mod config;
pub mod decimal_ext;
pub mod ebcdic;
pub mod editing;
pub mod error;
pub mod numeric_parse;
pub mod traits;

// Re-export key types at crate root for convenience
#[doc(inline)]
pub use category::DataCategory;
#[doc(inline)]
pub use config::{
    ArithMode, CobolDialect, CollatingSequence, DiagnosticLevel, NumProc, RoundingMode,
    RuntimeConfig,
};
#[doc(inline)]
pub use ebcdic::CodePage;
#[doc(inline)]
pub use editing::EditSymbol;
#[doc(inline)]
pub use error::{ArithError, CallError, CobolError, DataError, FileError, SortError};
#[doc(inline)]
pub use traits::{CobolField, CobolGroup, CobolNumeric, CobolNumericEdited};
