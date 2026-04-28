//! COBOL parser -- ANTLR4 orchestration and public API.
//!
//! Provides the entry point for parsing COBOL source into a typed AST.
//! Uses the ANTLR4-generated lexer/parser with custom listener walks
//! to build the AST incrementally.
//!
//! Each listener walk function creates its own parser instance (following
//! the coqu-di pattern) because the ANTLR4 parse tree has lifetime ties
//! to the parser/token stream.

pub mod bms_parser;
pub mod bms_references;
pub mod copy_expand;
pub mod copybook;
pub(crate) mod data_listener;
pub mod error_collector;
pub(crate) mod file_listener;
pub mod hierarchy;
pub mod pic_parser;
pub mod preprocess;
pub(crate) mod proc_listener;
pub mod sql_extract;

use crate::ast::{CobolProgram, DataDivision, DataEntry, ExecSqlStatement, FileDescription, Paragraph, ProcedureDivision, Section, SqlStatementType, Statement};
use crate::diagnostics::TranspileDiagnostic;
use rayon::prelude::*;
use crate::error::{Result, TranspileError};
use crate::generated::cobol85lexer::Cobol85Lexer;
use crate::generated::cobol85listener::Cobol85Listener;
use crate::generated::cobol85parser::{Cobol85Parser, Cobol85ParserContextType, Cobol85TreeWalker};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser::Parser as _;
use antlr_rust::tree::ParseTreeListener;

use error_collector::{ErrorCollector, ErrorStore, TokenError};

use data_listener::DataDivisionListener;
use file_listener::FileListener;
use hierarchy::{build_hierarchy, compute_layout};
use preprocess::{preprocess, preprocess_with_format, ExtractedExecBlock};
pub use preprocess::SourceFormat;
use proc_listener::ProcedureDivisionListener;
use sql_extract::analyze_exec_sql;

/// Result of parsing a complete COBOL program.
///
/// Contains the AST, diagnostics for unhandled statements/parse issues,
/// and token-level errors from the ANTLR lexer/parser.
#[derive(Debug)]
pub struct ParseResult {
    pub program: CobolProgram,
    pub diagnostics: Vec<TranspileDiagnostic>,
    pub token_errors: Vec<TokenError>,
}

/// Parse COBOL source into a typed AST.
///
/// Automatically detects fixed vs free format, preprocesses the source,
/// and then runs the ANTLR4 lexer/parser with listener walks.
///
/// # Errors
///
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails,
/// or `TranspileError::Preprocess` if preprocessing fails.
/// Internal parse engine. All public entry points delegate here.
///
/// Uses isolated per-division parsing with chunked + parallel ANTLR passes.
/// Each division is extracted and parsed in its own ANTLR instance(s),
/// preventing one malformed division from blocking the others.
///
/// `format`: `Some(fmt)` to force a source format, `None` for auto-detect.
fn parse_cobol_internal(source: &str, format: Option<SourceFormat>) -> Result<ParseResult> {
    let preprocessed = match format {
        Some(fmt) => preprocess_with_format(source, Some(fmt))?,
        None => preprocess(source)?,
    };

    let exec_blocks = drain_exec_blocks();
    let input = wrap_if_copybook(&preprocessed);

    let (working_storage, file_section) = parse_data_resilient(&input);
    let (mut procedure_division, diagnostics, token_errors) = parse_proc_resilient(&input);

    let program_id = extract_program_id(&input);
    let exec_sql_statements = build_exec_sql_ast(&exec_blocks);

    if let Some(ref mut proc_div) = procedure_division {
        let mut block_iter = exec_sql_statements
            .iter()
            .filter(|s| !matches!(s.sql_type, SqlStatementType::IncludeSqlca))
            .peekable();
        inject_exec_sql_into_proc_div(proc_div, &mut block_iter);
    }

    Ok(ParseResult {
        program: CobolProgram {
            program_id,
            data_division: Some(DataDivision {
                working_storage,
                file_section,
                ..Default::default()
            }),
            procedure_division,
            source_path: None,
            exec_sql_statements,
        },
        diagnostics,
        token_errors,
    })
}

/// Parse COBOL source into a typed AST.
///
/// Automatically detects fixed vs free format, preprocesses the source,
/// and runs the ANTLR4 parser with chunked + parallel listener walks.
///
/// # Errors
///
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails,
/// or `TranspileError::Preprocess` if preprocessing fails.
pub fn parse_cobol(source: &str) -> Result<CobolProgram> {
    parse_cobol_internal(source, None).map(|r| r.program)
}

/// Parse COBOL source into a typed AST with explicit format, returning
/// diagnostics and token errors.
///
/// Use this when the source format was detected separately (e.g., on the
/// original source before COPY expansion, since copybook content can fool
/// auto-detection).
///
/// # Errors
///
/// Returns `TranspileError::Preprocess` if source preprocessing fails.
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails.
pub fn parse_cobol_with_format(
    source: &str,
    format: SourceFormat,
) -> Result<(CobolProgram, Vec<TranspileDiagnostic>, Vec<TokenError>)> {
    let r = parse_cobol_internal(source, Some(format))?;
    Ok((r.program, r.diagnostics, r.token_errors))
}

/// Parse COBOL source into a typed AST, also returning diagnostics.
///
/// Uses auto-detected format. For explicit format control, use
/// `parse_cobol_with_format` instead.
///
/// # Errors
///
/// Returns `TranspileError::Preprocess` if source preprocessing fails.
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails.
pub fn parse_cobol_with_diagnostics(source: &str) -> Result<(CobolProgram, Vec<TranspileDiagnostic>)> {
    let r = parse_cobol_internal(source, None)?;
    Ok((r.program, r.diagnostics))
}

/// Parse PROCEDURE DIVISION using isolated parsing.
///
/// Extracts just the PROCEDURE DIVISION text and parses it independently.
fn parse_proc_resilient(input: &str) -> (Option<ProcedureDivision>, Vec<TranspileDiagnostic>, Vec<TokenError>) {
    match parse_proc_division_isolated(input) {
        Ok((pd, diags, token_errors)) => (pd, diags, token_errors),
        Err(e) => {
            tracing::warn!(error = %e, "Isolated PROCEDURE DIVISION parse failed");
            (None, Vec::new(), Vec::new())
        }
    }
}

/// Parse DATA DIVISION and FILE SECTION using isolated per-division parsing.
///
/// Each division is extracted from the source and parsed independently with
/// a minimal program wrapper. This prevents a malformed section from poisoning
/// the ANTLR parser state and blocking other sections from parsing.
fn parse_data_resilient(input: &str) -> (Vec<DataEntry>, Vec<FileDescription>) {
    let working_storage = parse_data_division_isolated(input);
    let file_section = parse_file_section_isolated(input);
    (working_storage, file_section)
}

