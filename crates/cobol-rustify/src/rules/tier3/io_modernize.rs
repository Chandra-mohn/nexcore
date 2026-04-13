//! T3-IO: Analyze file I/O patterns for modernization.
//!
//! Finds COBOL file I/O sequences (open/read/write/close) and maps
//! them to idiomatic Rust patterns. Assessment only.

use std::collections::HashMap;
use std::fmt::Write;

use crate::rules::transform::{Safety, Transform};
use crate::rules::{AnalysisContext, RustifyRule, Tier};

#[derive(Debug)]
pub struct IoModernizeRule;

impl RustifyRule for IoModernizeRule {
    fn id(&self) -> &'static str {
        "io-modernize"
    }

    fn description(&self) -> &'static str {
        "Analyze file I/O patterns for Rust idiom modernization"
    }

    fn tier(&self) -> Tier {
        Tier::Tier3
    }

    fn analyze(&self, ctx: &AnalysisContext) -> Vec<Transform> {
        analyze_io(ctx.source_text, ctx.file_path)
    }
}

/// Access pattern for a file handle.
#[derive(Debug, Clone, PartialEq)]
enum AccessPattern {
    ReadSequential,
    WriteSequential,
    ReadWrite,
    Indexed,
    Unknown,
}

impl std::fmt::Display for AccessPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadSequential => write!(f, "sequential read"),
            Self::WriteSequential => write!(f, "sequential write"),
            Self::ReadWrite => write!(f, "read-write"),
            Self::Indexed => write!(f, "indexed access"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// A detected file I/O handle with its operations.
#[derive(Debug)]
struct FileHandle {
    name: String,
    operations: Vec<String>,
    pattern: AccessPattern,
}

fn analyze_io(
    source_text: &str,
    file_path: &std::path::Path,
) -> Vec<Transform> {
    let lines: Vec<&str> = source_text.lines().collect();

    let handles = detect_file_handles(&lines);
    if handles.is_empty() {
        return Vec::new();
    }

    let mut desc = format!("{} file handle(s) detected", handles.len());
    let patterns: Vec<String> = handles
        .iter()
        .map(|h| format!("{}: {}", h.name, h.pattern))
        .collect();
    let _ = write!(desc, " -- {}", patterns.join(", "));

    let mut detail = String::new();
    let _ = writeln!(detail, "File handles: {}\n", handles.len());

    for handle in &handles {
        let _ = writeln!(detail, "{}:", handle.name);
        let _ = writeln!(detail, "  Pattern: {}", handle.pattern);
        let _ = writeln!(detail, "  Operations: {}", handle.operations.join(", "));
        detail.push_str("  Recommendation: ");
        match handle.pattern {
            AccessPattern::ReadSequential => {
                detail.push_str("Replace with BufReader + lines() iterator\n");
                detail.push_str("  Pattern: for line in BufReader::new(file).lines() { ... }\n");
            }
            AccessPattern::WriteSequential => {
                detail.push_str("Replace with BufWriter + write!/writeln!\n");
                detail.push_str("  Pattern: let mut writer = BufWriter::new(file); writeln!(writer, ...);\n");
            }
            AccessPattern::ReadWrite => {
                detail.push_str("Consider splitting into separate read/write phases\n");
                detail.push_str("  Or use OpenOptions::new().read(true).write(true)\n");
            }
            AccessPattern::Indexed => {
                detail.push_str("Consider HashMap<Key, Record> or BTreeMap for indexed access\n");
                detail.push_str("  Or use a database (SQLite/DuckDB) for persistent indexed storage\n");
            }
            AccessPattern::Unknown => {
                detail.push_str("Manual analysis needed\n");
            }
        }
        detail.push('\n');
    }

    vec![Transform {
        rule_id: "io-modernize".to_string(),
        file: file_path.to_path_buf(),
        line: 0,
        description: desc,
        safety: Safety::Assessment,
        edits: vec![],
    }]
}

/// Detect file handles and their operations from source.
fn detect_file_handles(lines: &[&str]) -> Vec<FileHandle> {
    let mut ops: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let trimmed = line.trim();

        // Pattern: `ws.FIELD.open_input(...)` or `ws.FIELD.open_output(...)`
        if let Some((handle, op)) = parse_file_op(trimmed) {
            ops.entry(handle).or_default().push(op);
        }
    }

    let mut handles: Vec<FileHandle> = ops
        .into_iter()
        .map(|(name, operations)| {
            let pattern = classify_pattern(&operations);
            FileHandle {
                name,
                operations,
                pattern,
            }
        })
        .collect();

    handles.sort_by(|a, b| a.name.cmp(&b.name));
    handles
}

