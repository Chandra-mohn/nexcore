//! Stateful viewer session for windowed record decoding.
//!
//! `ViewerSession` wraps dataset file access, schema (copybook) attachment,
//! and windowed decoding into a single high-level API.

use crate::discriminator;
use crate::encoding::{self, Encoding};
use crate::error::{DataError, Result};
use crate::export::{self, ExportFormat};
use crate::layout;
use crate::record::{self, DecodedRecord};
use crate::redefines::{self, RedefinesGroup};
use cobol_transpiler::ast::DataEntry;
use cobol_transpiler::parser;

// ---------------------------------------------------------------------------
// FileAccess trait
// ---------------------------------------------------------------------------

/// Abstraction for file I/O (native mmap vs WASM host callbacks).
pub trait FileAccess: Send + Sync {
    /// Get the size of a file in bytes.
    fn file_size(&self, path: &str) -> Result<u64>;
    /// Read `length` bytes starting at `offset` from a file.
    fn read_bytes(&self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>>;
}

/// Native file access using standard I/O.
#[derive(Debug)]
pub struct NativeFileAccess;

impl FileAccess for NativeFileAccess {
    fn file_size(&self, path: &str) -> Result<u64> {
        let meta = std::fs::metadata(path)?;
        Ok(meta.len())
    }

    fn read_bytes(&self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>> {
        use std::io::{Read, Seek, SeekFrom};
        let mut file = std::fs::File::open(path)?;
        file.seek(SeekFrom::Start(offset))?;
        let mut buf = vec![0u8; length];
        let n = file.read(&mut buf)?;
        buf.truncate(n);
        Ok(buf)
    }
}

// ---------------------------------------------------------------------------
// ViewerSession
// ---------------------------------------------------------------------------

/// Stateful session wrapping dataset + schema + decoding.
#[derive(Debug)]
pub struct ViewerSession<F: FileAccess> {
    fa: F,
    dataset_path: Option<String>,
    dataset_size: Option<u64>,
    record_length: Option<usize>,
    entries: Option<Vec<DataEntry>>,
    groups: Option<Vec<RedefinesGroup>>,
    encoding: Option<Encoding>,
}

impl<F: FileAccess> ViewerSession<F> {
    /// Create a new session with the given file access backend.
    pub fn new(fa: F) -> Self {
        Self {
            fa,
            dataset_path: None,
            dataset_size: None,
            record_length: None,
            entries: None,
            groups: None,
            encoding: None,
        }
    }

    /// Open a binary dataset file. Clears any previous schema.
    pub fn open_dataset(&mut self, path: &str) -> Result<u64> {
        let size = self.fa.file_size(path)?;
        self.dataset_path = Some(path.to_string());
        self.dataset_size = Some(size);
        // Clear schema state
        self.record_length = None;
        self.entries = None;
        self.groups = None;
        self.encoding = None;
        Ok(size)
    }

    /// Attach a copybook schema (+ optional program source for discriminators).
    ///
    /// Parses the copybook via cobol-transpiler, extracts REDEFINES groups,
    /// and detects discriminators if program source is provided.
    pub fn attach_schema(
        &mut self,
        copybook_src: &str,
        program_src: Option<&str>,
    ) -> Result<()> {
        // Parse copybook
        let mut entries = parser::parse_data_division(copybook_src)
            .map_err(|e| DataError::ParseError(e.to_string()))?;

        // Compute layout
        for entry in &mut entries {
            layout::compute_layout(entry);
        }

        // Extract REDEFINES groups
        let mut groups = redefines::extract_redefines_groups(&entries);

        // Detect discriminators if program source provided
        if let Some(prog_src) = program_src {
            // Inline COPY statements: replace "COPY <name>." with copybook content
            // so the parser can resolve field references in PROCEDURE DIVISION.
            let expanded = inline_copy_statements(prog_src, copybook_src);
            match parser::parse_cobol(&expanded) {
                Ok(program) => {
                    if let Some(ref proc_div) = program.procedure_division {
                        discriminator::detect_discriminators(proc_div, &entries, &mut groups);
                    }
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Failed to parse program source for discriminator detection");
                }
            }
        }

        // Record length from first 01-level
        let rec_len = layout::record_length(&entries);
        self.record_length = rec_len;

        // Auto-detect encoding if dataset is open
        if let (Some(path), Some(rec_len)) = (&self.dataset_path, rec_len) {
            if rec_len > 0 {
                match self.fa.read_bytes(path, 0, rec_len) {
                    Ok(sample) => {
                        self.encoding = Some(encoding::detect_encoding(&sample, &entries));
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "Cannot read sample for encoding detection");
                    }
                }
            }
        }
        if self.encoding.is_none() {
            self.encoding = Some(Encoding::Ebcdic); // default
        }

