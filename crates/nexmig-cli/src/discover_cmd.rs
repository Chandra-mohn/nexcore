//! `cobol2rust discover` -- auto-match data files to copybooks.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use miette::{IntoDiagnostic, Result};

use cobol_data::discovery::{
    self, CopybookInfo, DataFileInfo, ProgramInfo, FileDeclaration,
};
use cobol_data::layout;
use cobol_transpiler::parser;

use crate::Cli;

/// Arguments for `cobol2rust discover`.
#[derive(Debug, Args)]
pub struct DiscoverArgs {
    /// Directory containing binary data files.
    pub data_dir: PathBuf,

    /// Directory containing copybook files.
    #[arg(long)]
    pub copybooks: PathBuf,

    /// Directory containing program files (optional, for program-link matching).
    #[arg(long)]
    pub programs: Option<PathBuf>,

    /// Output format.
    #[arg(long, default_value = "table")]
    pub format: DiscoverFormat,

    /// File extensions to treat as data files.
    #[arg(long, default_value = "dat,bin,data")]
    pub data_extensions: String,

    /// File extensions to treat as copybooks.
    #[arg(long, default_value = "cpy,cob,copy")]
    pub copybook_extensions: String,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DiscoverFormat {
    Table,
    Json,
}

pub fn run(_cli: &Cli, args: &DiscoverArgs) -> Result<ExitCode> {
    let data_exts: Vec<&str> = args.data_extensions.split(',').collect();
    let cb_exts: Vec<&str> = args.copybook_extensions.split(',').collect();

    // Scan data files
    let dat_files = scan_data_files(&args.data_dir, &data_exts)?;
    eprintln!("Found {} data files", dat_files.len());

    // Parse copybooks
    let copybooks = parse_copybooks(&args.copybooks, &cb_exts)?;
    eprintln!("Parsed {} copybooks ({} with record length)",
        copybooks.len(),
        copybooks.iter().filter(|c| c.record_length.is_some()).count()
    );

    // Parse programs (optional)
    let programs = if let Some(ref prog_dir) = args.programs {
        parse_programs(prog_dir)?
    } else {
        vec![]
    };
    if !programs.is_empty() {
        eprintln!("Parsed {} programs", programs.len());
    }

    // Run matching
    let prog_matches = discovery::match_by_program(&programs, &copybooks, &dat_files);
    let len_matches = discovery::match_by_length(&copybooks, &dat_files);
    let report = discovery::merge_results(prog_matches, len_matches);

    // Output
    match args.format {
        DiscoverFormat::Json => {
            let json = serde_json::to_string_pretty(&report).into_diagnostic()?;
            println!("{json}");
        }
        DiscoverFormat::Table => {
            if report.results.is_empty() {
                println!("No matches found.");
            } else {
                for result in &report.results {
                    println!("{} ({} bytes)", result.data_file, result.file_size);
                    for m in &result.matches {
                        let prog = m
                            .program_path
                            .as_deref()
                            .unwrap_or("-");
                        let rec_len = m
                            .record_length
                            .map(|l| l.to_string())
                            .unwrap_or_else(|| "?".to_string());
                        let signals: Vec<&str> = m.signals.iter().map(|s| match s {
                            discovery::MatchSignal::ProgramLink => "program-link",
                            discovery::MatchSignal::RecordLengthMatch => "record-length",
                            discovery::MatchSignal::TrialDecode { .. } => "trial-decode",
                            discovery::MatchSignal::NameSimilarity { .. } => "name-similarity",
                        }).collect();
                        println!(
                            "  -> {} (rec_len={}, confidence={:?}, signals=[{}], via={})",
                            m.copybook_path, rec_len, m.confidence, signals.join(","), prog
                        );
                    }
                }
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn scan_data_files(dir: &PathBuf, extensions: &[&str]) -> Result<Vec<DataFileInfo>> {
    let mut files = Vec::new();
    if !dir.is_dir() {
        return Ok(files);
    }
    for entry in fs::read_dir(dir).into_diagnostic()? {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();
        if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            if extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_uppercase();
                let size = fs::metadata(&path).into_diagnostic()?.len();
                files.push(DataFileInfo {
                    path: path.display().to_string(),
                    stem,
                    size,
                });
            }
        }
    }
    Ok(files)
}

fn parse_copybooks(dir: &PathBuf, extensions: &[&str]) -> Result<Vec<CopybookInfo>> {
    let mut copybooks = Vec::new();
    if !dir.is_dir() {
        return Ok(copybooks);
    }
    for entry in fs::read_dir(dir).into_diagnostic()? {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();
        if path.is_file() {
            let ext = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            if extensions.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_uppercase();
                let source = match std::fs::read(&path) {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                    Err(e) => {
                        eprintln!("[WARN] Cannot read copybook {}: {e}", path.display());
                        continue;
                    }
                };
                let record_length = match parser::parse_data_division(&source) {
                    Ok(mut entries) => {
                        for e in &mut entries {
                            layout::compute_layout(e);
                        }
                        layout::record_length(&entries)
                    }
                    Err(_) => None,
                };
                copybooks.push(CopybookInfo {
                    path: path.display().to_string(),
                    stem,
                    record_length,
                });
            }
        }
    }
    Ok(copybooks)
}

fn parse_programs(dir: &PathBuf) -> Result<Vec<ProgramInfo>> {
    let mut programs = Vec::new();
    if !dir.is_dir() {
        return Ok(programs);
    }
    for entry in fs::read_dir(dir).into_diagnostic()? {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();
        if path.is_file() {
            let source = match std::fs::read(&path) {
                Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                Err(e) => {
                    eprintln!("[WARN] Cannot read program {}: {e}", path.display());
                    continue;
                }
            };
            let copy_targets = parser::extract_copy_targets(&source);
            // Extract file declarations from parsed program
            let declarations = match parser::parse_cobol(&source) {
                Ok(program) => {
                    if let Some(ref dd) = program.data_division {
                        dd.file_section
                            .iter()
                            .map(|fd| FileDeclaration {
                                logical_name: fd.file_name.clone(),
                                physical_name: fd.assign_to.clone().unwrap_or_default(),
                                recording_mode: None,
                                record_size: None,
                            })
                            .collect()
                    } else {
                        vec![]
                    }
                }
                Err(e) => {
                    eprintln!("[WARN] Parse failed for {}: {e}", path.display());
                    vec![]
                }
            };
            programs.push(ProgramInfo {
                path: path.display().to_string(),
                declarations,
                copy_targets,
            });
        }
    }
    Ok(programs)
}