/// Parse file I/O operations.
fn parse_file_op(line: &str) -> Option<(String, String)> {
    // Find "ws.HANDLE.OP(" anywhere in the line
    let ws_pos = line.find("ws.")?;
    let after_ws = &line[ws_pos + 3..];

    let file_ops = [
        "open_input", "open_output", "open_io", "open_extend",
        "close_file", "read_next", "read_record", "read_by_key",
        "write_record", "rewrite_record", "delete_record",
        "start",
    ];

    for op in &file_ops {
        let pattern = format!(".{op}(");
        if let Some(pos) = after_ws.find(&pattern) {
            let handle = &after_ws[..pos];
            if !handle.contains('.') && !handle.contains(' ') {
                return Some((handle.to_string(), op.to_string()));
            }
        }
    }

    None
}

/// Classify access pattern from operations.
fn classify_pattern(ops: &[String]) -> AccessPattern {
    let has_read = ops.iter().any(|o| o.starts_with("read"));
    let has_write = ops.iter().any(|o| o == "write_record");
    let has_rewrite = ops.iter().any(|o| o == "rewrite_record");
    let has_indexed = ops.iter().any(|o| o == "read_by_key" || o == "start");

    if has_indexed {
        AccessPattern::Indexed
    } else if has_read && (has_write || has_rewrite) {
        AccessPattern::ReadWrite
    } else if has_read {
        AccessPattern::ReadSequential
    } else if has_write {
        AccessPattern::WriteSequential
    } else {
        AccessPattern::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn detects_sequential_read() {
        let source = r#"fn process(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.input_file.open_input("data.dat");
    loop {
        if ws.input_file.read_next(&mut ws.input_record).is_err() {
            break;
        }
    }
    ws.input_file.close_file();
}
"#;
        let transforms = analyze_io(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("sequential read"));
    }

    #[test]
    fn detects_sequential_write() {
        let source = r#"fn output(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.out_file.open_output("output.dat");
    ws.out_file.write_record(ws.out_record.as_bytes());
    ws.out_file.close_file();
}
"#;
        let transforms = analyze_io(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("sequential write"));
    }

    #[test]
    fn detects_indexed() {
        let source = r#"fn lookup(ws: &mut WorkingStorage, ctx: &mut ProgramContext) {
    ws.master_file.open_io("master.idx");
    ws.master_file.read_by_key(ws.ws_key.as_bytes());
    ws.master_file.rewrite_record(ws.master_record.as_bytes());
    ws.master_file.close_file();
}
"#;
        let transforms = analyze_io(source, &PathBuf::from("test.rs"));
        assert_eq!(transforms.len(), 1);
        assert!(transforms[0].description.contains("indexed"));
    }

    #[test]
    fn no_io_no_transforms() {
        let source = "fn main() { println!(\"hello\"); }\n";
        let transforms = analyze_io(source, &PathBuf::from("test.rs"));
        assert!(transforms.is_empty());
    }

    #[test]
    fn classify_works() {
        assert_eq!(
            classify_pattern(&["open_input".to_string(), "read_next".to_string(), "close_file".to_string()]),
            AccessPattern::ReadSequential
        );
        assert_eq!(
            classify_pattern(&["open_output".to_string(), "write_record".to_string()]),
            AccessPattern::WriteSequential
        );
        assert_eq!(
            classify_pattern(&["read_by_key".to_string(), "rewrite_record".to_string()]),
            AccessPattern::Indexed
        );
    }
}