/// Drain all extracted EXEC blocks from the thread-local storage.
fn drain_exec_blocks() -> Vec<ExtractedExecBlock> {
    let mut blocks = Vec::new();
    while let Some(block) = preprocess::take_next_exec_block() {
        blocks.push(block);
    }
    blocks
}

/// Convert extracted EXEC blocks into AST nodes.
fn build_exec_sql_ast(blocks: &[ExtractedExecBlock]) -> Vec<ExecSqlStatement> {
    blocks
        .iter()
        .filter(|b| b.exec_type == "SQL")
        .map(|b| analyze_exec_sql(&b.text))
        .collect()
}

/// Parse COBOL source and extract DATA DIVISION into a hierarchical tree.
///
/// Runs the ANTLR4 parser with `DataDivisionListener`, then builds
/// the hierarchy from flat items and computes byte layouts.
///
/// # Errors
///
/// Returns `TranspileError::AntlrError` if the ANTLR4 parser fails.
pub fn parse_data_division(source: &str) -> Result<Vec<DataEntry>> {
    let preprocessed = preprocess(source)?;
    let input = wrap_if_copybook(&preprocessed);
    parse_data_division_preprocessed(&input)
}

/// Parse DATA DIVISION from already-preprocessed source.
/// Avoids double-preprocessing when called from parse_cobol().
fn parse_data_division_preprocessed(source: &str) -> Result<Vec<DataEntry>> {
    let listener = run_data_listener(source)?;
    for d in &listener.diagnostics {
        tracing::warn!(severity = %d.severity, category = %d.category, "{}", d.message);
    }
    let mut records = build_hierarchy(listener.items);

    // Compute byte offsets for all records
    for record in &mut records {
        compute_layout(record);
    }

    Ok(records)
}

/// Extract raw DATA DIVISION text from source (without program wrapper).
///
/// Includes FILE SECTION records (needed so FD record fields appear in the
/// data listener output), WORKING-STORAGE, LOCAL-STORAGE, and LINKAGE.
/// Starts from whichever comes first: FILE SECTION, WORKING-STORAGE, or
/// LINKAGE SECTION. Ends at PROCEDURE DIVISION (or end of source).
fn extract_data_division_text(source: &str) -> Option<String> {
    let upper = source.to_uppercase();

    // Find the earliest data section start
    let candidates = [
        upper.find("FILE SECTION"),
        upper.find("WORKING-STORAGE SECTION"),
        upper.find("LINKAGE SECTION"),
    ];
    let start_pos = candidates.iter().filter_map(|p| *p).min()?;

    // Find the start of the line containing the section header.
    let line_start = source[..start_pos].rfind('\n').map_or(0, |p| p + 1);

    // Find the end: PROCEDURE DIVISION or end of source.
    let end_pos = upper.find("PROCEDURE DIVISION").unwrap_or(source.len());

    Some(source[line_start..end_pos].to_string())
}

/// Split DATA DIVISION text into chunks at 01/77-level boundaries.
///
/// Each chunk contains up to `batch_size` top-level records (01 or 77-level
/// entries) with all their subordinate items (levels 02-49, 66, 88).
/// Section headers (WORKING-STORAGE, LINKAGE, LOCAL-STORAGE) force a new chunk.
fn split_data_division_into_chunks(data_text: &str, batch_size: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut top_level_count: usize = 0;

    for line in data_text.lines() {
        let trimmed = line.trim_start();
        let upper = trimmed.to_uppercase();

        // Section headers always force a new chunk
        if upper.starts_with("WORKING-STORAGE SECTION")
            || upper.starts_with("LINKAGE SECTION")
            || upper.starts_with("LOCAL-STORAGE SECTION")
            || upper.starts_with("FILE SECTION")
        {
            if !current_chunk.trim().is_empty() {
                chunks.push(std::mem::take(&mut current_chunk));
            }
            top_level_count = 0;
            current_chunk.push_str(line);
            current_chunk.push('\n');
            continue;
        }

        // 01 or 77 level starts a new top-level record
        let is_top_level = trimmed.starts_with("01 ") || trimmed.starts_with("01\t")
            || trimmed.starts_with("77 ") || trimmed.starts_with("77\t");

        if is_top_level {
            if top_level_count >= batch_size && !current_chunk.trim().is_empty() {
                chunks.push(std::mem::take(&mut current_chunk));
                top_level_count = 0;
            }
            top_level_count += 1;
        }

        current_chunk.push_str(line);
        current_chunk.push('\n');
    }

    // Flush remaining
    if !current_chunk.trim().is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

/// Wrap a DATA DIVISION text chunk in a minimal COBOL program skeleton
/// so ANTLR4 can parse it via `startRule`.
fn wrap_chunk_as_program(chunk: &str) -> String {
    let upper = chunk.trim_start().to_uppercase();
    let needs_section = !upper.starts_with("WORKING-STORAGE SECTION")
        && !upper.starts_with("LINKAGE SECTION")
        && !upper.starts_with("LOCAL-STORAGE SECTION")
        && !upper.starts_with("FILE SECTION");

    let section_prefix = if needs_section {
        "WORKING-STORAGE SECTION.\n"
    } else {
        ""
    };

    format!(
        "IDENTIFICATION DIVISION.\nPROGRAM-ID. CHUNK.\nDATA DIVISION.\n{section_prefix}{chunk}\nPROCEDURE DIVISION.\n    STOP RUN.\n"
    )
}

/// Parse DATA DIVISION by splitting into chunks at 01-level boundaries.
///
/// Each chunk is wrapped in a program skeleton and parsed independently.
/// Returns merged flat items (before hierarchy building).
fn parse_data_chunks(data_text: &str, batch_size: usize) -> Vec<DataEntry> {
    let chunks = split_data_division_into_chunks(data_text, batch_size);
    let total_chunks = chunks.len();

    tracing::info!(chunks = total_chunks, "DATA DIVISION chunked parsing starting");

    // Parse each chunk in parallel
    let results: Vec<_> = chunks
        .into_par_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let wrapped = wrap_chunk_as_program(&chunk);
            match run_data_listener(&wrapped) {
                Ok(listener) => {
                    for d in &listener.diagnostics {
                        tracing::warn!(severity = %d.severity, category = %d.category, "{}", d.message);
                    }
                    tracing::debug!(
                        chunk = i + 1,
                        total = total_chunks,
                        fields = listener.items.len(),
                        "data chunk parsed"
                    );
                    listener.items
                }
                Err(e) => {
                    tracing::warn!(chunk = i + 1, total = total_chunks, error = %e, "DATA DIVISION chunk parse failed");
                    Vec::new()
                }
            }
        })
        .collect();

    // Merge results in order
    let mut all_items: Vec<DataEntry> = Vec::new();
    let mut success_count = 0;
    for items in results {
        if !items.is_empty() {
            success_count += 1;
        }
        all_items.extend(items);
    }

    if total_chunks > 1 {
        tracing::info!(success = success_count, total = total_chunks, fields = all_items.len(), "DATA DIVISION chunked parsing complete");
    }

    all_items
}

