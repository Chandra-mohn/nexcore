// NexCore -- Nexflow Compiler: Diagnostics
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Diagnostic types for validation errors and warnings.

use std::fmt;

/// Severity level of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    /// Informational message (not an error).
    Info,
    /// Potential issue that might cause problems.
    Warning,
    /// Definite error that must be fixed.
    Error,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "info"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

/// A single diagnostic message from validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Severity level.
    pub level: DiagnosticLevel,
    /// Which grammar/file type the diagnostic relates to.
    pub source: DiagnosticSource,
    /// Human-readable message.
    pub message: String,
    /// Optional context (e.g., schema name, endpoint name).
    pub context: Option<String>,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctx = self
            .context
            .as_ref()
            .map(|c| format!(" [{c}]"))
            .unwrap_or_default();
        write!(f, "[{}] {}{}: {}", self.level, self.source, ctx, self.message)
    }
}

/// Which DSL grammar the diagnostic originates from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSource {
    Api,
    Service,
    Schema,
    CrossGrammar,
}

impl fmt::Display for DiagnosticSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::Service => write!(f, "service"),
            Self::Schema => write!(f, "schema"),
            Self::CrossGrammar => write!(f, "cross"),
        }
    }
}

/// Result of validation: a list of diagnostics.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub diagnostics: Vec<Diagnostic>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn error(
        &mut self,
        source: DiagnosticSource,
        message: impl Into<String>,
        context: Option<String>,
    ) {
        self.push(Diagnostic {
            level: DiagnosticLevel::Error,
            source,
            message: message.into(),
            context,
        });
    }

    pub fn warning(
        &mut self,
        source: DiagnosticSource,
        message: impl Into<String>,
        context: Option<String>,
    ) {
        self.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            source,
            message: message.into(),
            context,
        });
    }

    pub fn info(
        &mut self,
        source: DiagnosticSource,
        message: impl Into<String>,
        context: Option<String>,
    ) {
        self.push(Diagnostic {
            level: DiagnosticLevel::Info,
            source,
            message: message.into(),
            context,
        });
    }

    /// Returns true if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    /// Count of errors.
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .count()
    }

    /// Count of warnings.
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Warning)
            .count()
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}
