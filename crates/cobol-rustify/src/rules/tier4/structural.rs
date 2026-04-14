//! Structural rule trait and types for Tier 4 workspace-level transforms.

use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;

use crate::hints::HintsFile;
use crate::rules::transform::Transform;
use crate::target_config::TargetConfig;

/// A source file loaded into memory for structural analysis.
#[derive(Clone)]
pub struct SourceFile {
    /// Relative path within the workspace (e.g., "src/main.rs").
    pub rel_path: String,
    /// Absolute path on disk.
    pub abs_path: PathBuf,
    /// File content as text.
    pub text: String,
    /// Parsed syn AST (None if parse failed).
    pub parsed: Option<syn::File>,
}

impl std::fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceFile")
            .field("rel_path", &self.rel_path)
            .field("text_len", &self.text.len())
            .finish()
    }
}

/// Context provided to structural rules -- workspace-level.
#[allow(missing_debug_implementations)]
pub struct StructuralContext<'a> {
    /// All source files keyed by relative path.
    pub files: &'a HashMap<String, SourceFile>,
    /// COBOL semantic hints.
    pub hints: Option<&'a HintsFile>,
    /// Target architecture config.
    pub target: &'a TargetConfig,
    /// Tier 3 assessment transforms (from current or previous run).
    pub assessments: &'a [Transform],
}

/// A planned structural transformation (workspace-level).
#[derive(Debug, Clone)]
pub struct StructuralPlan {
    /// Rule that produced this plan.
    pub rule_id: String,
    /// Human-readable description of what this plan does.
    pub description: String,
    /// Files to modify: relative path -> new content.
    pub modified_files: HashMap<String, String>,
    /// New files to create: relative path -> content.
    pub new_files: HashMap<String, String>,
    /// Files to delete (relative paths).
    pub deleted_files: Vec<String>,
    /// Cargo.toml dependency additions.
    pub cargo_edits: Vec<CargoEdit>,
    /// Detailed summary for human review.
    pub summary: String,
}

impl StructuralPlan {
    /// Create an empty (no-op) plan.
    pub fn empty(rule_id: &str) -> Self {
        Self {
            rule_id: rule_id.to_string(),
            description: "No changes needed".to_string(),
            modified_files: HashMap::new(),
            new_files: HashMap::new(),
            deleted_files: Vec::new(),
            cargo_edits: Vec::new(),
            summary: String::new(),
        }
    }

    /// Whether this plan makes any changes.
    pub fn has_changes(&self) -> bool {
        !self.modified_files.is_empty()
            || !self.new_files.is_empty()
            || !self.deleted_files.is_empty()
            || !self.cargo_edits.is_empty()
    }

    /// Total number of file operations.
    pub fn file_op_count(&self) -> usize {
        self.modified_files.len() + self.new_files.len() + self.deleted_files.len()
    }
}

/// A dependency to add to Cargo.toml.
#[derive(Debug, Clone)]
pub struct CargoEdit {
    /// Crate name (e.g., "thiserror").
    pub dependency: String,
    /// Version requirement (e.g., "1.0").
    pub version: String,
    /// Optional features to enable.
    pub features: Vec<String>,
}

/// Trait for workspace-level structural rules (Tier 4).
///
/// Unlike `RustifyRule` (per-file), structural rules see the entire workspace
/// and produce a `StructuralPlan` that may create, modify, or delete files.
pub trait StructuralRule: Send + Sync {
    /// Unique rule identifier.
    fn id(&self) -> &'static str;

    /// Human-readable description.
    fn description(&self) -> &'static str;

    /// Analyze the workspace and produce a structural transformation plan.
    fn plan(&self, ctx: &StructuralContext) -> StructuralPlan;
}

/// Apply a structural plan to a mutable workspace file map.
///
/// Modifies `files` in place: updates content for modified files, adds new
/// files, removes deleted files. Returns the number of file operations.
#[allow(clippy::implicit_hasher)]
pub fn apply_plan(
    files: &mut HashMap<String, SourceFile>,
    plan: &StructuralPlan,
) -> usize {
    let mut ops = 0;

    // Apply modifications
    for (path, new_content) in &plan.modified_files {
        if let Some(file) = files.get_mut(path) {
            file.text.clone_from(new_content);
            // Re-parse the modified content
            file.parsed = match syn::parse_file(new_content) {
                Ok(f) => Some(f),
                Err(e) => {
                    tracing::warn!(path = %path, error = %e, "Rust parse failed");
                    None
                }
            };
            ops += 1;
        }
    }

    // Add new files
    for (path, content) in &plan.new_files {
        let abs_path = PathBuf::from(path); // Will be resolved by caller
        files.insert(
            path.clone(),
            SourceFile {
                rel_path: path.clone(),
                abs_path,
                text: content.clone(),
                parsed: match syn::parse_file(content) {
                    Ok(f) => Some(f),
                    Err(e) => {
                        tracing::warn!(path = %path, error = %e, "Rust parse failed");
                        None
                    }
                },
            },
        );
        ops += 1;
    }

    // Delete files
    for path in &plan.deleted_files {
        if files.remove(path).is_some() {
            ops += 1;
        }
    }

    ops
}

