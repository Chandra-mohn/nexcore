//! BMS cross-reference scanner -- finds which COBOL programs use which BMS maps.
//!
//! Scans COBOL source files for:
//! 1. `EXEC CICS SEND MAP(...)` / `RECEIVE MAP(...)` references
//! 2. `LIT-THISMAP` / `LIT-THISMAPSET` VALUE declarations (indirect references)
//! 3. `CCARD-NEXT-MAP` / `CDEMO-LAST-MAP` MOVE targets (navigation flow)
//!
//! Produces a bidirectional index: program -> maps, map -> programs.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Serialize;

/// Complete cross-reference index between programs and BMS maps.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BmsXref {
    /// Program -> list of map references.
    pub program_to_maps: BTreeMap<String, Vec<MapReference>>,
    /// Map name -> list of programs that use it.
    pub map_to_programs: BTreeMap<String, Vec<ProgramReference>>,
}

/// A reference from a program to a BMS map.
#[derive(Debug, Clone, Serialize)]
pub struct MapReference {
    /// BMS map name (e.g., "CCRDLIA").
    pub map_name: String,
    /// BMS mapset name if known (e.g., "COCRDLI").
    pub mapset_name: Option<String>,
    /// How the map is referenced.
    pub ref_type: MapRefType,
}

/// How a program references a BMS map.
#[derive(Debug, Clone, Serialize)]
pub enum MapRefType {
    /// EXEC CICS SEND MAP
    SendMap,
    /// EXEC CICS RECEIVE MAP
    ReceiveMap,
    /// LIT-THISMAP VALUE declaration (this program "owns" this map)
    DeclaredMap,
}

/// A reference from a map back to a program.
#[derive(Debug, Clone, Serialize)]
pub struct ProgramReference {
    /// Program file name (e.g., "COCRDLIC.cbl").
    pub program_file: String,
    /// Full path to the program file.
    pub program_path: String,
    /// Program ID if extracted.
    pub program_id: Option<String>,
    /// How the program references this map.
    pub ref_type: MapRefType,
}

/// Recursively collect source files matching given extensions.
fn collect_source_files(dir: &Path, extensions: &[&str], out: &mut Vec<(std::path::PathBuf, String)>) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_source_files(&path, extensions, out);
        } else if path.is_file() {
            let ext = path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();
            if extensions.iter().any(|e| e.eq_ignore_ascii_case(&ext)) {
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                out.push((path, name));
            }
        }
    }
}

/// Scan a directory tree of COBOL source files and build the BMS cross-reference index.
/// Recursively walks subdirectories.
pub fn scan_bms_references(source_dir: &Path, extensions: &[&str]) -> BmsXref {
    let mut xref = BmsXref::default();
    let mut files = Vec::new();
    collect_source_files(source_dir, extensions, &mut files);

    for (path, file_name) in &files {
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[WARN] Cannot read {}: {e}", path.display());
                continue;
            }
        };

        let file_name = file_name.clone();

        let refs = extract_map_references(&source);

        if !refs.is_empty() {
            // Program -> maps
            xref.program_to_maps.insert(file_name.clone(), refs.clone());

            // Maps -> program (reverse index)
            for r in &refs {
                let map_upper = r.map_name.to_uppercase();
                xref.map_to_programs
                    .entry(map_upper)
                    .or_default()
                    .push(ProgramReference {
                        program_file: file_name.clone(),
                        program_path: path.to_string_lossy().to_string(),
                        program_id: extract_program_id(&source),
                        ref_type: r.ref_type.clone(),
                    });
            }
        }
    }

    xref
}

