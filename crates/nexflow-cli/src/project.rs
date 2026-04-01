// NexCore -- Nexflow CLI: Project Configuration
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.
//
// PROPRIETARY AND CONFIDENTIAL
// Unauthorized use, reproduction, or distribution is prohibited.

//! Handles nexflow.toml manifest parsing and project structure.
//! Matches Python CLI's project.py for compatibility.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// DSL file extensions and their language types.
pub const DSL_EXTENSIONS: &[(&str, &str)] = &[
    (".api", "api"),
    (".service", "service"),
    (".schema", "schema"),
    (".proc", "proc"),
    (".xform", "transform"),
    (".rules", "rules"),
    (".nxq", "nexquery"),
    (".infra", "infra"),
];

/// Nexflow project configuration (from nexflow.toml).
#[derive(Debug, Clone)]
pub struct NexflowProject {
    pub name: String,
    pub version: String,
    pub root_dir: PathBuf,
    pub src_dir: PathBuf,
    pub output_dir: PathBuf,
}

impl NexflowProject {
    /// Load project from nexflow.toml, searching upward from cwd.
    pub fn load() -> Result<Self, String> {
        let root = find_project_root()?;
        let config_path = root.join("nexflow.toml");
        let content = std::fs::read_to_string(&config_path)
            .map_err(|e| format!("Cannot read nexflow.toml: {e}"))?;
        Self::from_toml(&content, &root)
    }

    fn from_toml(content: &str, root_dir: &Path) -> Result<Self, String> {
        // Minimal TOML parsing for [project] and [paths] sections.
        // We parse just what we need without a full TOML crate.
        let name = extract_toml_string(content, "name")
            .unwrap_or_else(|| root_dir.file_name().unwrap_or_default().to_string_lossy().to_string());
        let version = extract_toml_string(content, "version").unwrap_or_else(|| "0.1.0".to_string());
        let src = extract_toml_string(content, "src").unwrap_or_else(|| "src".to_string());
        let output = extract_toml_string(content, "output").unwrap_or_else(|| "generated".to_string());

        Ok(Self {
            name,
            version,
            root_dir: root_dir.to_path_buf(),
            src_dir: root_dir.join(src),
            output_dir: root_dir.join(output),
        })
    }

    /// Get file counts by DSL type.
    pub fn file_counts(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for &(ext, lang) in DSL_EXTENSIONS {
            let count = if self.src_dir.exists() {
                walkdir_count(&self.src_dir, ext)
            } else {
                0
            };
            counts.insert(lang.to_string(), count);
        }
        counts
    }

    /// Get all source files for a given extension.
    pub fn source_files(&self, ext: &str) -> Vec<PathBuf> {
        if !self.src_dir.exists() {
            return Vec::new();
        }
        walkdir_files(&self.src_dir, ext)
    }

    /// Get all DSL source files.
    pub fn all_source_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        for &(ext, _) in DSL_EXTENSIONS {
            files.extend(self.source_files(ext));
        }
        files.sort();
        files
    }
}

/// Find project root by searching upward for nexflow.toml.
fn find_project_root() -> Result<PathBuf, String> {
    let mut current = std::env::current_dir()
        .map_err(|e| format!("Cannot get current directory: {e}"))?;

    loop {
        if current.join("nexflow.toml").exists() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(
                "No nexflow.toml found. Run 'nexflow init' to create a project.".to_string(),
            );
        }
    }
}

/// Count files with a given extension recursively.
fn walkdir_count(dir: &Path, ext: &str) -> usize {
    walkdir_files(dir, ext).len()
}

/// Find files with a given extension recursively.
fn walkdir_files(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir_files(&path, ext));
            } else if path.extension().and_then(|e| e.to_str()) == Some(ext.trim_start_matches('.')) {
                files.push(path);
            }
        }
    }
    files
}

/// Extract a simple string value from TOML content.
/// Handles: key = "value" (within any section).
fn extract_toml_string(content: &str, key: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(key) {
            let rest = rest.trim();
            if let Some(rest) = rest.strip_prefix('=') {
                let rest = rest.trim();
                if let Some(rest) = rest.strip_prefix('"') {
                    if let Some(end) = rest.find('"') {
                        return Some(rest[..end].to_string());
                    }
                }
            }
        }
    }
    None
}

/// Create default nexflow.toml content.
pub fn create_default_config(name: &str) -> String {
    let package = name.replace('-', "_");
    format!(
        r#"# Nexflow Project Configuration

[project]
name = "{name}"
version = "0.1.0"

[paths]
src = "src"
output = "generated"

# Source directories for each DSL type
[sources.proc]
path = "src/proc"

[sources.schema]
path = "src/schema"

[sources.transform]
path = "src/transform"

[sources.rules]
path = "src/rules"

# Code generation targets
[targets.flink]
enabled = true
output = "generated/flink"

[targets.flink.options]
package = "{package}.flink"
java_version = "17"

[targets.spark]
enabled = false
output = "generated/spark"

# Code generation settings
[codegen]
source_breadcrumbs = true
source_breadcrumbs_path = "filename"
source_coverage_report = true
"#,
    )
}

/// Determine DSL language from file extension.
pub fn language_from_extension(path: &Path) -> Option<&'static str> {
    let ext = path.extension()?.to_str()?;
    for &(dsl_ext, lang) in DSL_EXTENSIONS {
        if dsl_ext.trim_start_matches('.') == ext {
            return Some(lang);
        }
    }
    None
}
