//! Manifest tracking for rustify outputs.
//!
//! Each rustified workspace contains `rustify/manifest.json` recording
//! what rules were applied, file checksums, and provenance.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::RustifyError;

/// Top-level manifest structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Schema version.
    pub version: String,
    /// cobol2rust version that generated this.
    pub cobol2rust_version: String,
    /// Relative path to the source directory.
    pub source: String,
    /// Absolute path to the source directory.
    pub source_resolved: String,
    /// Tier that was applied.
    pub tier: u8,
    /// Timestamp of generation (ISO 8601).
    pub timestamp: String,
    /// Rule IDs that were applied.
    pub rules_applied: Vec<String>,
    /// Rule IDs that were skipped.
    pub rules_skipped: Vec<String>,
    /// Include filter (if any).
    pub include_filter: Option<String>,
    /// Exclude filter (if any).
    pub exclude_filter: Option<String>,
    /// Per-file metadata.
    pub files: HashMap<String, FileManifest>,
    /// Summary statistics.
    pub summary: ManifestSummary,
}

/// Per-file manifest entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileManifest {
    /// SHA-256 of the file before transforms.
    pub checksum_before: String,
    /// SHA-256 of the file after transforms.
    pub checksum_after: String,
    /// Number of transforms applied.
    pub transforms_applied: usize,
    /// Per-rule hit counts (e.g., "const-extract:8").
    pub rules_hit: Vec<String>,
}

/// Summary statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSummary {
    pub files_processed: usize,
    pub files_changed: usize,
    pub files_unchanged: usize,
    pub total_transforms: usize,
    pub by_rule: HashMap<String, usize>,
}

/// Compute SHA-256 hex digest of a file's contents.
pub fn compute_file_checksum(path: &Path) -> Result<String, RustifyError> {
    let content = std::fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    Ok(format!("sha256:{result:x}"))
}

/// Compute SHA-256 hex digest of a byte slice.
pub fn compute_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("sha256:{result:x}")
}

/// Read manifest from a directory's `rustify/manifest.json`.
pub fn read_manifest(dir: &Path) -> Result<Option<Manifest>, RustifyError> {
    let path = dir.join("rustify").join("manifest.json");
    if !path.exists() {
        return Ok(None);
    }
    let content = std::fs::read_to_string(&path)?;
    let manifest: Manifest = serde_json::from_str(&content)?;
    Ok(Some(manifest))
}

/// Write manifest to a directory's `rustify/manifest.json`.
pub fn write_manifest(dir: &Path, manifest: &Manifest) -> Result<(), RustifyError> {
    let rustify_dir = dir.join("rustify");
    std::fs::create_dir_all(&rustify_dir)?;
    let content = serde_json::to_string_pretty(manifest)?;
    std::fs::write(rustify_dir.join("manifest.json"), content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_deterministic() {
        let a = compute_checksum(b"hello world");
        let b = compute_checksum(b"hello world");
        assert_eq!(a, b);
        assert!(a.starts_with("sha256:"));
    }

    #[test]
    fn checksum_different_inputs() {
        let a = compute_checksum(b"hello");
        let b = compute_checksum(b"world");
        assert_ne!(a, b);
    }

    #[test]
    fn manifest_round_trip() {
        let manifest = Manifest {
            version: "1.0".to_string(),
            cobol2rust_version: "0.1.0".to_string(),
            source: "../raw-rust".to_string(),
            source_resolved: "/tmp/raw-rust".to_string(),
            tier: 1,
            timestamp: "2026-03-16T14:30:00Z".to_string(),
            rules_applied: vec!["const-extract".to_string()],
            rules_skipped: vec![],
            include_filter: None,
            exclude_filter: None,
            files: HashMap::new(),
            summary: ManifestSummary {
                files_processed: 5,
                files_changed: 3,
                files_unchanged: 2,
                total_transforms: 10,
                by_rule: HashMap::new(),
            },
        };

        let json = serde_json::to_string_pretty(&manifest).unwrap();
        let parsed: Manifest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.tier, 1);
        assert_eq!(parsed.summary.files_processed, 5);
    }

    #[test]
    fn read_missing_manifest_returns_none() {
        let dir = std::env::temp_dir().join("cobol2rust_test_no_manifest");
        let _ = std::fs::create_dir_all(&dir);
        let result = read_manifest(&dir).unwrap();
        assert!(result.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
