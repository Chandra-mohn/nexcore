//! File-to-copybook auto-discovery.
//!
//! Matches binary data files to COBOL copybooks via two strategies:
//! 1. Program link chains (SELECT/ASSIGN -> FD -> COPY -> copybook)
//! 2. Record length divisibility (file_size % record_length == 0)

use crate::redefines::Confidence;
use serde::Serialize;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Information about a parsed copybook.
#[derive(Debug, Clone, Serialize)]
pub struct CopybookInfo {
    /// File path.
    pub path: String,
    /// Basename (uppercase, no extension).
    pub stem: String,
    /// Computed record length (byte_length of first 01-level).
    pub record_length: Option<usize>,
}

/// Information about a parsed program.
#[derive(Debug, Clone, Serialize)]
pub struct ProgramInfo {
    /// File path.
    pub path: String,
    /// SELECT/ASSIGN/FD declarations.
    pub declarations: Vec<FileDeclaration>,
    /// COPY targets referenced in the program.
    pub copy_targets: Vec<String>,
}

/// A file declaration from ENVIRONMENT/FILE SECTION.
#[derive(Debug, Clone, Serialize)]
pub struct FileDeclaration {
    /// Logical file name (from SELECT).
    pub logical_name: String,
    /// Physical file name (from ASSIGN TO).
    pub physical_name: String,
    /// Recording mode (Fixed, Variable, Undefined).
    pub recording_mode: Option<RecordingMode>,
    /// Record size specification.
    pub record_size: Option<RecordSize>,
}

/// Recording mode.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum RecordingMode {
    Fixed,
    Variable,
    Undefined,
}

/// Record size specification.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum RecordSize {
    Fixed(usize),
    Variable { min: usize, max: usize },
}

/// Information about a binary data file.
#[derive(Debug, Clone, Serialize)]
pub struct DataFileInfo {
    /// File path.
    pub path: String,
    /// Basename (uppercase, no extension).
    pub stem: String,
    /// File size in bytes.
    pub size: u64,
}

/// A signal contributing to a match decision.
#[derive(Debug, Clone, Serialize)]
pub enum MatchSignal {
    /// Matched via SELECT/ASSIGN + COPY chain.
    ProgramLink,
    /// file_size % record_length == 0.
    RecordLengthMatch,
    /// Trial decode success rate.
    TrialDecode { score: f64 },
    /// Filename stem similarity.
    NameSimilarity { score: f64 },
}

/// A single match between a data file and a copybook.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryMatch {
    /// Copybook path.
    pub copybook_path: String,
    /// Program that established the link (if program-link match).
    pub program_path: Option<String>,
    /// Match confidence.
    pub confidence: Confidence,
    /// Contributing signals.
    pub signals: Vec<MatchSignal>,
    /// Computed record length.
    pub record_length: Option<usize>,
}

/// Discovery results for a single data file.
#[derive(Debug, Clone, Serialize)]
pub struct FileDiscoveryResult {
    /// Data file path.
    pub data_file: String,
    /// Data file size.
    pub file_size: u64,
    /// Matched copybooks (sorted by confidence, descending).
    pub matches: Vec<DiscoveryMatch>,
}

/// Full discovery report.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryReport {
    /// Per-file results.
    pub results: Vec<FileDiscoveryResult>,
    /// Copybooks without an 01-level record (fragments).
    pub fragment_copybooks: Vec<String>,
    /// Files that failed to parse.
    pub parse_errors: Vec<(String, String)>,
}

// ---------------------------------------------------------------------------
// Matching algorithms
// ---------------------------------------------------------------------------