/// Size threshold (in bytes) above which procedure division parsing uses
/// chunked + parallel mode instead of monolithic ANTLR parse.
const PROC_CHUNK_THRESHOLD: usize = 500_000; // ~500KB

/// Default number of COBOL sections per chunk when splitting the procedure
/// division for parallel parsing.
const PROC_SECTIONS_PER_CHUNK: usize = 50;

/// Split PROCEDURE DIVISION text at SECTION boundaries.
///
/// Groups `sections_per_chunk` consecutive sections into each chunk.
/// Lines before the first SECTION header (standalone paragraphs at the
/// top of the procedure division) go into the first chunk.
///
/// The PROCEDURE DIVISION header line itself is NOT included -- the caller
/// must prepend it when wrapping each chunk.
fn split_proc_division_into_chunks(proc_text: &str, sections_per_chunk: usize) -> Vec<String> {
    let mut chunks: Vec<String> = Vec::new();
    let mut current_chunk = String::new();
    let mut section_count: usize = 0;

    for line in proc_text.lines() {
        let trimmed = line.trim();
        let upper = trimmed.to_uppercase();

        // Detect SECTION headers: "SOME-NAME SECTION." or "SOME-NAME SECTION"
        // but NOT "PROCEDURE DIVISION" or "WORKING-STORAGE SECTION" etc.
        let is_section_header = (upper.ends_with("SECTION.")
            || upper.ends_with("SECTION"))
            && !upper.contains("DIVISION")
            && !upper.starts_with("WORKING-STORAGE")
            && !upper.starts_with("LINKAGE")
            && !upper.starts_with("LOCAL-STORAGE")
            && !upper.starts_with("FILE")
            && !upper.starts_with("INPUT-OUTPUT")
            && !upper.starts_with("CONFIGURATION");

        if is_section_header {
            if section_count >= sections_per_chunk && !current_chunk.trim().is_empty() {
                chunks.push(std::mem::take(&mut current_chunk));
                section_count = 0;
            }
            section_count += 1;
        }

        current_chunk.push_str(line);
        current_chunk.push('\n');
    }

    // Flush remaining
    if !current_chunk.trim().is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

/// Wrap a PROCEDURE DIVISION chunk in a minimal COBOL program skeleton
/// so ANTLR4 can parse it as a complete program.
fn wrap_proc_chunk_as_program(chunk: &str) -> String {
    format!(
        "IDENTIFICATION DIVISION.\nPROGRAM-ID. CHUNK.\nDATA DIVISION.\nWORKING-STORAGE SECTION.\n01 FILLER PIC X.\nPROCEDURE DIVISION.\n{chunk}"
    )
}

/// Parse PROCEDURE DIVISION by splitting into chunks at SECTION boundaries
/// and parsing each chunk in parallel with rayon.
///
/// Returns merged (sections, standalone_paragraphs, diagnostics, token_errors).
fn parse_proc_chunks(
    proc_text: &str,
    sections_per_chunk: usize,
) -> (Vec<Section>, Vec<Paragraph>, Vec<TranspileDiagnostic>, Vec<TokenError>) {
    let chunks = split_proc_division_into_chunks(proc_text, sections_per_chunk);
    let total_chunks = chunks.len();

    tracing::info!(chunks = total_chunks, "PROCEDURE DIVISION chunked parsing starting");

    // Parse each chunk in parallel
    let results: Vec<_> = chunks
        .into_par_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let wrapped = wrap_proc_chunk_as_program(&chunk);
            match run_proc_listener_with_errors(&wrapped) {
                Ok((listener, token_errors)) => {
                    // Drain thread-local diagnostics collected during statement extraction
                    let diags = proc_listener::drain_diagnostics();
                    tracing::debug!(
                        chunk = i + 1,
                        total = total_chunks,
                        sections = listener.sections.len(),
                        paragraphs = listener.paragraphs.len(),
                        "proc chunk parsed"
                    );
                    (listener.sections, listener.paragraphs, diags, token_errors)
                }
                Err(e) => {
                    tracing::warn!(
                        chunk = i + 1,
                        total = total_chunks,
                        error = %e,
                        "PROCEDURE DIVISION chunk parse failed"
                    );
                    (Vec::new(), Vec::new(), Vec::new(), Vec::new())
                }
            }
        })
        .collect();

    // Merge results in order
    let mut all_sections = Vec::new();
    let mut all_paragraphs = Vec::new();
    let mut all_diagnostics = Vec::new();
    let mut all_token_errors = Vec::new();
    let mut success_count = 0;

    for (sections, paragraphs, diags, errors) in results {
        if !sections.is_empty() || !paragraphs.is_empty() {
            success_count += 1;
        }
        all_sections.extend(sections);
        all_paragraphs.extend(paragraphs);
        all_diagnostics.extend(diags);
        all_token_errors.extend(errors);
    }

    tracing::info!(
        success = success_count,
        total = total_chunks,
        sections = all_sections.len(),
        paragraphs = all_paragraphs.len(),
        "PROCEDURE DIVISION chunked parsing complete"
    );

    (all_sections, all_paragraphs, all_diagnostics, all_token_errors)
}

/// Extract FILE SECTION text (from FILE SECTION through WORKING-STORAGE or
/// LINKAGE or PROCEDURE DIVISION) for isolated parsing.
///
/// Also includes the INPUT-OUTPUT SECTION / FILE-CONTROL from the ENVIRONMENT
/// DIVISION, since the file listener needs SELECT statements to map FDs to files.
fn extract_file_section_source(source: &str) -> Option<String> {
    let upper = source.to_uppercase();
    let file_pos = upper.find("FILE SECTION")?;
    let file_line_start = source[..file_pos].rfind('\n').map_or(0, |p| p + 1);

    // End at WORKING-STORAGE, LINKAGE, or PROCEDURE DIVISION
    let end_candidates = [
        upper.find("WORKING-STORAGE SECTION"),
        upper.find("LINKAGE SECTION"),
        upper.find("PROCEDURE DIVISION"),
    ];
    let end_pos = end_candidates.iter().filter_map(|p| *p).min().unwrap_or(source.len());
    let file_text = &source[file_line_start..end_pos];

    // Extract FILE-CONTROL: from INPUT-OUTPUT SECTION up to DATA DIVISION.
    let io_text = if let Some(io_pos) = upper.find("INPUT-OUTPUT SECTION") {
        let io_line_start = source[..io_pos].rfind('\n').map_or(0, |n| n + 1);
        let data_pos = upper.find("DATA DIVISION").unwrap_or(file_line_start);
        let data_line_start = source[..data_pos].rfind('\n').map_or(data_pos, |n| n + 1);
        &source[io_line_start..data_line_start]
    } else {
        ""
    };

    Some(format!(
        "IDENTIFICATION DIVISION.\nPROGRAM-ID. ISOLATED-FILE.\nENVIRONMENT DIVISION.\n{io_text}DATA DIVISION.\n{file_text}\nWORKING-STORAGE SECTION.\n01 FILLER PIC X.\nPROCEDURE DIVISION.\n    STOP RUN.\n"
    ))
}

