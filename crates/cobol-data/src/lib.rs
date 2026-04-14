//! COBOL binary data intelligence.
//!
//! Provides layout computation, binary field decoding, REDEFINES group
//! extraction, discriminator detection, file-to-copybook discovery,
//! and a stateful viewer session for windowed record decoding.
//!
//! This crate operates on cobol-transpiler's AST types (`DataEntry`,
//! `ProcedureDivision`, `FileDescription`) -- no separate parser.

pub mod decode;
pub mod discovery;
pub mod discriminator;
pub mod encoding;
pub mod error;
pub mod export;
pub mod layout;
pub mod record;
pub mod redefines;
pub mod session;

// Re-export key types for convenience.
#[doc(inline)]
pub use decode::DecodedValue;
#[doc(inline)]
pub use encoding::Encoding;
#[doc(inline)]
pub use error::{DataError, Result};
#[doc(inline)]
pub use export::ExportFormat;
#[doc(inline)]
pub use record::DecodedRecord;
#[doc(inline)]
pub use redefines::{Confidence, Discriminator, RedefinesGroup};
#[doc(inline)]
pub use session::{FileAccess, NativeFileAccess, ViewerSession};