/// Match data files to copybooks via program link chains.
///
/// HIGH confidence: program declares SELECT/ASSIGN -> file matches data file,
/// and program COPYs a copybook whose record length divides the file size.
pub fn match_by_program(
    programs: &[ProgramInfo],
    copybooks: &[CopybookInfo],
    dat_files: &[DataFileInfo],
) -> Vec<FileDiscoveryResult> {
    // Build copybook lookup by stem
    let cb_by_stem: HashMap<&str, &CopybookInfo> = copybooks
        .iter()
        .map(|cb| (cb.stem.as_str(), cb))
        .collect();

    let mut results_map: HashMap<String, FileDiscoveryResult> = HashMap::new();

    for program in programs {
        // Map COPY targets to copybooks
        let linked_copybooks: Vec<&CopybookInfo> = program
            .copy_targets
            .iter()
            .filter_map(|t| cb_by_stem.get(t.to_uppercase().as_str()).copied())
            .collect();

        for decl in &program.declarations {
            let physical = normalize_physical_name(&decl.physical_name);

            for dat in dat_files {
                if !physical_matches_dat(&physical, &dat.stem) {
                    continue;
                }

                for cb in &linked_copybooks {
                    let mut signals = vec![MatchSignal::ProgramLink];

                    // Check record length match
                    if let Some(rec_len) = cb.record_length {
                        if rec_len > 0 && dat.size > 0 && dat.size % (rec_len as u64) == 0 {
                            signals.push(MatchSignal::RecordLengthMatch);
                        }
                    }

                    let entry = results_map
                        .entry(dat.path.clone())
                        .or_insert_with(|| FileDiscoveryResult {
                            data_file: dat.path.clone(),
                            file_size: dat.size,
                            matches: Vec::new(),
                        });

                    entry.matches.push(DiscoveryMatch {
                        copybook_path: cb.path.clone(),
                        program_path: Some(program.path.clone()),
                        confidence: Confidence::High,
                        signals,
                        record_length: cb.record_length,
                    });
                }
            }
        }
    }

    results_map.into_values().collect()
}

/// Match data files to copybooks by record length divisibility.
///
/// MEDIUM/LOW confidence: file_size % record_length == 0,
/// optionally boosted by name similarity.
pub fn match_by_length(
    copybooks: &[CopybookInfo],
    dat_files: &[DataFileInfo],
) -> Vec<FileDiscoveryResult> {
    let mut results_map: HashMap<String, FileDiscoveryResult> = HashMap::new();

    for dat in dat_files {
        if dat.size == 0 {
            continue;
        }

        for cb in copybooks {
            let rec_len = match cb.record_length {
                Some(l) if l > 0 => l,
                _ => continue,
            };

            if dat.size % (rec_len as u64) != 0 {
                continue;
            }

            let mut signals = vec![MatchSignal::RecordLengthMatch];
            let mut confidence = Confidence::Low;

            // Name similarity boost
            if stems_similar(&dat.stem, &cb.stem) {
                signals.push(MatchSignal::NameSimilarity { score: 0.8 });
                confidence = Confidence::Medium;
            }

            let entry = results_map
                .entry(dat.path.clone())
                .or_insert_with(|| FileDiscoveryResult {
                    data_file: dat.path.clone(),
                    file_size: dat.size,
                    matches: Vec::new(),
                });

            entry.matches.push(DiscoveryMatch {
                copybook_path: cb.path.clone(),
                program_path: None,
                confidence,
                signals,
                record_length: Some(rec_len),
            });
        }
    }

    results_map.into_values().collect()
}