/// Parse DATA DIVISION in isolation using chunked parsing.
///
/// Extracts DATA DIVISION text, splits at 01-level boundaries into chunks
/// of ~100 top-level records each, parses each chunk independently, and
/// merges results. This handles monster files (100K+ line DATA DIVISIONs)
/// where ANTLR4 silently truncates output on very large inputs.
fn parse_data_division_isolated(source: &str) -> Vec<DataEntry> {
    let data_text = match extract_data_division_text(source) {
        Some(s) => s,
        None => return Vec::new(),
    };

    let flat_items = parse_data_chunks(&data_text, 100);
    if flat_items.is_empty() {
        return Vec::new();
    }

    let mut records = build_hierarchy(flat_items);
    for record in &mut records {
        compute_layout(record);
    }
    records
}

/// Attempt to parse FILE SECTION in isolation when the full-source parse
/// produces 0 file descriptions.
fn parse_file_section_isolated(source: &str) -> Vec<FileDescription> {
    let isolated = match extract_file_section_source(source) {
        Some(s) => s,
        None => return Vec::new(),
    };

    match run_file_listener(&isolated) {
        Ok(listener) => listener.into_file_descriptions(),
        Err(e) => {
            tracing::warn!(error = %e, "Isolated FILE SECTION parse failed");
            Vec::new()
        }
    }
}

/// Extract raw PROCEDURE DIVISION text from source (without wrapping).
///
/// Returns the text starting from the PROCEDURE DIVISION header line
/// through end of source. Returns `None` if no PROCEDURE DIVISION found.
fn extract_proc_division_text(source: &str) -> Option<String> {
    let upper = source.to_uppercase();
    let proc_pos = upper.find("PROCEDURE DIVISION")?;
    let line_start = source[..proc_pos].rfind('\n').map_or(0, |p| p + 1);
    let proc_text = &source[line_start..];

    tracing::debug!(
        source_lines = source.lines().count(),
        proc_offset = proc_pos,
        proc_lines = proc_text.lines().count(),
        "extracted PROCEDURE DIVISION text"
    );

    Some(proc_text.to_string())
}

/// Wrap raw PROCEDURE DIVISION text in a minimal COBOL program skeleton
/// so ANTLR4 can parse it as a complete program.
fn wrap_proc_as_program(proc_text: &str) -> String {
    format!(
        "IDENTIFICATION DIVISION.\nPROGRAM-ID. ISOLATED-PROC.\nDATA DIVISION.\nWORKING-STORAGE SECTION.\n01 FILLER PIC X.\n{proc_text}"
    )
}

/// Attempt to parse PROCEDURE DIVISION in isolation.
///
/// For large procedure divisions (>{threshold} bytes), splits at SECTION
/// boundaries and parses each chunk in parallel with rayon. For smaller
/// inputs, uses a single monolithic ANTLR pass.
fn parse_proc_division_isolated(
    source: &str,
) -> Result<(Option<ProcedureDivision>, Vec<TranspileDiagnostic>, Vec<TokenError>)> {
    let proc_text = match extract_proc_division_text(source) {
        Some(s) => s,
        None => return Ok((None, Vec::new(), Vec::new())),
    };

    let proc_bytes = proc_text.len();

    // Route to chunked or monolithic based on size
    let (sections, paragraphs, diagnostics, token_errors) = if proc_bytes > PROC_CHUNK_THRESHOLD {
        tracing::info!(
            bytes = proc_bytes,
            threshold = PROC_CHUNK_THRESHOLD,
            "large PROCEDURE DIVISION -- using chunked parallel parsing"
        );
        // Strip the PROCEDURE DIVISION header line for chunking --
        // the header is on the first line, each chunk gets its own via wrap_proc_chunk_as_program
        let body = proc_text
            .lines()
            .skip(1)
            .collect::<Vec<_>>()
            .join("\n");
        parse_proc_chunks(&body, PROC_SECTIONS_PER_CHUNK)
    } else {
        // Small enough for monolithic parse
        let wrapped = wrap_proc_as_program(&proc_text);
        let (listener, token_errors) = run_proc_listener_with_errors(&wrapped)?;
        let diags = proc_listener::drain_diagnostics();
        (listener.sections, listener.paragraphs, diags, token_errors)
    };

    tracing::info!(
        sections = sections.len(),
        paragraphs = paragraphs.len(),
        diagnostics = diagnostics.len(),
        token_errors = token_errors.len(),
        "PROCEDURE DIVISION parse complete"
    );

    if sections.is_empty() && paragraphs.is_empty() {
        return Ok((None, diagnostics, token_errors));
    }

    Ok((
        Some(ProcedureDivision {
            using_params: Vec::new(),
            returning: None,
            sections,
            paragraphs,
        }),
        diagnostics,
        token_errors,
    ))
}

/// Run ANTLR4 parse and walk with `ProcedureDivisionListener`, also collecting
/// token recognition errors from the lexer and parser.
fn run_proc_listener_with_errors(
    source: &str,
) -> Result<(ProcedureDivisionListener, Vec<TokenError>)> {
    let error_store = ErrorStore::new();

    let input: InputStream<&str> = InputStream::new(source);
    let mut lexer = Cobol85Lexer::new(input);
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(ErrorCollector::new(&error_store)));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = Cobol85Parser::new(token_stream);
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(ErrorCollector::new(&error_store)));

    let tree = parser.startRule().map_err(|e| TranspileError::AntlrError {
        message: format!("{e:?}"),
    })?;

    let listener = Box::new(ProcedureDivisionListener::new());
    let mut listener = *Cobol85TreeWalker::walk(listener, &*tree);

    // Drain diagnostics collected via thread-local during statement extraction
    listener.diagnostics = proc_listener::drain_diagnostics();

    Ok((listener, error_store.drain()))
}

/// Run ANTLR4 parse and walk with `DataDivisionListener`.
fn run_data_listener(source: &str) -> Result<DataDivisionListener> {
    let error_store = ErrorStore::new();

    let input: InputStream<&str> = InputStream::new(source);
    let mut lexer = Cobol85Lexer::new(input);
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(ErrorCollector::new(&error_store)));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = Cobol85Parser::new(token_stream);
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(ErrorCollector::new(&error_store)));

    let tree = parser.startRule().map_err(|e| TranspileError::AntlrError {
        message: format!("{e:?}"),
    })?;

    let listener = Box::new(DataDivisionListener::new());
    let listener = Cobol85TreeWalker::walk(listener, &*tree);

    Ok(*listener)
}

