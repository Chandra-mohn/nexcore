//! Shared utilities for the cobol2rust CLI.
//!
//! Consolidates common patterns used across multiple subcommands:
//! thread pool setup, progress bars, JSON output, and file discovery.

use indicatif::{ProgressBar, ProgressStyle};
use miette::{IntoDiagnostic, Result};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Thread pool
// ---------------------------------------------------------------------------

/// Configure the global rayon thread pool with the given number of threads.
///
/// Safe to call multiple times -- silently ignores errors if already initialized.
pub fn setup_thread_pool(num_threads: usize) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .ok();
}

// ---------------------------------------------------------------------------
// Progress bars
// ---------------------------------------------------------------------------

/// Create a styled progress bar with a prefix label.
///
/// Standard style: `<prefix> [========>       ] 42/100 (12.3/s) ETA: 5s`
pub fn make_progress_bar(total: u64, prefix: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(&format!(
            "{prefix} [{{bar:40}}] {{pos}}/{{len}} ({{per_sec}}) ETA: {{eta}}"
        ))
        .unwrap_or_else(|_| ProgressStyle::default_bar())
        .progress_chars("=> "),
    );
    pb
}

// ---------------------------------------------------------------------------
// JSON output
// ---------------------------------------------------------------------------

/// Serialize a value as pretty JSON and print to stdout.
pub fn print_json<T: Serialize>(value: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(value)
        .into_diagnostic()
        .map_err(|e| miette::miette!("JSON serialization failed: {e}"))?;
    println!("{json}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_bar_creates() {
        let pb = make_progress_bar(100, "  Test");
        assert_eq!(pb.length(), Some(100));
    }

    #[test]
    fn print_json_simple() {
        // Just verify it doesn't panic on a simple value.
        let value = serde_json::json!({"key": "value"});
        // Can't easily capture stdout in a test, but verify no error.
        assert!(print_json(&value).is_ok());
    }
}