/// Apply Cargo.toml edits (dependency additions).
///
/// Reads the existing Cargo.toml, adds dependencies, writes back.
/// Returns the updated content, or None if no changes needed.
pub fn apply_cargo_edits(cargo_toml: &str, edits: &[CargoEdit]) -> Option<String> {
    if edits.is_empty() {
        return None;
    }

    let mut content = cargo_toml.to_string();

    // Find [dependencies] section
    let deps_marker = "[dependencies]";
    let insert_pos = if let Some(pos) = content.find(deps_marker) {
        // Insert after the [dependencies] line
        content[pos..].find('\n').map(|nl| pos + nl + 1)
    } else {
        // Add [dependencies] section at end
        content.push_str("\n[dependencies]\n");
        Some(content.len())
    };

    let insert_pos = insert_pos?;

    let mut additions = String::new();
    for edit in edits {
        // Check if dependency already exists
        let dep_pattern = format!("{} ", edit.dependency);
        let dep_pattern2 = format!("{}=", edit.dependency);
        if content.contains(&dep_pattern) || content.contains(&dep_pattern2) {
            continue;
        }

        if edit.features.is_empty() {
            let _ = writeln!(additions, "{} = \"{}\"", edit.dependency, edit.version);
        } else {
            let features = edit
                .features
                .iter()
                .map(|f| format!("\"{f}\""))
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(
                additions,
                "{} = {{ version = \"{}\", features = [{}] }}",
                edit.dependency, edit.version, features
            );
        }
    }

    if additions.is_empty() {
        return None;
    }

    content.insert_str(insert_pos, &additions);
    Some(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_plan_has_no_changes() {
        let plan = StructuralPlan::empty("test");
        assert!(!plan.has_changes());
        assert_eq!(plan.file_op_count(), 0);
    }

    #[test]
    fn plan_with_modifications() {
        let mut plan = StructuralPlan::empty("test");
        plan.modified_files
            .insert("src/main.rs".to_string(), "fn main() {}".to_string());
        assert!(plan.has_changes());
        assert_eq!(plan.file_op_count(), 1);
    }

    #[test]
    fn plan_with_new_files() {
        let mut plan = StructuralPlan::empty("test");
        plan.new_files
            .insert("src/types.rs".to_string(), "pub struct Foo;".to_string());
        assert!(plan.has_changes());
        assert_eq!(plan.file_op_count(), 1);
    }

    #[test]
    fn apply_plan_modifies_files() {
        let mut files = HashMap::new();
        files.insert(
            "src/main.rs".to_string(),
            SourceFile {
                rel_path: "src/main.rs".to_string(),
                abs_path: PathBuf::from("src/main.rs"),
                text: "fn main() { old() }".to_string(),
                parsed: None,
            },
        );

        let mut plan = StructuralPlan::empty("test");
        plan.modified_files
            .insert("src/main.rs".to_string(), "fn main() { new() }".to_string());

        let ops = apply_plan(&mut files, &plan);
        assert_eq!(ops, 1);
        assert_eq!(files["src/main.rs"].text, "fn main() { new() }");
    }

    #[test]
    fn apply_plan_creates_files() {
        let mut files = HashMap::new();

        let mut plan = StructuralPlan::empty("test");
        plan.new_files.insert(
            "src/types.rs".to_string(),
            "pub struct AcctInfo { pub number: String }".to_string(),
        );

        let ops = apply_plan(&mut files, &plan);
        assert_eq!(ops, 1);
        assert!(files.contains_key("src/types.rs"));
        assert!(files["src/types.rs"].text.contains("AcctInfo"));
    }

    #[test]
    fn apply_plan_deletes_files() {
        let mut files = HashMap::new();
        files.insert(
            "src/old.rs".to_string(),
            SourceFile {
                rel_path: "src/old.rs".to_string(),
                abs_path: PathBuf::from("src/old.rs"),
                text: "// obsolete".to_string(),
                parsed: None,
            },
        );

        let mut plan = StructuralPlan::empty("test");
        plan.deleted_files.push("src/old.rs".to_string());

        let ops = apply_plan(&mut files, &plan);
        assert_eq!(ops, 1);
        assert!(!files.contains_key("src/old.rs"));
    }

    #[test]
    fn apply_cargo_edits_adds_deps() {
        let cargo = "[package]\nname = \"test\"\n\n[dependencies]\nrust_decimal = \"1.0\"\n";

        let edits = vec![
            CargoEdit {
                dependency: "thiserror".to_string(),
                version: "1.0".to_string(),
                features: vec![],
            },
            CargoEdit {
                dependency: "tokio".to_string(),
                version: "1".to_string(),
                features: vec!["rt-multi-thread".to_string(), "macros".to_string()],
            },
        ];

        let result = apply_cargo_edits(cargo, &edits).unwrap();
        assert!(result.contains("thiserror = \"1.0\""));
        assert!(result.contains("tokio = { version = \"1\", features = [\"rt-multi-thread\", \"macros\"] }"));
    }

    #[test]
    fn apply_cargo_edits_skips_existing() {
        let cargo = "[dependencies]\nthiserror = \"1.0\"\n";

        let edits = vec![CargoEdit {
            dependency: "thiserror".to_string(),
            version: "2.0".to_string(),
            features: vec![],
        }];

        let result = apply_cargo_edits(cargo, &edits);
        assert!(result.is_none(), "should skip existing dependency");
    }

    #[test]
    fn apply_cargo_edits_empty_returns_none() {
        let cargo = "[dependencies]\n";
        let result = apply_cargo_edits(cargo, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn apply_cargo_edits_creates_section() {
        let cargo = "[package]\nname = \"test\"\n";

        let edits = vec![CargoEdit {
            dependency: "serde".to_string(),
            version: "1.0".to_string(),
            features: vec!["derive".to_string()],
        }];

        let result = apply_cargo_edits(cargo, &edits).unwrap();
        assert!(result.contains("[dependencies]"));
        assert!(result.contains("serde = { version = \"1.0\", features = [\"derive\"] }"));
    }
}