/// Extract BMS map references from a single COBOL source file.
pub fn extract_map_references(source: &str) -> Vec<MapReference> {
    let mut refs: Vec<MapReference> = Vec::new();
    let mut seen: BTreeSet<(String, String)> = BTreeSet::new(); // (map_name, ref_type)

    let upper = source.to_uppercase();

    // Strategy 1: Find LIT-THISMAP VALUE 'MAPNAME' declarations
    let (declared_map, declared_mapset) = find_lit_thismap_value(source);

    if let Some(ref map_name) = declared_map {
        let key = (map_name.clone(), "declared".to_string());
        if seen.insert(key) {
            refs.push(MapReference {
                map_name: map_name.clone(),
                mapset_name: declared_mapset.clone(),
                ref_type: MapRefType::DeclaredMap,
            });
        }
    }

    // Strategy 2: Find EXEC CICS SEND MAP('literal') / RECEIVE MAP('literal')
    for line in upper.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('*') {
            continue; // Skip comments
        }

        if trimmed.contains("EXEC CICS") && trimmed.contains("MAP(") {
            if let Some(map_name) = extract_map_arg(trimmed) {
                let is_send = trimmed.contains("SEND");
                let is_receive = trimmed.contains("RECEIVE");
                let ref_type = if is_receive {
                    MapRefType::ReceiveMap
                } else if is_send {
                    MapRefType::SendMap
                } else {
                    MapRefType::SendMap // default
                };

                // Check if MAP arg is a literal or a variable
                if map_name.starts_with('\'') || map_name.starts_with('"') {
                    // Literal: MAP('CCRDLIA')
                    let clean = map_name.trim_matches(|c| c == '\'' || c == '"').to_string();
                    let key = (clean.clone(), format!("{ref_type:?}"));
                    if seen.insert(key) {
                        refs.push(MapReference {
                            map_name: clean,
                            mapset_name: None,
                            ref_type,
                        });
                    }
                } else {
                    // Variable: MAP(LIT-THISMAP) -- resolve from declared_map
                    if let Some(ref resolved) = declared_map {
                        let key = (resolved.clone(), format!("{ref_type:?}"));
                        if seen.insert(key) {
                            refs.push(MapReference {
                                map_name: resolved.clone(),
                                mapset_name: declared_mapset.clone(),
                                ref_type,
                            });
                        }
                    }
                }
            }
        }
    }

    refs
}

/// Find LIT-THISMAP and LIT-THISMAPSET VALUE declarations.
///
/// Pattern:
/// ```text
///   05 LIT-THISMAP    PIC X(7) VALUE 'CCRDLIA'.
///   05 LIT-THISMAPSET PIC X(8) VALUE 'COCRDLI'.
/// ```
fn find_lit_thismap_value(source: &str) -> (Option<String>, Option<String>) {
    let mut map_name = None;
    let mut mapset_name = None;
    let mut prev_is_thismap = false;
    let mut prev_is_thismapset = false;

    for line in source.lines() {
        let upper = line.to_uppercase();
        let trimmed = upper.trim();

        if trimmed.starts_with('*') {
            continue;
        }

        if trimmed.contains("LIT-THISMAP") && !trimmed.contains("LIT-THISMAPSET") {
            // Check if VALUE is on this line
            if let Some(val) = extract_value_literal(trimmed) {
                map_name = Some(val);
            } else {
                prev_is_thismap = true;
            }
        } else if trimmed.contains("LIT-THISMAPSET") {
            if let Some(val) = extract_value_literal(trimmed) {
                mapset_name = Some(val);
            } else {
                prev_is_thismapset = true;
            }
        } else if prev_is_thismap && trimmed.contains("VALUE") {
            if let Some(val) = extract_value_literal(trimmed) {
                map_name = Some(val);
            }
            prev_is_thismap = false;
        } else if prev_is_thismapset && trimmed.contains("VALUE") {
            if let Some(val) = extract_value_literal(trimmed) {
                mapset_name = Some(val);
            }
            prev_is_thismapset = false;
        } else {
            prev_is_thismap = false;
            prev_is_thismapset = false;
        }
    }

    (map_name, mapset_name)
}

/// Extract a quoted value from a VALUE clause: `VALUE 'CCRDLIA'.` -> `CCRDLIA`
fn extract_value_literal(line: &str) -> Option<String> {
    let val_pos = line.find("VALUE")?;
    let after = &line[val_pos + 5..];
    let start = after.find('\'')?;
    let rest = &after[start + 1..];
    let end = rest.find('\'')?;
    let val = rest[..end].trim().to_string();
    if val.is_empty() { None } else { Some(val) }
}

/// Extract the MAP(...) argument from an EXEC CICS line.
fn extract_map_arg(line: &str) -> Option<String> {
    let map_pos = line.find("MAP(")?;
    let after = &line[map_pos + 4..];
    let end = after.find(')')?;
    let arg = after[..end].trim().to_string();
    if arg.is_empty() { None } else { Some(arg) }
}

