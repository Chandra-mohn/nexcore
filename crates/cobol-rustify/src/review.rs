//! Review file generation for Tier 2+ safety-gated items.
//!
//! Generates `rustify/review.md` listing items that need
//! engineer review before promotion.

use std::io::Write;
use std::path::Path;

use crate::error::RustifyError;
use crate::rules::transform::{Safety, Transform};

/// Write review.md for transforms that need engineer review.
pub fn write_review(
    output_dir: &Path,
    transforms: &[Transform],
    tier: u8,
) -> Result<(), RustifyError> {
    let review_items: Vec<_> = transforms.iter().filter(|t| t.safety.is_review()).collect();

    if review_items.is_empty() {
        return Ok(());
    }

    let rustify_dir = output_dir.join("rustify");
    std::fs::create_dir_all(&rustify_dir)?;

    let mut f = std::fs::File::create(rustify_dir.join("review.md"))?;
    write_review_content(&mut f, &review_items, tier)?;

    Ok(())
}

fn write_review_content<W: Write>(
    w: &mut W,
    items: &[&Transform],
    tier: u8,
) -> std::io::Result<()> {
    writeln!(w, "# Rustify Review -- Tier {tier}")?;
    writeln!(w)?;
    writeln!(w, "## Needs Review ({} items)", items.len())?;
    writeln!(w)?;

    // Group by rule
    let mut by_rule: std::collections::HashMap<&str, Vec<&&Transform>> =
        std::collections::HashMap::new();
    for item in items {
        by_rule.entry(item.rule_id.as_str()).or_default().push(item);
    }

    let mut rules: Vec<_> = by_rule.keys().copied().collect();
    rules.sort_unstable();

    for rule_id in rules {
        let rule_items = &by_rule[rule_id];
        writeln!(w, "### {rule_id}")?;
        writeln!(w)?;

        for (i, item) in rule_items.iter().enumerate() {
            let id = format!(
                "{}-{:03}",
                rule_id.to_uppercase().replace('-', ""),
                i + 1
            );
            writeln!(w, "#### [{id}] {}:{}", item.file.display(), item.line)?;

            if let Safety::Review {
                reason,
                recommendation,
            } = &item.safety
            {
                writeln!(w, "- **Reason**: {reason}")?;
                writeln!(w, "- **Recommendation**: {recommendation}")?;
            }

            writeln!(w, "- **Description**: {}", item.description)?;
            writeln!(w, "- **Action**: [ ] Approve  [ ] Keep as-is  [ ] Defer")?;
            writeln!(w)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn empty_review_no_file() {
        let dir = std::env::temp_dir().join("cobol2rust_test_review_empty");
        let _ = std::fs::create_dir_all(&dir);
        write_review(&dir, &[], 2).unwrap();
        assert!(!dir.join("rustify").join("review.md").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn review_with_items() {
        let transforms = vec![Transform {
            rule_id: "pic-to-string".to_string(),
            file: PathBuf::from("src/program_a.rs"),
            line: 42,
            description: "ws_account_number: PicX -> String".to_string(),
            safety: Safety::Review {
                reason: "field is redefined by ws_acct_parts".to_string(),
                recommendation: "keep as PicX until REDEFINES eliminated".to_string(),
            },
            edits: vec![],
        }];

        let mut buf = Vec::new();
        let items: Vec<_> = transforms.iter().collect();
        write_review_content(&mut buf, &items, 2).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(text.contains("Tier 2"));
        assert!(text.contains("pic-to-string"));
        assert!(text.contains("redefined by ws_acct_parts"));
        assert!(text.contains("[ ] Approve"));
    }
}
