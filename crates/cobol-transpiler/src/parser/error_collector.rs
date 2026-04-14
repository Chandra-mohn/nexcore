//! ANTLR error collector -- captures token recognition and parse errors.
//!
//! Replaces the default `ConsoleErrorListener` with a collector that stores
//! errors for later analysis instead of printing to stderr.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::recognizer::Recognizer;
use antlr_rust::token_factory::TokenFactory;

/// A single token or parse error captured from ANTLR.
#[derive(Debug, Clone)]
pub struct TokenError {
    /// Source line number (1-based).
    pub line: usize,
    /// Column number (0-based).
    pub column: usize,
    /// The error message from ANTLR.
    pub message: String,
    /// The offending text (e.g., the unrecognized character).
    pub offending_text: String,
}

/// Shared storage for errors collected across lexer and parser.
#[derive(Debug, Clone, Default)]
pub struct ErrorStore {
    errors: Arc<Mutex<Vec<TokenError>>>,
}

impl ErrorStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Drain all collected errors.
    pub fn drain(&self) -> Vec<TokenError> {
        std::mem::take(&mut *self.errors.lock().expect("ErrorStore mutex poisoned"))
    }

    /// Total number of collected errors.
    pub fn count(&self) -> usize {
        self.errors.lock().expect("ErrorStore mutex poisoned").len()
    }

    /// Summarize errors by offending text (character -> count).
    pub fn summarize(&self) -> BTreeMap<String, usize> {
        let errors = self.errors.lock().expect("ErrorStore mutex poisoned");
        let mut counts = BTreeMap::new();
        for err in errors.iter() {
            *counts.entry(err.offending_text.clone()).or_insert(0) += 1;
        }
        counts
    }

    fn push(&self, error: TokenError) {
        self.errors.lock().expect("ErrorStore mutex poisoned").push(error);
    }
}

/// Error listener that collects errors into a shared `ErrorStore`.
///
/// Create one `ErrorStore`, then create `ErrorCollector` instances from it
/// for both the lexer and parser. All errors flow into the same store.
#[derive(Debug)]
pub struct ErrorCollector {
    store: ErrorStore,
}

impl ErrorCollector {
    pub fn new(store: &ErrorStore) -> Self {
        Self {
            store: store.clone(),
        }
    }
}

/// Extract the offending text from an ANTLR error message.
///
/// Token recognition errors have the format: `token recognition error at: 'X'`
fn extract_offending_text(msg: &str) -> String {
    if let Some(start) = msg.find("'") {
        if let Some(end) = msg[start + 1..].find("'") {
            return msg[start + 1..start + 1 + end].to_string();
        }
    }
    String::new()
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for ErrorCollector {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        let offending_text = extract_offending_text(msg);
        self.store.push(TokenError {
            line: line.max(0) as usize,
            column: column.max(0) as usize,
            message: msg.to_string(),
            offending_text,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_store_collects_and_drains() {
        let store = ErrorStore::new();
        store.push(TokenError {
            line: 1,
            column: 0,
            message: "token recognition error at: '#'".to_string(),
            offending_text: "#".to_string(),
        });
        store.push(TokenError {
            line: 2,
            column: 5,
            message: "token recognition error at: '@'".to_string(),
            offending_text: "@".to_string(),
        });

        assert_eq!(store.count(), 2);
        let errors = store.drain();
        assert_eq!(errors.len(), 2);
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn summarize_groups_by_character() {
        let store = ErrorStore::new();
        for _ in 0..5 {
            store.push(TokenError {
                line: 1,
                column: 0,
                message: "token recognition error at: '#'".to_string(),
                offending_text: "#".to_string(),
            });
        }
        for _ in 0..3 {
            store.push(TokenError {
                line: 1,
                column: 0,
                message: "token recognition error at: '@'".to_string(),
                offending_text: "@".to_string(),
            });
        }

        let summary = store.summarize();
        assert_eq!(summary.get("#"), Some(&5));
        assert_eq!(summary.get("@"), Some(&3));
    }

    #[test]
    fn extract_offending_text_parses_message() {
        assert_eq!(
            extract_offending_text("token recognition error at: '#'"),
            "#"
        );
        assert_eq!(
            extract_offending_text("token recognition error at: '~'"),
            "~"
        );
        assert_eq!(extract_offending_text("some other error"), "");
    }
}
