//! REDEFINES group extraction and discriminator types.
//!
//! Scans a `DataEntry` tree for REDEFINES relationships, groups variants
//! by base field, and provides types for discriminator metadata.

use cobol_transpiler::ast::DataEntry;
use serde::{Deserialize, Serialize};

/// A group of REDEFINES variants sharing the same storage area.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedefinesGroup {
    /// Name of the base field (the one being redefined).
    pub base_field: String,
    /// Byte offset of the base field in the record.
    pub byte_offset: usize,
    /// Byte length of the base field.
    pub byte_length: usize,
    /// Variant overlays (each REDEFINES the base).
    pub variants: Vec<RedefinesVariant>,
    /// Discriminator info (if detected from PROCEDURE DIVISION).
    pub discriminator: Option<Discriminator>,
}

/// A single REDEFINES variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedefinesVariant {
    /// Name of the variant (the field that has REDEFINES clause).
    pub name: String,
    /// Names of the variant's child fields (leaf names for matching).
    pub fields: Vec<String>,
}

/// Discriminator metadata -- how to select a REDEFINES variant at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discriminator {
    /// Field name used as the discriminator.
    pub field: String,
    /// How the discriminator was detected.
    pub pattern_type: DiscriminatorPattern,
    /// Mapping rules: value -> variant name.
    pub rules: Vec<DiscriminatorRule>,
    /// Detection confidence level.
    pub confidence: Confidence,
}

/// How the discriminator pattern was detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscriminatorPattern {
    /// `IF field = 'value' ... variant-fields ... END-IF`
    DirectIf,
    /// `EVALUATE field WHEN 'value' ... variant-fields`
    EvaluateWhen,
    /// `IF condition-name ...` (88-level indirection)
    Level88,
}

/// A single discriminator rule: when field equals `value`, select `selects`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscriminatorRule {
    /// The discriminator value (e.g., "P", "B", "CR").
    pub value: String,
    /// The variant name this value selects.
    pub selects: String,
}

/// Confidence level for discriminator detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Confidence {
    /// No discriminator found -- needs SME review.
    Unresolved,
    /// Nested conditions -- uncertain (50-69%).
    Low,
    /// 88-level indirection (70-89%).
    Medium,
    /// Direct IF or EVALUATE match (90-100%).
    High,
}

/// Extract REDEFINES groups from a list of record entries.
///
/// Scans the top-level children of each 01-level record for REDEFINES
/// relationships. Groups variants by base field name. Returns groups
/// with `discriminator: None` (populated later by `detect_discriminators`).
pub fn extract_redefines_groups(entries: &[DataEntry]) -> Vec<RedefinesGroup> {
    let mut groups = Vec::new();

    for record in entries {
        extract_from_children(&record.children, &mut groups);
    }

    groups
}

/// Scan children for REDEFINES, building groups. Recurses into nested groups.
fn extract_from_children(children: &[DataEntry], groups: &mut Vec<RedefinesGroup>) {
    for child in children {
        // Recurse into group children to find nested REDEFINES
        if child.redefines.is_none() && !child.children.is_empty() {
            extract_from_children(&child.children, groups);
        }

        if let Some(ref target) = child.redefines {
            let target_upper = target.to_uppercase();

            // Find or create group for this base field.
            // Use prefix matching as fallback for fixed-format column 72 truncation
            // (e.g., "CC-TXN-PURCHASE-DA" should match "CC-TXN-PURCHASE-DATA").
            let resolved_target = resolve_target_name(&target_upper, children);
            let group = if let Some(g) = groups.iter_mut().find(|g| g.base_field == resolved_target) {
                g
            } else {
                let base_entry = children
                    .iter()
                    .find(|c| c.name.eq_ignore_ascii_case(&resolved_target));
                let (byte_offset, byte_length) = base_entry
                    .map(|c| (c.byte_offset.unwrap_or(0), c.byte_length.unwrap_or(0)))
                    .unwrap_or((0, 0));

                // Include base field's children as the first "variant"
                // so the UI highlights them as part of the REDEFINES region
                let mut variants = Vec::new();
                if let Some(base) = base_entry {
                    if !base.children.is_empty() {
                        let base_fields = collect_leaf_names(&base.children);
                        variants.push(RedefinesVariant {
                            name: base.name.to_uppercase(),
                            fields: base_fields,
                        });
                    }
                }

                groups.push(RedefinesGroup {
                    base_field: resolved_target.clone(),
                    byte_offset,
                    byte_length,
                    variants,
                    discriminator: None,
                });
                groups.last_mut().expect("just pushed to groups")
            };

            // Collect child field names for this variant
            let fields = collect_leaf_names(&child.children);

            group.variants.push(RedefinesVariant {
                name: child.name.to_uppercase(),
                fields,
            });
        }
    }
}

