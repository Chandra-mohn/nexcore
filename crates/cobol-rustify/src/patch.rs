//! Patch detection for rustify outputs.
//!
//! Compares current file checksums against the manifest to detect
//! user modifications in the output directory.

use std::path::Path;

use crate::error::RustifyError;
use crate::manifest::{compute_file_checksum, read_manifest};

/// Result of patch detection scan.
#[derive(Debug)]
pub struct PatchDetection {
    /// Files whose checksum still matches `checksum_after` (no user edits).
    pub clean: Vec<String>,
    /// Files whose checksum differs from `checksum_after` (user edited).
    pub modified: Vec<String>,
    /// Files listed in manifest but missing from disk.
    pub missing: Vec<String>,
}

impl PatchDetection {
    /// True if any files have been modified or are missing.
    pub fn has_patches(&self) -> bool {
        !self.modified.is_empty() || !self.missing.is_empty()
    }
}

/// Detect user patches in an output directory by comparing against its manifest.
///
/// Returns `None` if no manifest exists (fresh directory).
pub fn detect_patches(output_dir: &Path) -> Result<Option<PatchDetection>, RustifyError> {
    let Some(manifest) = read_manifest(output_dir)? else {
        return Ok(None);
    };

    let mut clean = Vec::new();
    let mut modified = Vec::new();
    let mut missing = Vec::new();

    for (rel_path, file_manifest) in &manifest.files {
        let abs_path = output_dir.join(rel_path);
        if !abs_path.exists() {
            missing.push(rel_path.clone());
            continue;
        }

        let current_checksum = compute_file_checksum(&abs_path)?;
        if current_checksum == file_manifest.checksum_after {
            clean.push(rel_path.clone());
        } else {
            modified.push(rel_path.clone());
        }
    }

    Ok(Some(PatchDetection {
        clean,
        modified,
        missing,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{
        compute_checksum, write_manifest, FileManifest, Manifest, ManifestSummary,
    };
    use std::collections::HashMap;

    fn test_dir(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("cobol2rust_test_patch_{name}"))
    }

    fn cleanup(dir: &Path) {
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn no_manifest_returns_none() {
        let dir = test_dir("no_manifest");
        cleanup(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let result = detect_patches(&dir).unwrap();
        assert!(result.is_none());
        cleanup(&dir);
    }

    #[test]
    fn clean_files_detected() {
        let dir = test_dir("clean");
        cleanup(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();

        // Write a file
        let content = b"fn main() {}";
        std::fs::write(dir.join("src/test.rs"), content).unwrap();
        let checksum = compute_checksum(content);

        // Write manifest with matching checksum
        let mut files = HashMap::new();
        files.insert(
            "src/test.rs".to_string(),
            FileManifest {
                checksum_before: checksum.clone(),
                checksum_after: checksum,
                transforms_applied: 0,
                rules_hit: vec![],
            },
        );

        let manifest = Manifest {
            version: "1.0".to_string(),
            cobol2rust_version: "0.1.0".to_string(),
            source: "../raw-rust".to_string(),
            source_resolved: "/tmp/raw-rust".to_string(),
            tier: 1,
            timestamp: "2026-03-16T00:00:00Z".to_string(),
            rules_applied: vec![],
            rules_skipped: vec![],
            include_filter: None,
            exclude_filter: None,
            files,
            summary: ManifestSummary {
                files_processed: 1,
                files_changed: 0,
                files_unchanged: 1,
                total_transforms: 0,
                by_rule: HashMap::new(),
            },
        };
        write_manifest(&dir, &manifest).unwrap();

        let detection = detect_patches(&dir).unwrap().unwrap();
        assert_eq!(detection.clean.len(), 1);
        assert!(detection.modified.is_empty());
        assert!(detection.missing.is_empty());
        assert!(!detection.has_patches());

        cleanup(&dir);
    }

    #[test]
    fn modified_file_detected() {
        let dir = test_dir("modified");
        cleanup(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();

        // Write a file with different content than manifest expects
        std::fs::write(dir.join("src/test.rs"), b"fn main() { modified(); }").unwrap();
        let original_checksum = compute_checksum(b"fn main() {}");

        let mut files = HashMap::new();
        files.insert(
            "src/test.rs".to_string(),
            FileManifest {
                checksum_before: original_checksum.clone(),
                checksum_after: original_checksum,
                transforms_applied: 0,
                rules_hit: vec![],
            },
        );

        let manifest = Manifest {
            version: "1.0".to_string(),
            cobol2rust_version: "0.1.0".to_string(),
            source: "../raw-rust".to_string(),
            source_resolved: "/tmp/raw-rust".to_string(),
            tier: 1,
            timestamp: "2026-03-16T00:00:00Z".to_string(),
            rules_applied: vec![],
            rules_skipped: vec![],
            include_filter: None,
            exclude_filter: None,
            files,
            summary: ManifestSummary {
                files_processed: 1,
                files_changed: 0,
                files_unchanged: 1,
                total_transforms: 0,
                by_rule: HashMap::new(),
            },
        };
        write_manifest(&dir, &manifest).unwrap();

        let detection = detect_patches(&dir).unwrap().unwrap();
        assert!(detection.clean.is_empty());
        assert_eq!(detection.modified.len(), 1);
        assert!(detection.has_patches());

        cleanup(&dir);
    }

    #[test]
    fn missing_file_detected() {
        let dir = test_dir("missing");
        cleanup(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let mut files = HashMap::new();
        files.insert(
            "src/gone.rs".to_string(),
            FileManifest {
                checksum_before: "sha256:abc".to_string(),
                checksum_after: "sha256:abc".to_string(),
                transforms_applied: 0,
                rules_hit: vec![],
            },
        );

        let manifest = Manifest {
            version: "1.0".to_string(),
            cobol2rust_version: "0.1.0".to_string(),
            source: "../raw-rust".to_string(),
            source_resolved: "/tmp/raw-rust".to_string(),
            tier: 1,
            timestamp: "2026-03-16T00:00:00Z".to_string(),
            rules_applied: vec![],
            rules_skipped: vec![],
            include_filter: None,
            exclude_filter: None,
            files,
            summary: ManifestSummary {
                files_processed: 1,
                files_changed: 0,
                files_unchanged: 1,
                total_transforms: 0,
                by_rule: HashMap::new(),
            },
        };
        write_manifest(&dir, &manifest).unwrap();

        let detection = detect_patches(&dir).unwrap().unwrap();
        assert!(detection.clean.is_empty());
        assert!(detection.modified.is_empty());
        assert_eq!(detection.missing.len(), 1);
        assert!(detection.has_patches());

        cleanup(&dir);
    }
}