/// Run ANTLR4 parse and walk with `FileListener`.
fn run_file_listener(source: &str) -> Result<FileListener> {
    let error_store = ErrorStore::new();

    let input: InputStream<&str> = InputStream::new(source);
    let mut lexer = Cobol85Lexer::new(input);
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(ErrorCollector::new(&error_store)));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = Cobol85Parser::new(token_stream);
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(ErrorCollector::new(&error_store)));

    let tree = parser.startRule().map_err(|e| TranspileError::AntlrError {
        message: format!("{e:?}"),
    })?;

    let listener = Box::new(FileListener::new());
    let listener = Cobol85TreeWalker::walk(listener, &*tree);

    Ok(*listener)
}

/// Run ANTLR4 parse with a validation-only listener.
///
/// Verifies that the source is syntactically valid COBOL.
#[allow(dead_code)]
fn run_validation_walk(source: &str) -> Result<()> {
    let error_store = ErrorStore::new();

    let input: InputStream<&str> = InputStream::new(source);
    let mut lexer = Cobol85Lexer::new(input);
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(ErrorCollector::new(&error_store)));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = Cobol85Parser::new(token_stream);
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(ErrorCollector::new(&error_store)));

    let tree = parser.startRule().map_err(|e| TranspileError::AntlrError {
        message: format!("{e:?}"),
    })?;

    let listener = Box::new(ValidationListener);
    let _listener = Cobol85TreeWalker::walk(listener, &*tree);

    Ok(())
}

/// Placeholder listener that collects nothing (validates parse only).
#[derive(Debug, Default)]
struct ValidationListener;

impl ParseTreeListener<'_, Cobol85ParserContextType> for ValidationListener {}

#[allow(unused_variables)]
impl Cobol85Listener<'_> for ValidationListener {}

/// Wrap standalone copybook content in a minimal COBOL program skeleton
/// so the ANTLR4 `startRule` entry point can parse it.
fn wrap_if_copybook(source: &str) -> String {
    let upper = source.to_uppercase();
    if upper.contains("IDENTIFICATION DIVISION") || upper.contains("ID DIVISION") {
        return source.to_string();
    }

    let mut wrapped = String::with_capacity(source.len() + 200);
    wrapped.push_str("       IDENTIFICATION DIVISION.\n");
    wrapped.push_str("       PROGRAM-ID. WRAPPER.\n");
    wrapped.push_str("       DATA DIVISION.\n");
    wrapped.push_str("       WORKING-STORAGE SECTION.\n");
    wrapped.push_str(source);
    wrapped.push('\n');
    wrapped
}

/// Extract PROGRAM-ID from preprocessed source via simple text scan.
///
/// Falls back to "UNKNOWN" if not found.
pub fn extract_program_id(source: &str) -> String {
    for line in source.lines() {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("PROGRAM-ID") {
            let rest = trimmed
                .trim_start_matches("PROGRAM-ID")
                .trim_start_matches('.')
                .trim_start();
            // Take only the first token (the actual program name).
            // Stops at period, space, or end of string.
            let name: String = rest
                .chars()
                .take_while(|c| !c.is_whitespace() && *c != '.')
                .collect();
            if !name.is_empty() {
                return name;
            }
        }
    }
    "UNKNOWN".to_string()
}

/// Extract COPY statement targets from raw COBOL source.
///
/// Text-level scan (no ANTLR4 parsing) since COPY is a preprocessor
/// directive that should be resolved before parsing.
///
/// Returns uppercased copy-member names (e.g., `["ACCTFILE", "COMMON"]`).
pub fn extract_copy_targets(source: &str) -> Vec<String> {
    let mut targets = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("COPY ") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[1]
                    .trim_end_matches('.')
                    .trim_matches('\'')
                    .trim_matches('"');
                if !name.is_empty() {
                    targets.push(name.to_string());
                }
            }
        }
    }
    targets
}

/// Replace `Statement::Continue` placeholders with `Statement::ExecSql` nodes.
///
/// The preprocessor replaces each `EXEC SQL ... END-EXEC` in the PROCEDURE
/// DIVISION with `CONTINUE`. The extracted blocks are stored in order.
/// This function walks the procedure division statement tree and replaces
/// each `Continue` with the next available `ExecSqlStatement`, restoring
/// the 1:1 correspondence.
fn inject_exec_sql_into_proc_div<'a>(
    proc_div: &mut ProcedureDivision,
    blocks: &mut std::iter::Peekable<impl Iterator<Item = &'a ExecSqlStatement>>,
) {
    for para in &mut proc_div.paragraphs {
        for sentence in &mut para.sentences {
            inject_exec_sql_into_stmts(&mut sentence.statements, blocks);
        }
    }
    for section in &mut proc_div.sections {
        for para in &mut section.paragraphs {
            for sentence in &mut para.sentences {
                inject_exec_sql_into_stmts(&mut sentence.statements, blocks);
            }
        }
    }
}