/// Resolve a REDEFINES target name that may be truncated at column 72.
/// Tries exact match first, then prefix match against sibling names.
fn resolve_target_name(target: &str, siblings: &[DataEntry]) -> String {
    // Exact match
    if siblings.iter().any(|c| c.name.eq_ignore_ascii_case(target)) {
        return target.to_string();
    }
    // Prefix match: find a sibling whose name starts with the truncated target
    if let Some(found) = siblings.iter().find(|c| {
        let name_upper = c.name.to_uppercase();
        name_upper.starts_with(target) && name_upper.len() > target.len()
    }) {
        return found.name.to_uppercase();
    }
    // No match -- return as-is
    target.to_string()
}

/// Collect leaf field names from a DataEntry tree (for variant field matching).
fn collect_leaf_names(entries: &[DataEntry]) -> Vec<String> {
    let mut names = Vec::new();
    for entry in entries {
        if entry.level == 88 {
            continue;
        }
        names.push(entry.name.to_uppercase());
        if !entry.children.is_empty() {
            names.extend(collect_leaf_names(&entry.children));
        }
    }
    names
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::{PicCategory, PicClause, Usage};

    fn leaf(level: u8, name: &str, length: usize) -> DataEntry {
        DataEntry {
            level,
            name: name.to_string(),
            pic: Some(PicClause {
                raw: format!("X({length})"),
                category: PicCategory::Alphanumeric,
                total_digits: 0,
                scale: 0,
                signed: false,
                display_length: length as u32,
                edit_symbols: vec![],
            }),
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: vec![],
            condition_values: vec![],
            byte_offset: Some(0),
            byte_length: Some(length),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    fn group(level: u8, name: &str, children: Vec<DataEntry>) -> DataEntry {
        let len: usize = children
            .iter()
            .filter(|c| c.redefines.is_none())
            .filter_map(|c| c.byte_length)
            .sum();
        DataEntry {
            level,
            name: name.to_string(),
            pic: None,
            usage: Usage::Display,
            value: None,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children,
            condition_values: vec![],
            byte_offset: Some(0),
            byte_length: Some(len),
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    #[test]
    fn test_extract_single_redefines() {
        let mut variant = group(
            5,
            "PERSONAL",
            vec![leaf(10, "FIRST-NAME", 25), leaf(10, "LAST-NAME", 25)],
        );
        variant.redefines = Some("BASE-DATA".to_string());

        let base = {
            let mut b = leaf(5, "BASE-DATA", 50);
            b.byte_offset = Some(0);
            b
        };
        let record = group(1, "RECORD", vec![base, variant]);
        let groups = extract_redefines_groups(&[record]);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].base_field, "BASE-DATA");
        assert_eq!(groups[0].byte_offset, 0);
        assert_eq!(groups[0].byte_length, 50);
        assert_eq!(groups[0].variants.len(), 1);
        assert_eq!(groups[0].variants[0].name, "PERSONAL");
        assert_eq!(groups[0].variants[0].fields, vec!["FIRST-NAME", "LAST-NAME"]);
        assert!(groups[0].discriminator.is_none());
    }

    #[test]
    fn test_extract_multiple_variants() {
        let base = {
            let mut b = leaf(5, "ACCT-DATA", 100);
            b.byte_offset = Some(10);
            b
        };
        let mut var_a = group(
            5,
            "PERSONAL-DATA",
            vec![leaf(10, "NAME", 50), leaf(10, "ADDR", 50)],
        );
        var_a.redefines = Some("ACCT-DATA".to_string());

        let mut var_b = group(
            5,
            "BUSINESS-DATA",
            vec![leaf(10, "COMPANY", 60), leaf(10, "DEPT", 40)],
        );
        var_b.redefines = Some("ACCT-DATA".to_string());

        let record = group(1, "RECORD", vec![base, var_a, var_b]);
        let groups = extract_redefines_groups(&[record]);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].variants.len(), 2);
        assert_eq!(groups[0].variants[0].name, "PERSONAL-DATA");
        assert_eq!(groups[0].variants[1].name, "BUSINESS-DATA");
    }

    #[test]
    fn test_no_redefines_returns_empty() {
        let record = group(1, "RECORD", vec![leaf(5, "A", 10), leaf(5, "B", 20)]);
        let groups = extract_redefines_groups(&[record]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_confidence_ordering() {
        assert!(Confidence::High > Confidence::Medium);
        assert!(Confidence::Medium > Confidence::Low);
        assert!(Confidence::Low > Confidence::Unresolved);
    }

    #[test]
    fn test_nested_leaf_names() {
        let inner = group(
            10,
            "INNER",
            vec![leaf(15, "FIELD-A", 5), leaf(15, "FIELD-B", 5)],
        );
        let names = collect_leaf_names(&[inner]);
        assert_eq!(names, vec!["INNER", "FIELD-A", "FIELD-B"]);
    }
}
