//! Read COBOL source files tolerantly (non-UTF-8 bytes replaced with spaces).
//!
//! Mainframe-transferred COBOL files often contain stray non-ASCII bytes
//! (e.g. EBCDIC sequence numbers in columns 1-6). COBOL is a pure ASCII
//! language, so replacing these bytes is safe and avoids UTF-8 read failures.

use std::fs;
use std::path::Path;

use miette::{Context, IntoDiagnostic, Result};

/// Read a COBOL source or copybook file, replacing non-UTF-8 bytes.
pub fn read_cobol_source(path: &Path) -> Result<String> {
    let bytes = fs::read(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read {}", path.display()))?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