/// Recursively replace `Continue` with `ExecSql` in a statement list.
fn inject_exec_sql_into_stmts<'a>(
    stmts: &mut Vec<Statement>,
    blocks: &mut std::iter::Peekable<impl Iterator<Item = &'a ExecSqlStatement>>,
) {
    for stmt in stmts.iter_mut() {
        match stmt {
            Statement::Continue => {
                if blocks.peek().is_some() {
                    let exec = blocks.next().expect("peek confirmed element exists").clone();
                    *stmt = Statement::ExecSql(exec);
                }
            }
            Statement::If(if_stmt) => {
                inject_exec_sql_into_stmts(&mut if_stmt.then_body, blocks);
                inject_exec_sql_into_stmts(&mut if_stmt.else_body, blocks);
            }
            Statement::Evaluate(eval) => {
                for branch in &mut eval.when_branches {
                    inject_exec_sql_into_stmts(&mut branch.body, blocks);
                }
                inject_exec_sql_into_stmts(&mut eval.when_other, blocks);
            }
            Statement::Perform(perf) => {
                inject_exec_sql_into_stmts(&mut perf.body, blocks);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_program_id_basic() {
        let source = "\
       IDENTIFICATION DIVISION.
       PROGRAM-ID. HELLO.
       DATA DIVISION.";
        assert_eq!(extract_program_id(source), "HELLO");
    }

    #[test]
    fn extract_program_id_missing() {
        let source = "\
       IDENTIFICATION DIVISION.
       DATA DIVISION.";
        assert_eq!(extract_program_id(source), "UNKNOWN");
    }

    #[test]
    fn wrap_if_copybook_full_program() {
        let source = "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. TEST.";
        let result = wrap_if_copybook(source);
        assert!(!result.contains("WRAPPER"));
    }

    #[test]
    fn wrap_if_copybook_copybook() {
        let source = "       01  WS-FIELD PIC X(10).";
        let result = wrap_if_copybook(source);
        assert!(result.contains("IDENTIFICATION DIVISION"));
        assert!(result.contains("WRAPPER"));
        assert!(result.contains("WS-FIELD"));
    }

    #[test]
    fn extract_copy_targets_basic() {
        let source = "       FD  ACCT-FILE.\n           COPY ACCTFILE.\n";
        let targets = extract_copy_targets(source);
        assert_eq!(targets, vec!["ACCTFILE"]);
    }

    #[test]
    fn extract_copy_targets_none() {
        let source = "       01  WS-FIELD PIC X(10).";
        let targets = extract_copy_targets(source);
        assert!(targets.is_empty());
    }

    #[test]
    fn parse_cobol_minimal_program() {
        // Free-format COBOL source (no column restrictions)
        let source = "IDENTIFICATION DIVISION.\nPROGRAM-ID. HELLO.\nDATA DIVISION.\nWORKING-STORAGE SECTION.\n01  WS-NAME PIC X(20).\nPROCEDURE DIVISION.\n    DISPLAY 'HELLO WORLD'.\n    STOP RUN.";

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse_cobol failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "HELLO");
        assert!(program.data_division.is_some());
    }

    #[test]
    fn parse_procedure_division_statements() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TESTSTMTS.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-A PIC 9(5) VALUE 10.\n",
            "01  WS-B PIC 9(5) VALUE 20.\n",
            "01  WS-C PIC 9(5).\n",
            "01  WS-NAME PIC X(20).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    MOVE WS-A TO WS-C.\n",
            "    ADD WS-A TO WS-B.\n",
            "    DISPLAY 'RESULT: ' WS-B.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();

        let proc_div = program.procedure_division.as_ref()
            .expect("should have procedure division");

        // Should have at least one paragraph
        assert!(!proc_div.paragraphs.is_empty(), "should have paragraphs");
        let para = &proc_div.paragraphs[0];
        assert_eq!(para.name, "MAIN-PARA");

        // Collect all statements across sentences
        let stmts: Vec<&crate::ast::Statement> = para.sentences.iter()
            .flat_map(|s| &s.statements)
            .collect();
        assert!(stmts.len() >= 3, "should have at least 3 statements, got {}", stmts.len());

        // Verify MOVE
        assert!(matches!(stmts[0], crate::ast::Statement::Move(_)));
        // Verify ADD
        assert!(matches!(stmts[1], crate::ast::Statement::Add(_)));
        // Verify DISPLAY
        assert!(matches!(stmts[2], crate::ast::Statement::Display(_)));
    }

    #[test]
    fn parse_if_statement() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TESTIF.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-X PIC 9(3) VALUE 5.\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    IF WS-X > 0\n",
            "        DISPLAY 'POSITIVE'\n",
            "    ELSE\n",
            "        DISPLAY 'NOT POSITIVE'\n",
            "    END-IF.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();
        let proc_div = program.procedure_division.as_ref()
            .expect("should have procedure division");

        let stmts: Vec<&crate::ast::Statement> = proc_div.paragraphs[0].sentences.iter()
            .flat_map(|s| &s.statements)
            .collect();

        // First statement should be IF
        match &stmts[0] {
            crate::ast::Statement::If(if_stmt) => {
                assert!(!if_stmt.then_body.is_empty(), "IF should have then body");
                assert!(!if_stmt.else_body.is_empty(), "IF should have else body");
            }
            other => panic!("expected If, got {other:?}"),
        }
    }

    #[test]
    fn parse_perform_procedure() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. TESTPERF.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-COUNT PIC 9(3).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    PERFORM WORK-PARA.\n",
            "    STOP RUN.\n",
            "WORK-PARA.\n",
            "    DISPLAY 'WORKING'.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();
        let proc_div = program.procedure_division.as_ref()
            .expect("should have procedure division");

        assert!(proc_div.paragraphs.len() >= 2,
            "should have at least 2 paragraphs, got {}", proc_div.paragraphs.len());

        let main_stmts: Vec<&crate::ast::Statement> = proc_div.paragraphs[0].sentences.iter()
            .flat_map(|s| &s.statements)
            .collect();

        // First statement should be PERFORM
        match &main_stmts[0] {
            crate::ast::Statement::Perform(perf) => {
                assert!(perf.target.is_some(), "should have target paragraph");
                let target = perf.target.as_ref().unwrap();
                assert_eq!(target.name, "WORK-PARA");
            }
            other => panic!("expected Perform, got {other:?}"),
        }
    }

    #[test]
    fn parse_cobol_fixed_format() {
        // Fixed-format COBOL source with sequence numbers and indicators
        let source = concat!(
            "000100 IDENTIFICATION DIVISION.                                         \n",
            "000200 PROGRAM-ID. TESTPG.                                              \n",
            "000300 DATA DIVISION.                                                    \n",
            "000400 WORKING-STORAGE SECTION.                                          \n",
            "000500 01  WS-COUNTER PIC 9(5).                                         \n",
            "000600 PROCEDURE DIVISION.                                               \n",
            "000700     DISPLAY 'HELLO'.                                              \n",
            "000800     STOP RUN.                                                     \n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse_cobol fixed-format failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "TESTPG");
    }

    #[test]
    fn parse_pre_expanded_source() {
        // Source that has already been preprocessed (free format, no columns)
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. EXPANDED.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-FLD PIC X(10).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    DISPLAY WS-FLD.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse_cobol failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "EXPANDED");
        assert!(program.data_division.is_some());
        assert!(program.procedure_division.is_some());
    }

    #[test]
    fn parse_large_working_storage() {
        // Test that programs with many WS fields still parse the proc division
        let mut source = String::from(
            "IDENTIFICATION DIVISION.\n\
             PROGRAM-ID. LARGE-WS-TEST.\n\
             DATA DIVISION.\n\
             WORKING-STORAGE SECTION.\n"
        );
        // Add 600+ WS fields to simulate large_working_storage.cbl
        for i in 1..=300 {
            source.push_str(&format!("01  WS-ALPHA-{i:03} PIC X(20).\n"));
        }
        for i in 1..=300 {
            source.push_str(&format!("01  WS-NUM-{i:03}   PIC 9(09).\n"));
        }
        source.push_str(
            "PROCEDURE DIVISION.\n\
             MAIN-PARAGRAPH.\n\
                 MOVE \"HELLO\" TO WS-ALPHA-001.\n\
                 DISPLAY WS-ALPHA-001.\n\
                 STOP RUN.\n"
        );

        let result = parse_cobol(&source);
        assert!(result.is_ok(), "parse_cobol failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "LARGE-WS-TEST");
        assert!(
            program.procedure_division.is_some(),
            "procedure_division should NOT be None for programs with PROCEDURE DIVISION"
        );
        let pd = program.procedure_division.unwrap();
        assert!(
            !pd.paragraphs.is_empty(),
            "Expected at least 1 paragraph but got 0"
        );
    }

    #[test]
    fn parse_large_ws_from_file() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap().parent().unwrap()
            .join("cobol/volume/large_working_storage.cbl");
        if !path.exists() {
            return;
        }
        let source = std::fs::read_to_string(&path).unwrap();
        let result = parse_cobol(&source);
        assert!(result.is_ok(), "parse_cobol failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "LARGE-WS-TEST");
        assert!(
            program.procedure_division.is_some(),
            "PROCEDURE DIVISION was not parsed from large_working_storage.cbl"
        );
        let pd = program.procedure_division.unwrap();
        assert!(
            !pd.paragraphs.is_empty(),
            "Expected at least 1 paragraph but got 0"
        );
    }

    #[test]
    fn parse_exec_sql_select_into() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. SQLTEST.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-EMPNO  PIC 9(6).\n",
            "01  WS-ENAME  PIC X(20).\n",
            "01  WS-SAL    PIC 9(7)V99.\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    MOVE 100 TO WS-EMPNO.\n",
            "    EXEC SQL\n",
            "        SELECT ENAME, SAL\n",
            "        INTO :WS-ENAME, :WS-SAL\n",
            "        FROM EMP\n",
            "        WHERE EMPNO = :WS-EMPNO\n",
            "    END-EXEC.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();
        assert_eq!(program.program_id, "SQLTEST");

        // Should have extracted 1 EXEC SQL statement
        assert_eq!(program.exec_sql_statements.len(), 1);
        let sql = &program.exec_sql_statements[0];
        assert_eq!(sql.sql_type, crate::ast::SqlStatementType::SelectInto);
        assert_eq!(sql.output_vars.len(), 2);
        assert_eq!(sql.output_vars[0].field_name, "WS-ENAME");
        assert_eq!(sql.output_vars[1].field_name, "WS-SAL");
        assert_eq!(sql.input_vars.len(), 1);
        assert_eq!(sql.input_vars[0].field_name, "WS-EMPNO");

        // Procedure division should still parse (EXEC SQL replaced with CONTINUE)
        assert!(program.procedure_division.is_some());
    }

    #[test]
    fn parse_exec_sql_multiple() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. MULTISQL.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-EMPNO  PIC 9(6).\n",
            "01  WS-ENAME  PIC X(20).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    EXEC SQL\n",
            "        INSERT INTO EMP (EMPNO, ENAME)\n",
            "        VALUES (:WS-EMPNO, :WS-ENAME)\n",
            "    END-EXEC.\n",
            "    EXEC SQL COMMIT END-EXEC.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok(), "parse failed: {result:?}");
        let program = result.unwrap();

        assert_eq!(program.exec_sql_statements.len(), 2);
        assert_eq!(
            program.exec_sql_statements[0].sql_type,
            crate::ast::SqlStatementType::Insert,
        );
        assert_eq!(
            program.exec_sql_statements[1].sql_type,
            crate::ast::SqlStatementType::Commit,
        );
    }

    #[test]
    fn parse_no_exec_sql() {
        let source = concat!(
            "IDENTIFICATION DIVISION.\n",
            "PROGRAM-ID. NOSQL.\n",
            "DATA DIVISION.\n",
            "WORKING-STORAGE SECTION.\n",
            "01  WS-X PIC 9(3).\n",
            "PROCEDURE DIVISION.\n",
            "MAIN-PARA.\n",
            "    DISPLAY 'HELLO'.\n",
            "    STOP RUN.\n",
        );

        let result = parse_cobol(source);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert!(program.exec_sql_statements.is_empty());
    }

    // -----------------------------------------------------------------------
    // Chunked DATA DIVISION parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn split_chunks_basic() {
        let data = "\
WORKING-STORAGE SECTION.
01  REC-A.
    05  FIELD-A PIC X(10).