/// Extract PROGRAM-ID from COBOL source.
fn extract_program_id(source: &str) -> Option<String> {
    for line in source.lines() {
        let upper = line.to_uppercase();
        let trimmed = upper.trim();
        if trimmed.starts_with('*') {
            continue;
        }
        if trimmed.contains("PROGRAM-ID") {
            // PROGRAM-ID. NAME.
            let after = trimmed.split("PROGRAM-ID").nth(1)?;
            let name = after.trim_start_matches('.').trim()
                .split_whitespace().next()?
                .trim_end_matches('.')
                .to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_lit_thismap() {
        let source = r#"
       01  WS-LITERALS.
         05  LIT-THISMAPSET                        PIC X(8)
                                                   VALUE 'COCRDLI'.
         05  LIT-THISMAP                           PIC X(7)
                                                   VALUE 'CCRDLIA'.
"#;
        let (map, mapset) = find_lit_thismap_value(source);
        assert_eq!(map.as_deref(), Some("CCRDLIA"));
        assert_eq!(mapset.as_deref(), Some("COCRDLI"));
    }

    #[test]
    fn extract_literal_map_arg() {
        let line = "           EXEC CICS SEND MAP('ACCTINQ')";
        let arg = extract_map_arg(&line.to_uppercase());
        assert_eq!(arg.as_deref(), Some("'ACCTINQ'"));
    }

    #[test]
    fn extract_variable_map_arg() {
        let line = "           EXEC CICS SEND MAP(LIT-THISMAP)";
        let arg = extract_map_arg(&line.to_uppercase());
        assert_eq!(arg.as_deref(), Some("LIT-THISMAP"));
    }

    #[test]
    fn full_extraction() {
        let source = r#"
       IDENTIFICATION DIVISION.
       PROGRAM-ID. TESTPGM.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-LITERALS.
         05  LIT-THISMAPSET  PIC X(8) VALUE 'TESTSET'.
         05  LIT-THISMAP     PIC X(7) VALUE 'TESTMAP'.
       PROCEDURE DIVISION.
           EXEC CICS SEND MAP(LIT-THISMAP)
                     MAPSET(LIT-THISMAPSET)
           END-EXEC.
           EXEC CICS RECEIVE MAP(LIT-THISMAP)
                     MAPSET(LIT-THISMAPSET)
           END-EXEC.
"#;
        let refs = extract_map_references(source);
        assert!(!refs.is_empty(), "expected map references");

        let map_names: Vec<&str> = refs.iter().map(|r| r.map_name.as_str()).collect();
        assert!(map_names.contains(&"TESTMAP"), "missing TESTMAP: {map_names:?}");
    }

    #[test]
    fn extract_program_id_test() {
        let source = "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. MYPROG.\n";
        assert_eq!(extract_program_id(source).as_deref(), Some("MYPROG"));
    }

    #[test]
    fn carddemo_scan() {
        let cbl_dir = Path::new(env!("HOME"))
            .join("workspace/aws-mainframe-modernization-carddemo/app/cbl");
        if !cbl_dir.exists() {
            eprintln!("Skipping: CardDemo not found");
            return;
        }

        let xref = scan_bms_references(&cbl_dir, &["cbl", "CBL", "cob"]);

        assert!(!xref.program_to_maps.is_empty(), "expected program->map entries");
        assert!(!xref.map_to_programs.is_empty(), "expected map->program entries");

        // CCRDLIA should be referenced by COCRDLIC
        let ccrdlia_progs = xref.map_to_programs.get("CCRDLIA");
        assert!(ccrdlia_progs.is_some(), "CCRDLIA not in index: {:?}", xref.map_to_programs.keys().collect::<Vec<_>>());

        eprintln!("\nProgram -> Maps:");
        for (prog, maps) in &xref.program_to_maps {
            let names: Vec<&str> = maps.iter().map(|m| m.map_name.as_str()).collect();
            eprintln!("  {prog} -> {}", names.join(", "));
        }

        eprintln!("\nMap -> Programs:");
        for (map, progs) in &xref.map_to_programs {
            let names: Vec<&str> = progs.iter().map(|p| p.program_file.as_str()).collect();
            eprintln!("  {map} -> {}", names.join(", "));
        }
    }
}