        self.entries = Some(entries);
        self.groups = Some(groups);

        Ok(())
    }

    /// Number of records in the dataset.
    pub fn record_count(&self) -> Result<usize> {
        let size = self.dataset_size.ok_or(DataError::NoDataset)?;
        let rec_len = self.record_length.ok_or(DataError::NoSchema)?;
        if rec_len == 0 {
            return Ok(0);
        }
        Ok(size as usize / rec_len)
    }

    /// Decode a window of records starting at `start` index.
    pub fn decode_window(&self, start: usize, count: usize) -> Result<Vec<DecodedRecord>> {
        let path = self.dataset_path.as_deref().ok_or(DataError::NoDataset)?;
        let entries = self.entries.as_ref().ok_or(DataError::NoSchema)?;
        let groups = self.groups.as_deref().unwrap_or(&[]);
        let rec_len = self.record_length.ok_or(DataError::NoSchema)?;
        let encoding = self.encoding.unwrap_or(Encoding::Ebcdic);
        let total = self.record_count()?;

        if start >= total {
            return Err(DataError::OutOfRange {
                index: start,
                total,
            });
        }

        let actual_count = count.min(total - start);
        let byte_offset = (start * rec_len) as u64;
        let byte_length = actual_count * rec_len;

        let data = self.fa.read_bytes(path, byte_offset, byte_length)?;

        Ok(record::decode_records(
            &data, rec_len, entries, groups, encoding, Some(actual_count),
        ))
    }

    /// Read raw bytes from the dataset.
    pub fn read_raw_window(&self, start: u64, length: usize) -> Result<Vec<u8>> {
        let path = self.dataset_path.as_deref().ok_or(DataError::NoDataset)?;
        let size = self.dataset_size.ok_or(DataError::NoDataset)?;

        if start >= size {
            return Err(DataError::OutOfRange {
                index: start as usize,
                total: size as usize,
            });
        }

        let actual_len = length.min((size - start) as usize);
        self.fa.read_bytes(path, start, actual_len)
    }

    /// Export a range of records to a formatted string.
    pub fn export_range(
        &self,
        start: usize,
        count: usize,
        format: ExportFormat,
        pretty: bool,
    ) -> Result<String> {
        let records = self.decode_window(start, count)?;
        Ok(export::format_records(&records, format, pretty))
    }

