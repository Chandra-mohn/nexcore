//! Transform types representing proposed code changes.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A proposed code transform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transform {
    /// Rule that proposed this transform.
    pub rule_id: String,
    /// File this transform applies to.
    pub file: PathBuf,
    /// Line number (1-based).
    pub line: usize,
    /// Human-readable description.
    pub description: String,
    /// Safety classification.
    pub safety: Safety,
    /// Edits to apply (empty for assessment-only transforms).
    #[serde(default)]
    pub edits: Vec<TextEdit>,
}

/// A text-level edit: replace a span of source text with new text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextEdit {
    /// Line number (1-based).
    pub line: usize,
    /// Column start (0-based byte offset within the line).
    pub col_start: usize,
    /// Column end (0-based byte offset, exclusive).
    pub col_end: usize,
    /// Replacement text.
    pub new_text: String,
}

/// An insertion of new lines at a specific position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineInsertion {
    /// Insert after this line number (1-based). 0 = beginning of file.
    pub after_line: usize,
    /// Text to insert (may contain newlines).
    pub text: String,
}

/// Safety classification for a transform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Safety {
    /// Fully automatic, no review needed.
    Auto,
    /// Needs engineer review.
    Review {
        reason: String,
        recommendation: String,
    },
    /// Assessment only, no auto-apply (Tier 3).
    Assessment,
}

impl Safety {
    pub fn is_auto(&self) -> bool {
        matches!(self, Self::Auto)
    }

    pub fn is_review(&self) -> bool {
        matches!(self, Self::Review { .. })
    }
}

/// Apply a set of text edits to source text, returning the modified source.
///
/// Edits are applied bottom-up (highest line first) to preserve offsets.
/// Multiple edits on the same line are applied right-to-left.
pub fn apply_edits(source: &str, edits: &[TextEdit]) -> String {
    if edits.is_empty() {
        return source.to_string();
    }

    let mut lines: Vec<String> = source.lines().map(String::from).collect();
    // Preserve trailing newline
    let trailing_newline = source.ends_with('\n');

    // Sort edits: by line descending, then by col_start descending
    let mut sorted: Vec<&TextEdit> = edits.iter().collect();
    sorted.sort_by(|a, b| {
        b.line.cmp(&a.line).then_with(|| b.col_start.cmp(&a.col_start))
    });

    for edit in sorted {
        if edit.line == 0 || edit.line > lines.len() {
            continue;
        }
        let idx = edit.line - 1;
        let line = &lines[idx];
        let start = edit.col_start.min(line.len());
        let end = edit.col_end.min(line.len());
        let mut new_line = String::with_capacity(line.len());
        new_line.push_str(&line[..start]);
        new_line.push_str(&edit.new_text);
        new_line.push_str(&line[end..]);
        lines[idx] = new_line;
    }

    let mut result = lines.join("\n");
    if trailing_newline {
        result.push('\n');
    }
    result
}

/// Apply line insertions to source text.
///
/// Insertions are applied bottom-up to preserve line numbers.
pub fn apply_insertions(source: &str, insertions: &[LineInsertion]) -> String {
    if insertions.is_empty() {
        return source.to_string();
    }

    let mut lines: Vec<String> = source.lines().map(String::from).collect();
    let trailing_newline = source.ends_with('\n');

    // Sort by after_line descending
    let mut sorted: Vec<&LineInsertion> = insertions.iter().collect();
    sorted.sort_by(|a, b| b.after_line.cmp(&a.after_line));

    for ins in sorted {
        let insert_at = ins.after_line.min(lines.len());
        let new_lines: Vec<String> = ins.text.lines().map(String::from).collect();
        for (i, new_line) in new_lines.into_iter().enumerate() {
            lines.insert(insert_at + i, new_line);
        }
    }

    let mut result = lines.join("\n");
    if trailing_newline {
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_single_edit() {
        //                  01234567890123456789012345678901234567
        let source = "let x = \"0\".parse::<Decimal>().unwrap();\n";
        // "0".parse::<Decimal>().unwrap() spans col 8..39 (`;` is at col 39)
        let edits = vec![TextEdit {
            line: 1,
            col_start: 8,
            col_end: 39,
            new_text: "Decimal::ZERO".to_string(),
        }];
        let result = apply_edits(source, &edits);
        assert_eq!(result, "let x = Decimal::ZERO;\n");
    }

    #[test]
    fn apply_multiple_edits_same_line() {
        let source = "let a = \"0\".parse::<Decimal>().unwrap() + \"0\".parse::<Decimal>().unwrap();\n";
        // First:  col 8..39
        // " + " at 39..42
        // Second: col 42..73
        // ";" at 73
        let edits = vec![
            TextEdit {
                line: 1,
                col_start: 8,
                col_end: 39,
                new_text: "Decimal::ZERO".to_string(),
            },
            TextEdit {
                line: 1,
                col_start: 42,
                col_end: 73,
                new_text: "Decimal::ZERO".to_string(),
            },
        ];
        let result = apply_edits(source, &edits);
        assert_eq!(result, "let a = Decimal::ZERO + Decimal::ZERO;\n");
    }

    #[test]
    fn apply_edits_different_lines() {
        let source = "line1 old\nline2 old\nline3\n";
        let edits = vec![
            TextEdit {
                line: 1,
                col_start: 6,
                col_end: 9,
                new_text: "new".to_string(),
            },
            TextEdit {
                line: 2,
                col_start: 6,
                col_end: 9,
                new_text: "new".to_string(),
            },
        ];
        let result = apply_edits(source, &edits);
        assert_eq!(result, "line1 new\nline2 new\nline3\n");
    }

    #[test]
    fn apply_empty_edits() {
        let source = "unchanged\n";
        let result = apply_edits(source, &[]);
        assert_eq!(result, source);
    }

    #[test]
    fn apply_line_insertion() {
        let source = "use something;\n\nfn main() {}\n";
        let insertions = vec![LineInsertion {
            after_line: 1,
            text: "use rust_decimal::Decimal;".to_string(),
        }];
        let result = apply_insertions(source, &insertions);
        assert_eq!(
            result,
            "use something;\nuse rust_decimal::Decimal;\n\nfn main() {}\n"
        );
    }

    #[test]
    fn apply_insertion_at_start() {
        let source = "fn main() {}\n";
        let insertions = vec![LineInsertion {
            after_line: 0,
            text: "// header".to_string(),
        }];
        let result = apply_insertions(source, &insertions);
        assert_eq!(result, "// header\nfn main() {}\n");
    }
}
