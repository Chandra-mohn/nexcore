//! T1-DEAD: Remove unused WorkingStorage fields.
//!
//! Requires hints.json with read_count and write_count per field.
//! Removes fields where both counts are zero.
//! Never removes fields that are part of REDEFINES groups.

use crate::hints::FileHints;
use crate::rules::transform::{Safety, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct DeadFieldRule;

impl RustifyRule for DeadFieldRule {
    fn id(&self) -> &'static str {
        "dead-field"
    }

    fn description(&self) -> &'static str {
        "Remove unused WorkingStorage fields (requires hints)"
    }

    fn tier(&self) -> Tier {
        Tier::Tier1
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        let Some(hints) = ctx.hints else {
            return Vec::new();
        };
        find_dead_fields(hints, ctx.file_path)
    }
}

fn find_dead_fields(hints: &FileHints, file_path: &std::path::Path) -> Vec<Transform> {
    let mut transforms = Vec::new();

    for (field_name, hint) in &hints.fields {
        // Skip if field is used at all
        if hint.read_count > 0 || hint.write_count > 0 {
            continue;
        }

        // Skip if field is part of REDEFINES
        if hint.redefines.is_some() || !hint.redefined_by.is_empty() {
            continue;
        }

        // Skip if field is used in CALL BY REFERENCE
        if hint.call_by_ref {
            continue;
        }

        // Skip if field is an I/O record field
        // (file_io_fields are checked at the FileHints level)

        // Skip if field is a MOVE CORRESPONDING target
        if hint.move_corr_target {
            continue;
        }

        transforms.push(Transform {
            rule_id: "dead-field".to_string(),
            file: file_path.to_path_buf(),
            line: 0, // We don't know the line without parsing
            description: format!(
                "{field_name}: unused (read=0, write=0) -- remove from WorkingStorage"
            ),
            safety: Safety::Auto,
            edits: vec![], // Text edits will be added when we implement apply for this rule
        });
    }

    // Also skip file I/O fields
    transforms.retain(|t| {
        let field_name = t.description.split(':').next().unwrap_or("");
        !hints.file_io_fields.contains(&field_name.to_string())
    });

    transforms.sort_by(|a, b| a.description.cmp(&b.description));
    transforms
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::FieldHint;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_hints(fields: Vec<(&str, u32, u32)>) -> FileHints {
        let mut field_map = HashMap::new();
        for (name, reads, writes) in fields {
            field_map.insert(
                name.to_string(),
                FieldHint {
                    pic: "X(10)".to_string(),
                    usage: "DISPLAY".to_string(),
                    level: 5,
                    redefines: None,
                    redefined_by: vec![],
                    call_by_ref: false,
                    move_corr_target: false,
                    read_count: reads,
                    write_count: writes,
                    paragraph_scope: vec![],
                },
            );
        }
        FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields: field_map,
            paragraphs: HashMap::new(),
            level_88_groups: HashMap::new(),
            call_targets: vec![],
            file_io_fields: vec![],
        }
    }

    #[test]
    fn detects_dead_fields() {
        let hints = make_hints(vec![
            ("ws_used", 5, 2),
            ("ws_dead", 0, 0),
            ("ws_write_only", 0, 3),
        ]);
        let transforms = find_dead_fields(&hints, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("ws_dead"));
    }

    #[test]
    fn skips_redefines_fields() {
        let mut hints = make_hints(vec![("ws_redef", 0, 0)]);
        hints.fields.get_mut("ws_redef").unwrap().redefines = Some("ws_original".to_string());
        let transforms = find_dead_fields(&hints, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn skips_redefined_by_fields() {
        let mut hints = make_hints(vec![("ws_original", 0, 0)]);
        hints
            .fields
            .get_mut("ws_original")
            .unwrap()
            .redefined_by = vec!["ws_redef".to_string()];
        let transforms = find_dead_fields(&hints, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn skips_call_by_ref() {
        let mut hints = make_hints(vec![("ws_param", 0, 0)]);
        hints.fields.get_mut("ws_param").unwrap().call_by_ref = true;
        let transforms = find_dead_fields(&hints, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn skips_file_io_fields() {
        let mut hints = make_hints(vec![("ws_record", 0, 0)]);
        hints.file_io_fields = vec!["ws_record".to_string()];
        let transforms = find_dead_fields(&hints, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn no_hints_no_transforms() {
        let rule = DeadFieldRule;
        let source = "fn main() {}\n";
        let parsed = syn::parse_file(source).unwrap();
        let ctx = AnalysisContext {
            source: &parsed,
            source_text: source,
            file_path: &PathBuf::from("test.rs"),
            hints: None,
        };
        let transforms = rule.analyze(&ctx);
        assert!(transforms.is_empty());
    }
}