    /// Streaming export to a writer. Returns (records_written, bytes_written).
    pub fn export_to_writer<W: std::io::Write>(
        &self,
        writer: &mut W,
        format: ExportFormat,
        pretty: bool,
    ) -> Result<(usize, u64)> {
        let total = self.record_count()?;
        let batch_size = 1000;
        let mut total_written = 0usize;
        let mut total_bytes = 0u64;

        match format {
            ExportFormat::Json => {
                writer
                    .write_all(if pretty { b"[\n" } else { b"[" })
                    .map_err(DataError::Io)?;
                total_bytes += if pretty { 2 } else { 1 };

                let mut start = 0;
                let mut first = true;
                while start < total {
                    let count = batch_size.min(total - start);
                    let records = self.decode_window(start, count)?;

                    for record in &records {
                        if !first {
                            let sep = if pretty { ",\n" } else { "," };
                            writer.write_all(sep.as_bytes()).map_err(DataError::Io)?;
                            total_bytes += sep.len() as u64;
                        }
                        first = false;

                        let json = record::decoded_to_json(record);
                        let s = if pretty {
                            serde_json::to_string_pretty(&json)
                        } else {
                            serde_json::to_string(&json)
                        }
                        .unwrap_or_else(|_| "{}".to_string());

                        writer.write_all(s.as_bytes()).map_err(DataError::Io)?;
                        total_bytes += s.len() as u64;
                        total_written += 1;
                    }

                    start += count;
                }

                writer
                    .write_all(if pretty { b"\n]" } else { b"]" })
                    .map_err(DataError::Io)?;
                total_bytes += if pretty { 2 } else { 1 };
            }
            ExportFormat::Csv => {
                // Decode first batch to get headers
                if total == 0 {
                    return Ok((0, 0));
                }
                let first_batch = self.decode_window(0, batch_size.min(total))?;
                let output = export::format_records(&first_batch, ExportFormat::Csv, false);
                writer.write_all(output.as_bytes()).map_err(DataError::Io)?;
                total_bytes += output.len() as u64;
                total_written += first_batch.len();

                let mut start = first_batch.len();
                while start < total {
                    let count = batch_size.min(total - start);
                    let records = self.decode_window(start, count)?;
                    // CSV rows only (no header)
                    for record in &records {
                        let headers: Vec<&String> = record.fields.keys().collect();
                        let row: Vec<String> = headers
                            .iter()
                            .map(|h| {
                                record
                                    .fields
                                    .get(*h)
                                    .map(|v| csv_value_inline(v))
                                    .unwrap_or_default()
                            })
                            .collect();
                        let line = format!("{}\n", row.join(","));
                        writer.write_all(line.as_bytes()).map_err(DataError::Io)?;
                        total_bytes += line.len() as u64;
                        total_written += 1;
                    }
                    start += count;
                }
            }
        }

        Ok((total_written, total_bytes))
    }

    // -- Getters --

    pub fn encoding(&self) -> Option<Encoding> {
        self.encoding
    }

    pub fn record_length(&self) -> Option<usize> {
        self.record_length
    }

    pub fn has_schema(&self) -> bool {
        self.entries.is_some()
    }

    pub fn dataset_path(&self) -> Option<&str> {
        self.dataset_path.as_deref()
    }

    pub fn dataset_size(&self) -> Option<u64> {
        self.dataset_size
    }

    pub fn entries(&self) -> Option<&[DataEntry]> {
        self.entries.as_deref()
    }

    pub fn groups(&self) -> Option<&[RedefinesGroup]> {
        self.groups.as_deref()
    }
}

/// Replace `COPY <name>.` lines in program source with copybook content.
///
/// Simple text-level substitution: any line matching `COPY <word>.` gets
/// replaced with the copybook source. This allows the ANTLR4 parser to
/// see field definitions from the copybook when parsing the program's
/// PROCEDURE DIVISION.
pub fn inline_copy_statements(program_src: &str, copybook_src: &str) -> String {
    let mut result = String::with_capacity(program_src.len() + copybook_src.len());
    for line in program_src.lines() {
        let trimmed = line.trim().to_uppercase();
        if trimmed.starts_with("COPY ") && trimmed.ends_with('.') {
            // Replace COPY line with copybook content
            result.push_str(copybook_src);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

fn csv_value_inline(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => {
            if s.contains(',') || s.contains('"') || s.contains('\n') {
                format!("\"{}\"", s.replace('"', "\"\""))
            } else {
                s.clone()
            }
        }
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        other => other.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Mutex;

    /// In-memory file access for testing.
    struct MemoryFileAccess {
        files: Mutex<HashMap<String, Vec<u8>>>,
    }

    impl MemoryFileAccess {
        fn new() -> Self {
            Self {
                files: Mutex::new(HashMap::new()),
            }
        }

        fn add_file(&self, path: &str, data: Vec<u8>) {
            self.files
                .lock()
                .unwrap()
                .insert(path.to_string(), data);
        }
    }

    impl FileAccess for MemoryFileAccess {
        fn file_size(&self, path: &str) -> Result<u64> {
            let files = self.files.lock().unwrap();
            files
                .get(path)
                .map(|d| d.len() as u64)
                .ok_or_else(|| DataError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("file not found: {path}"),
                )))
        }

        fn read_bytes(&self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>> {
            let files = self.files.lock().unwrap();
            let data = files
                .get(path)
                .ok_or_else(|| DataError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("file not found: {path}"),
                )))?;
            let start = offset as usize;
            let end = (start + length).min(data.len());
            Ok(data[start..end].to_vec())
        }
    }

    fn simple_copybook() -> &'static str {
        r#"       IDENTIFICATION DIVISION.
       PROGRAM-ID. TEST.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 TEST-RECORD.
          05 NAME       PIC X(20).
          05 CITY       PIC X(10).
"#
    }