01  REC-B.
    05  FIELD-B PIC 9(5).
01  REC-C.
    05  FIELD-C PIC X(20).
01  REC-D.
    05  FIELD-D PIC 9(3).
01  REC-E.
    05  FIELD-E PIC X(5).
";
        let chunks = split_data_division_into_chunks(data, 2);
        // batch_size=2: chunk1=[WS SECTION + REC-A + REC-B], chunk2=[REC-C + REC-D], chunk3=[REC-E]
        assert_eq!(chunks.len(), 3, "5 01-levels with batch=2 should produce 3 chunks");
        assert!(chunks[0].contains("REC-A"), "chunk 0 should have REC-A");
        assert!(chunks[0].contains("REC-B"), "chunk 0 should have REC-B");
        assert!(chunks[1].contains("REC-C"), "chunk 1 should have REC-C");
        assert!(chunks[1].contains("REC-D"), "chunk 1 should have REC-D");
        assert!(chunks[2].contains("REC-E"), "chunk 2 should have REC-E");
    }

    #[test]
    fn split_chunks_section_boundary() {
        let data = "\
WORKING-STORAGE SECTION.
01  WS-REC PIC X(10).
LINKAGE SECTION.
01  LS-REC PIC X(20).
";
        let chunks = split_data_division_into_chunks(data, 100);
        assert_eq!(chunks.len(), 2, "LINKAGE SECTION should force a new chunk");
        assert!(chunks[0].contains("WS-REC"), "chunk 0 should have WS-REC");
        assert!(chunks[1].contains("LS-REC"), "chunk 1 should have LS-REC");
        assert!(chunks[1].contains("LINKAGE SECTION"), "chunk 1 should have LINKAGE header");
    }

    #[test]
    fn split_chunks_subordinates_stay() {
        let data = "\
WORKING-STORAGE SECTION.
01  REC-A.
    05  FIELD-A1 PIC X(10).
    10  FIELD-A2 PIC 9(3).
    88  FLAG-YES VALUE 'Y'.
01  REC-B.
    05  FIELD-B1 PIC X(5).
    66  RENAMED RENAMES FIELD-B1.
";
        let chunks = split_data_division_into_chunks(data, 1);
        // batch_size=1: each 01-level in its own chunk (+ WS SECTION in first)
        assert_eq!(chunks.len(), 2, "2 01-levels with batch=1 should produce 2 chunks");
        assert!(chunks[0].contains("FIELD-A1"), "subordinate 05 stays with parent");
        assert!(chunks[0].contains("FIELD-A2"), "subordinate 10 stays with parent");
        assert!(chunks[0].contains("FLAG-YES"), "subordinate 88 stays with parent");
        assert!(chunks[1].contains("FIELD-B1"), "second record in chunk 1");
        assert!(chunks[1].contains("RENAMED"), "subordinate 66 stays with parent");
    }

    #[test]
    fn wrap_chunk_with_section() {
        let chunk = "WORKING-STORAGE SECTION.\n01  REC PIC X(5).\n";
        let wrapped = wrap_chunk_as_program(chunk);
        assert!(wrapped.contains("IDENTIFICATION DIVISION"), "has ID division");
        assert!(wrapped.contains("PROCEDURE DIVISION"), "has PROC division");
        // Should NOT duplicate WORKING-STORAGE SECTION
        assert_eq!(
            wrapped.matches("WORKING-STORAGE SECTION").count(),
            1,
            "should not duplicate section header"
        );
    }

    #[test]
    fn wrap_chunk_without_section() {
        let chunk = "01  REC PIC X(5).\n";
        let wrapped = wrap_chunk_as_program(chunk);
        assert!(wrapped.contains("WORKING-STORAGE SECTION"), "adds WS section header");
        assert!(wrapped.contains("IDENTIFICATION DIVISION"), "has ID division");
    }

    #[test]
    fn parse_chunks_end_to_end() {
        let source = "\
       IDENTIFICATION DIVISION.
       PROGRAM-ID. CHUNKTEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-REC-A.
           05  WS-FIELD-A PIC X(10).
       01  WS-REC-B.
           05  WS-FIELD-B PIC 9(5).
       01  WS-REC-C.
           05  WS-FIELD-C PIC X(20).
       01  WS-REC-D.
           05  WS-FIELD-D PIC 9(3).
       PROCEDURE DIVISION.
           STOP RUN.
";
        let data_text = extract_data_division_text(source).unwrap();
        let items = parse_data_chunks(&data_text, 2);

        // Should find all 4 01-level records + their 05-level children = 8 items
        let top_levels = items.iter().filter(|e| e.level == 1).count();
        let sub_levels = items.iter().filter(|e| e.level == 5).count();
        assert_eq!(top_levels, 4, "should find all 4 01-level records");
        assert_eq!(sub_levels, 4, "should find all 4 05-level fields");
    }

    // -----------------------------------------------------------------------
    // Chunked PROCEDURE DIVISION parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn split_proc_chunks_basic() {
        let proc_body = "\
SSUPDATE-MAINLINE SECTION.
MAIN-PARA.
    MOVE 1 TO WS-A.