/// Merge program-link and length-based results, deduplicating by
/// (data_file, copybook) pairs. Program-link results take priority.
pub fn merge_results(
    program_matches: Vec<FileDiscoveryResult>,
    length_matches: Vec<FileDiscoveryResult>,
) -> DiscoveryReport {
    let mut merged: HashMap<String, FileDiscoveryResult> = HashMap::new();

    // Program matches first (higher priority)
    for result in program_matches {
        let entry = merged
            .entry(result.data_file.clone())
            .or_insert_with(|| FileDiscoveryResult {
                data_file: result.data_file.clone(),
                file_size: result.file_size,
                matches: Vec::new(),
            });
        entry.matches.extend(result.matches);
    }

    // Length matches: skip duplicates
    for result in length_matches {
        let entry = merged
            .entry(result.data_file.clone())
            .or_insert_with(|| FileDiscoveryResult {
                data_file: result.data_file.clone(),
                file_size: result.file_size,
                matches: Vec::new(),
            });

        for m in result.matches {
            let already_matched = entry
                .matches
                .iter()
                .any(|existing| existing.copybook_path == m.copybook_path);
            if !already_matched {
                entry.matches.push(m);
            }
        }
    }

    // Sort matches by confidence (descending) within each file
    let mut results: Vec<FileDiscoveryResult> = merged.into_values().collect();
    for r in &mut results {
        r.matches
            .sort_by(|a, b| b.confidence.cmp(&a.confidence));
    }
    results.sort_by(|a, b| a.data_file.cmp(&b.data_file));

    DiscoveryReport {
        results,
        fragment_copybooks: Vec::new(),
        parse_errors: Vec::new(),
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Normalize a physical file name: strip quotes, DD:, DSN= prefixes, uppercase.
pub fn normalize_physical_name(raw: &str) -> String {
    let mut s = raw.trim().to_uppercase();

    // Strip surrounding quotes
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s = s[1..s.len() - 1].to_string();
    }

    // Strip DD: prefix
    if let Some(rest) = s.strip_prefix("DD:") {
        s = rest.to_string();
    }

    // Strip DSN= prefix
    if let Some(rest) = s.strip_prefix("DSN=") {
        s = rest.to_string();
    }

    // Take last component after dots or slashes
    if let Some(last) = s.rsplit(&['/', '\\', '.'][..]).next() {
        if !last.is_empty() {
            return last.to_string();
        }
    }

    s
}

/// Check if a normalized physical name matches a data file stem.
fn physical_matches_dat(physical: &str, dat_stem: &str) -> bool {
    physical.eq_ignore_ascii_case(dat_stem)
        || dat_stem
            .to_uppercase()
            .contains(&physical.to_uppercase())
}

/// Simple stem similarity: check if one contains the other.
fn stems_similar(a: &str, b: &str) -> bool {
    let a_upper = a.to_uppercase();
    let b_upper = b.to_uppercase();
    a_upper.contains(&b_upper) || b_upper.contains(&a_upper)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_physical_name_simple() {
        assert_eq!(normalize_physical_name("ACCTFILE"), "ACCTFILE");
    }

    #[test]
    fn test_normalize_physical_name_quoted() {
        assert_eq!(normalize_physical_name("\"ACCTFILE\""), "ACCTFILE");
        assert_eq!(normalize_physical_name("'ACCTFILE'"), "ACCTFILE");
    }

    #[test]
    fn test_normalize_physical_name_dd_prefix() {
        assert_eq!(normalize_physical_name("DD:ACCTFILE"), "ACCTFILE");
    }

    #[test]
    fn test_normalize_physical_name_dsn() {
        assert_eq!(normalize_physical_name("DSN=MY.DATA.FILE"), "FILE");
    }

    #[test]
    fn test_physical_matches_dat() {
        assert!(physical_matches_dat("ACCTFILE", "ACCTFILE"));
        assert!(physical_matches_dat("ACCT", "ACCTFILE"));
        assert!(!physical_matches_dat("TRANSFILE", "ACCTFILE"));
    }

    #[test]
    fn test_match_by_program_basic() {
        let programs = vec![ProgramInfo {
            path: "PROG1.cbl".to_string(),
            declarations: vec![FileDeclaration {
                logical_name: "ACCT-FILE".to_string(),
                physical_name: "ACCTFILE".to_string(),
                recording_mode: None,
                record_size: None,
            }],
            copy_targets: vec!["ACCTCOPY".to_string()],
        }];

        let copybooks = vec![CopybookInfo {
            path: "ACCTCOPY.cpy".to_string(),
            stem: "ACCTCOPY".to_string(),
            record_length: Some(100),
        }];

        let dat_files = vec![DataFileInfo {
            path: "ACCTFILE.dat".to_string(),
            stem: "ACCTFILE".to_string(),
            size: 1000, // 10 records * 100 bytes
        }];

        let results = match_by_program(&programs, &copybooks, &dat_files);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].matches.len(), 1);
        assert_eq!(results[0].matches[0].confidence, Confidence::High);
        assert_eq!(results[0].matches[0].signals.len(), 2); // ProgramLink + RecordLengthMatch
    }

    #[test]
    fn test_match_by_length_basic() {
        let copybooks = vec![
            CopybookInfo {
                path: "COPY1.cpy".to_string(),
                stem: "COPY1".to_string(),
                record_length: Some(100),
            },
            CopybookInfo {
                path: "COPY2.cpy".to_string(),
                stem: "COPY2".to_string(),
                record_length: Some(200),
            },
        ];

        let dat_files = vec![DataFileInfo {
            path: "DATA.dat".to_string(),
            stem: "DATA".to_string(),
            size: 1000,
        }];

        let results = match_by_length(&copybooks, &dat_files);
        assert_eq!(results.len(), 1);
        // Both copybooks match: 1000 % 100 == 0 and 1000 % 200 == 0
        assert_eq!(results[0].matches.len(), 2);
    }

    #[test]
    fn test_match_by_length_no_match() {
        let copybooks = vec![CopybookInfo {
            path: "COPY1.cpy".to_string(),
            stem: "COPY1".to_string(),
            record_length: Some(300),
        }];

        let dat_files = vec![DataFileInfo {
            path: "DATA.dat".to_string(),
            stem: "DATA".to_string(),
            size: 1000, // 1000 % 300 != 0
        }];

        let results = match_by_length(&copybooks, &dat_files);
        assert!(results.is_empty() || results[0].matches.is_empty());
    }

    #[test]
    fn test_match_by_length_name_similarity() {
        let copybooks = vec![CopybookInfo {
            path: "ACCTCOPY.cpy".to_string(),
            stem: "ACCTCOPY".to_string(),
            record_length: Some(100),
        }];

        let dat_files = vec![DataFileInfo {
            path: "ACCTCOPY.dat".to_string(),
            stem: "ACCTCOPY".to_string(),
            size: 1000,
        }];

        let results = match_by_length(&copybooks, &dat_files);
        assert_eq!(results[0].matches[0].confidence, Confidence::Medium);
    }

    #[test]
    fn test_merge_deduplication() {
        let prog = vec![FileDiscoveryResult {
            data_file: "DATA.dat".to_string(),
            file_size: 1000,
            matches: vec![DiscoveryMatch {
                copybook_path: "COPY1.cpy".to_string(),
                program_path: Some("PROG1.cbl".to_string()),
                confidence: Confidence::High,
                signals: vec![MatchSignal::ProgramLink],
                record_length: Some(100),
            }],
        }];

        let length = vec![FileDiscoveryResult {
            data_file: "DATA.dat".to_string(),
            file_size: 1000,
            matches: vec![
                DiscoveryMatch {
                    copybook_path: "COPY1.cpy".to_string(), // duplicate
                    program_path: None,
                    confidence: Confidence::Low,
                    signals: vec![MatchSignal::RecordLengthMatch],
                    record_length: Some(100),
                },
                DiscoveryMatch {
                    copybook_path: "COPY2.cpy".to_string(), // new
                    program_path: None,
                    confidence: Confidence::Low,
                    signals: vec![MatchSignal::RecordLengthMatch],
                    record_length: Some(200),
                },
            ],
        }];

        let report = merge_results(prog, length);
        assert_eq!(report.results.len(), 1);
        // COPY1 (from program, not duplicated) + COPY2 (from length)
        assert_eq!(report.results[0].matches.len(), 2);
        // Sorted by confidence: High first
        assert_eq!(report.results[0].matches[0].confidence, Confidence::High);
    }

    #[test]
    fn test_stems_similar() {
        assert!(stems_similar("ACCTFILE", "ACCT"));
        assert!(stems_similar("ACCT", "ACCTFILE"));
        assert!(!stems_similar("TRANS", "ACCT"));
    }
}