    #[test]
    fn test_session_no_dataset() {
        let session = ViewerSession::new(NativeFileAccess);
        assert!(session.record_count().is_err());
    }

    #[test]
    fn test_session_no_schema() {
        let fa = MemoryFileAccess::new();
        fa.add_file("test.dat", vec![0; 100]);
        let mut session = ViewerSession::new(fa);
        session.open_dataset("test.dat").unwrap();
        assert!(session.record_count().is_err()); // no schema yet
    }

    #[test]
    fn test_session_lifecycle() {
        let fa = MemoryFileAccess::new();

        // Create a 30-byte dataset (1 record of 30 bytes)
        let mut record_data = Vec::new();
        record_data.extend_from_slice(b"Alice               "); // NAME (20)
        record_data.extend_from_slice(b"Boston    ");           // CITY (10)
        assert_eq!(record_data.len(), 30);

        // Two records
        let mut dataset = record_data.clone();
        dataset.extend_from_slice(&record_data);
        fa.add_file("test.dat", dataset);

        let mut session = ViewerSession::new(fa);

        // Open dataset
        let size = session.open_dataset("test.dat").unwrap();
        assert_eq!(size, 60);

        // Attach schema
        session.attach_schema(simple_copybook(), None).unwrap();
        assert!(session.has_schema());
        assert_eq!(session.record_length(), Some(30));

        // Record count
        assert_eq!(session.record_count().unwrap(), 2);

        // Decode window
        let records = session.decode_window(0, 2).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].fields.get("NAME").unwrap(), "Alice");
        assert_eq!(records[0].fields.get("CITY").unwrap(), "Boston");
    }

    #[test]
    fn test_session_decode_window_bounds() {
        let fa = MemoryFileAccess::new();
        let mut data = Vec::new();
        data.extend_from_slice(b"Rec1                Bos       ");
        data.extend_from_slice(b"Rec2                Chi       ");
        fa.add_file("test.dat", data);

        let mut session = ViewerSession::new(fa);
        session.open_dataset("test.dat").unwrap();
        session.attach_schema(simple_copybook(), None).unwrap();

        // Request more than available
        let records = session.decode_window(1, 10).unwrap();
        assert_eq!(records.len(), 1); // only 1 record from index 1

        // Out of range
        assert!(session.decode_window(5, 1).is_err());
    }

    #[test]
    fn test_session_export_json() {
        let fa = MemoryFileAccess::new();
        let mut data = Vec::new();
        data.extend_from_slice(b"Alice               Boston    ");
        fa.add_file("test.dat", data);

        let mut session = ViewerSession::new(fa);
        session.open_dataset("test.dat").unwrap();
        session.attach_schema(simple_copybook(), None).unwrap();

        let json = session
            .export_range(0, 1, ExportFormat::Json, false)
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_session_read_raw() {
        let fa = MemoryFileAccess::new();
        fa.add_file("test.dat", vec![0xAB, 0xCD, 0xEF, 0x01]);

        let mut session = ViewerSession::new(fa);
        session.open_dataset("test.dat").unwrap();

        let raw = session.read_raw_window(1, 2).unwrap();
        assert_eq!(raw, vec![0xCD, 0xEF]);
    }

    #[test]
    fn test_session_getters() {
        let fa = MemoryFileAccess::new();
        fa.add_file("test.dat", vec![0; 30]);

        let mut session = ViewerSession::new(fa);
        assert!(session.dataset_path().is_none());
        assert!(!session.has_schema());

        session.open_dataset("test.dat").unwrap();
        assert_eq!(session.dataset_path(), Some("test.dat"));
        assert_eq!(session.dataset_size(), Some(30));
    }
}