PROCESS-DATA SECTION.
DATA-PARA.
    MOVE 2 TO WS-B.
CLEANUP SECTION.
CLEANUP-PARA.
    STOP RUN.
";
        let chunks = split_proc_division_into_chunks(proc_body, 2);
        // 3 sections with batch=2: chunk1=[SSUPDATE-MAINLINE + PROCESS-DATA], chunk2=[CLEANUP]
        assert_eq!(chunks.len(), 2, "3 sections with batch=2 should produce 2 chunks");
        assert!(chunks[0].contains("SSUPDATE-MAINLINE"), "chunk 0 has first section");
        assert!(chunks[0].contains("PROCESS-DATA"), "chunk 0 has second section");
        assert!(chunks[1].contains("CLEANUP"), "chunk 1 has third section");
    }

    #[test]
    fn split_proc_chunks_one_per_chunk() {
        let proc_body = "\
SECTION-A SECTION.
PARA-A.
    DISPLAY 'A'.
SECTION-B SECTION.
PARA-B.
    DISPLAY 'B'.
SECTION-C SECTION.
PARA-C.
    DISPLAY 'C'.
";
        let chunks = split_proc_division_into_chunks(proc_body, 1);
        assert_eq!(chunks.len(), 3, "3 sections with batch=1 should produce 3 chunks");
        assert!(chunks[0].contains("SECTION-A"), "chunk 0");
        assert!(chunks[1].contains("SECTION-B"), "chunk 1");
        assert!(chunks[2].contains("SECTION-C"), "chunk 2");
    }

    #[test]
    fn split_proc_chunks_standalone_paragraphs_first() {
        // Paragraphs before first SECTION stay with the first chunk
        let proc_body = "\
INIT-PARA.
    MOVE 0 TO WS-A.
FIRST-SECTION SECTION.
SECTION-PARA.
    DISPLAY 'HELLO'.
SECOND-SECTION SECTION.
SECOND-PARA.
    DISPLAY 'BYE'.
";
        let chunks = split_proc_division_into_chunks(proc_body, 1);
        assert_eq!(chunks.len(), 2, "standalone para + 2 sections with batch=1 = 2 chunks");
        assert!(chunks[0].contains("INIT-PARA"), "standalone para in first chunk");
        assert!(chunks[0].contains("FIRST-SECTION"), "first section in first chunk with standalone para");
        assert!(chunks[1].contains("SECOND-SECTION"), "second section in second chunk");
    }

    #[test]
    fn wrap_proc_chunk() {
        let chunk = "TEST-SECTION SECTION.\nTEST-PARA.\n    DISPLAY 'HI'.\n";
        let wrapped = wrap_proc_chunk_as_program(chunk);
        assert!(wrapped.contains("IDENTIFICATION DIVISION"), "has ID division");
        assert!(wrapped.contains("PROCEDURE DIVISION"), "has PROC division");
        assert!(wrapped.contains("TEST-SECTION SECTION"), "has the section");
    }

    #[test]
    fn parse_proc_chunks_end_to_end() {
        let source = "\
       IDENTIFICATION DIVISION.
       PROGRAM-ID. CHUNKPROC.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01  WS-A PIC X.
       PROCEDURE DIVISION.
       SECTION-A SECTION.
       PARA-A1.
           DISPLAY 'A1'.
       PARA-A2.
           DISPLAY 'A2'.
       SECTION-B SECTION.
       PARA-B1.
           MOVE 'X' TO WS-A.
";
        let proc_text = extract_proc_division_text(source).unwrap();
        // Strip the PROCEDURE DIVISION header line
        let body: String = proc_text.lines().skip(1).collect::<Vec<_>>().join("\n");
        let (sections, paragraphs, _diags, _errors) = parse_proc_chunks(&body, 1);

        assert_eq!(sections.len(), 2, "should find 2 sections");
        assert_eq!(sections[0].name, "SECTION-A", "first section name");
        assert_eq!(sections[1].name, "SECTION-B", "second section name");
        assert_eq!(sections[0].paragraphs.len(), 2, "SECTION-A has 2 paragraphs");
        assert_eq!(sections[1].paragraphs.len(), 1, "SECTION-B has 1 paragraph");
        assert!(paragraphs.is_empty(), "no standalone paragraphs");
    }
}
