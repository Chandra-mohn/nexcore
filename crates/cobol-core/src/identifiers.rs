//! Strongly-typed identifiers for COBOL entities.
//!
//! These newtypes prevent accidental mixing of program names, field names,
//! and copybook names in function signatures. Per Microsoft Pragmatic Rust
//! Guidelines C-NEWTYPE: avoid primitive obsession by using strong types.
//!
//! All identifiers store their canonical COBOL form (uppercase, hyphens preserved).
//! Use `.to_rust_name()` to get the snake_case Rust equivalent.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A COBOL program name (from PROGRAM-ID).
///
/// Stored in uppercase with hyphens (e.g., "ACCT-PROCESS").
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProgramName(String);

/// A COBOL data field name (from DATA DIVISION).
///
/// Stored in uppercase with hyphens (e.g., "WS-ACCT-NUMBER").
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FieldName(String);

/// A COBOL copybook name (from COPY statement).
///
/// Stored in uppercase, no extension (e.g., "ACCT-MASTER").
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CopybookName(String);

/// A COBOL paragraph or section name (from PROCEDURE DIVISION).
///
/// Stored in uppercase with hyphens (e.g., "2200-CALCULATE-PAY").
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ParagraphName(String);

// -- Shared behavior for all identifier types --

macro_rules! impl_identifier {
    ($ty:ident) => {
        impl $ty {
            /// Create a new identifier from a string, converting to uppercase.
            pub fn new(name: impl Into<String>) -> Self {
                Self(name.into().to_uppercase())
            }

            /// Create from an already-uppercase string (no conversion).
            pub fn from_upper(name: String) -> Self {
                Self(name)
            }

            /// The canonical COBOL name (uppercase with hyphens).
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Convert to snake_case Rust identifier.
            pub fn to_rust_name(&self) -> String {
                self.0.to_lowercase().replace('-', "_")
            }

            /// Consume and return the inner string.
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl AsRef<str> for $ty {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $ty {
            fn from(s: &str) -> Self {
                Self::new(s)
            }
        }

        impl From<String> for $ty {
            fn from(s: String) -> Self {
                Self::new(s)
            }
        }
    };
}

impl_identifier!(ProgramName);
impl_identifier!(FieldName);
impl_identifier!(CopybookName);
impl_identifier!(ParagraphName);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_name_uppercase() {
        let name = ProgramName::new("acct-process");
        assert_eq!(name.as_str(), "ACCT-PROCESS");
    }

    #[test]
    fn field_name_to_rust() {
        let name = FieldName::new("WS-ACCT-NUMBER");
        assert_eq!(name.to_rust_name(), "ws_acct_number");
    }

    #[test]
    fn copybook_from_str() {
        let name: CopybookName = "acct-master".into();
        assert_eq!(name.as_str(), "ACCT-MASTER");
    }

    #[test]
    fn paragraph_display() {
        let name = ParagraphName::new("2200-CALCULATE-PAY");
        assert_eq!(format!("{name}"), "2200-CALCULATE-PAY");
    }

    #[test]
    fn equality_and_ordering() {
        let a = ProgramName::new("ABC");
        let b = ProgramName::new("abc");
        assert_eq!(a, b);
    }
}
