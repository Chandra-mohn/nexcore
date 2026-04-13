# cobol-data Crate Design Specification

**Status**: DRAFT
**Date**: 2026-03-26
**Replaces**: coqu-di/core parser + layout + decode + discovery + viewer
**Decision**: Option D -- clean build in cobol2rust, coqu-di as reference only

## 1. Overview

The `cobol-data` crate provides binary COBOL data intelligence: parsing copybook
layouts, decoding binary mainframe datasets, detecting REDEFINES discriminators,
and auto-discovering file-to-copybook mappings.

It operates on cobol-transpiler's existing AST (`DataEntry`, `ProcedureDivision`,
`FileDescription`) -- no separate parser, no grammar duplication.

### What This Replaces

| coqu-di Module | cobol-data Module | Change |
|----------------|-------------------|--------|
| core/parser (ANTLR4) | -- (uses cobol-transpiler) | Eliminated |
| core/ast.rs | -- (uses DataEntry) | Eliminated |
| core/pic.rs | -- (uses PicClause) | Eliminated |
| core/layout.rs | layout.rs | Rewritten |
| core/decode.rs | decode.rs | Rewritten |
| core/record.rs | record.rs | Rewritten |
| core/redefines.rs | redefines.rs | Rewritten |
| core/discovery.rs | discovery.rs | Rewritten |
| viewer/session.rs | session.rs | Rewritten |
| cli/ | cobol-cli subcommands | New subcommands |
| tui/ | -- | Obsolete (NexStudio) |

### What Stays in coqu-di

Nothing. coqu-di becomes an archived reference implementation.

## 2. Crate Structure

```
crates/cobol-data/
  Cargo.toml
  src/
    lib.rs           Public API re-exports
    layout.rs        Byte offset computation
    decode.rs        Binary field decoding (EBCDIC, packed, zoned, binary)
    record.rs        Record-level decode with variant selection
    redefines.rs     REDEFINES group extraction + types
    discriminator.rs Discriminator detection from ProcedureDivision AST
    discovery.rs     File-to-copybook auto-matching
    session.rs       Stateful viewer session (windowed decode, mmap)
    encoding.rs      EBCDIC/ASCII tables and detection
    error.rs         Error types
    export.rs        JSON/CSV export formatting
```

## 3. Dependencies

```toml
[dependencies]
cobol-transpiler = { path = "../cobol-transpiler" }   # AST types + parser
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
miette = "7"
memmap2 = "0.9"       # Memory-mapped file access

[dev-dependencies]
tempfile = "3"
```

**Key**: cobol-data depends on cobol-transpiler for parsing. It does NOT
contain a parser. It consumes `DataEntry`, `PicClause`, `Usage`,
`FileDescription`, `CobolProgram`, and `ProcedureDivision` types.

## 4. Module Specifications

### 4.1 encoding.rs -- EBCDIC/ASCII Support

**Types**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Encoding {
    Ebcdic,   // CP037 (US/Canada mainframe)
    Ascii,    // ASCII/Latin-1
}
```

**Constants**:
- `CP037_TO_ASCII: [u8; 256]` -- static lookup table, EBCDIC byte -> ASCII byte
- `ASCII_TO_CP037: [u8; 256]` -- reverse table (for encoding)

**Functions**:
```rust
/// Decode EBCDIC bytes to UTF-8 string, trimming trailing spaces
pub fn decode_ebcdic(bytes: &[u8]) -> String

/// Decode ASCII bytes to UTF-8 string, trimming trailing spaces
pub fn decode_ascii(bytes: &[u8]) -> String

/// Decode bytes using specified encoding
pub fn decode_text(bytes: &[u8], encoding: Encoding) -> String

/// Auto-detect encoding from a sample of binary data + field definitions
/// Heuristic: score EBCDIC vs ASCII byte patterns in first PIC X field
pub fn detect_encoding(data: &[u8], entries: &[DataEntry]) -> Encoding
```

**Detection algorithm**:
1. Find first PIC X field with >= 5 bytes
2. Score byte patterns: 0x40 (EBCDIC space) vs 0x20 (ASCII space),
   0xF0-0xF9 (EBCDIC digits) vs 0x30-0x39 (ASCII digits),
   EBCDIC letter ranges vs ASCII letter ranges
3. Higher score wins; default to EBCDIC (mainframe assumption)

### 4.2 layout.rs -- Byte Offset Computation

cobol-transpiler already computes `byte_offset` and `byte_length` during
hierarchy building. This module provides supplementary layout utilities
for data intelligence scenarios where we need layout recomputation or
validation.

**Functions**:
```rust
/// Compute byte offsets for a record tree in-place.
/// Sets byte_offset on every DataEntry, handling:
///   - Sequential fields (cumulative offsets)
///   - REDEFINES (shares base field offset, no advance)
///   - OCCURS (total_space = byte_length * count)
///   - Level 88 (zero space, parent's offset)
///   - Group items (byte_length = sum of children span)
pub fn compute_layout(record: &mut DataEntry)

/// Get the total record length (byte_length of the 01-level record)
pub fn record_length(entries: &[DataEntry]) -> Option<usize>

/// Validate layout consistency (offsets don't overlap except REDEFINES)
pub fn validate_layout(entries: &[DataEntry]) -> Vec<LayoutWarning>
```

**Algorithm** (recursive):
```
compute_offsets(item, start_offset) -> next_offset:
  item.byte_offset = start_offset

  if no data children (leaf):
    if level 88: return start_offset (zero space)
    return start_offset + byte_length * occurs.unwrap_or(1)

  if group:
    offset = start_offset
    base_offsets: HashMap<String, usize>  // for REDEFINES lookup
    for child in children:
      if level 88: child.byte_offset = offset; continue
      if child.redefines.is_some():
        base = base_offsets[child.redefines]
        compute_offsets(child, base)  // don't advance offset
      else:
        base_offsets[child.name] = offset
        offset = compute_offsets(child, offset)
    item.byte_length = offset - start_offset
    return offset
```

### 4.3 decode.rs -- Binary Field Decoding

**Types**:
```rust
#[derive(Debug, Clone, Serialize)]
pub enum DecodedValue {
    Text(String),
    Integer(i64),
    Decimal { value: i64, scale: u32 },  // value / 10^scale
    Bytes(Vec<u8>),                       // raw (COMP-1/COMP-2 hex floats)
}
```

**Functions**:
```rust
/// Decode a single field from binary bytes.
/// Routes to format-specific decoder based on Usage.
pub fn decode_field(
    bytes: &[u8],
    pic: &PicClause,
    usage: Usage,
    encoding: Encoding,
) -> Result<DecodedValue>

/// Decode zoned decimal (DISPLAY numeric).
/// One digit per byte. EBCDIC: zone in high nibble, digit in low.
/// Sign: last byte zone 0x0D=negative, 0x0C=positive, 0x0F=unsigned.
pub fn decode_zoned_decimal(
    bytes: &[u8],
    pic: &PicClause,
    encoding: Encoding,
) -> Result<DecodedValue>

/// Decode packed decimal (COMP-3 / BCD).
/// Two digits per byte (high nibble, low nibble).
/// Last nibble = sign: 0x0C=positive, 0x0D=negative, 0x0F=unsigned.
pub fn decode_packed_decimal(
    bytes: &[u8],
    pic: &PicClause,
) -> Result<DecodedValue>

/// Decode binary integer (COMP / COMP-5).
/// Big-endian, 2/4/8 bytes. Signed if PIC has S prefix.
pub fn decode_binary(
    bytes: &[u8],
    pic: &PicClause,
) -> Result<DecodedValue>
```

**Dispatcher logic** (`decode_field`):
```
match usage:
  Display:
    if pic.category is Alphanumeric/Alphabetic/Edited:
      decode_text(bytes, encoding) -> DecodedValue::Text
    else:
      decode_zoned_decimal(bytes, pic, encoding)
  Comp3:
    decode_packed_decimal(bytes, pic)
  Comp | Comp5:
    decode_binary(bytes, pic)
  Comp1 | Comp2:
    DecodedValue::Bytes(bytes.to_vec())  // IBM hex float, pass through
  Index | Pointer:
    DecodedValue::Bytes(bytes.to_vec())
```

### 4.4 redefines.rs -- REDEFINES Group Extraction

**Types**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedefinesGroup {
    pub base_field: String,
    pub byte_offset: usize,
    pub byte_length: usize,
    pub variants: Vec<RedefinesVariant>,
    pub discriminator: Option<Discriminator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedefinesVariant {
    pub name: String,
    pub fields: Vec<String>,  // child field names
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discriminator {
    pub field: String,
    pub pattern_type: DiscriminatorPattern,
    pub rules: Vec<DiscriminatorRule>,
    pub confidence: Confidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscriminatorPattern {
    DirectIf,       // IF field = 'value' ... variant-fields
    EvaluateWhen,   // EVALUATE field WHEN 'value' ... variant-fields
    Level88,        // IF condition-name ... variant-fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscriminatorRule {
    pub value: String,
    pub selects: String,  // variant name
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    High,       // Direct IF or EVALUATE on field (90-100%)
    Medium,     // 88-level indirection (70-89%)
    Low,        // Nested conditions (50-69%)
    Unresolved, // No discriminator found, needs SME review
}
```

**Functions**:
```rust
/// Extract REDEFINES groups from a record's children.
/// Scans top-level entries for REDEFINES relationships,
/// groups variants by base field, populates byte range.
pub fn extract_redefines_groups(entries: &[DataEntry]) -> Vec<RedefinesGroup>
```

**Algorithm**:
```
for each entry in record's children:
  if entry.redefines.is_some():
    find or create group for base_field
    add variant { name, child field names }
    set group byte_offset and byte_length from base field
return groups (with discriminator = None initially)
```

### 4.5 discriminator.rs -- Discriminator Detection from AST

This is the key module that replaces coqu-di's `proc_listener.rs`. Instead
of a second ANTLR4 parse, it walks cobol-transpiler's `ProcedureDivision` AST.

**Functions**:
```rust
/// Detect discriminators for REDEFINES groups by walking the
/// ProcedureDivision AST. Mutates groups in-place.
///
/// Detection patterns:
///   1. EVALUATE field WHEN 'value' ... (variant fields referenced)
///   2. IF field = 'value' ... (variant fields referenced)
///   3. IF condition-name ... (88-level -> parent field -> comparison)
///
/// Uses ConditionMap from cobol-transpiler for 88-level resolution.
pub fn detect_discriminators(
    proc_div: &ProcedureDivision,
    entries: &[DataEntry],
    groups: &mut [RedefinesGroup],
)
```

**Algorithm**:
```
1. Build ConditionMap from entries (88-level -> parent field + values)
2. Build variant_to_group map: field_name -> which RedefinesGroup
3. Walk all statements in ProcedureDivision:
   - sections -> paragraphs -> sentences -> statements (recursive)

4. For each EVALUATE statement:
   a. Extract subject field name from EvaluateSubject::Expr
   b. For each WhenBranch:
      - Extract literal values from WhenValue::Value
      - Collect field names referenced in branch body
      - Match referenced fields against variant_to_group
      - If match: record rule (value -> variant)
   c. If rules found: attach Discriminator with EvaluateWhen pattern

5. For each IF statement:
   a. Walk Condition tree:
      - Comparison { left: DataRef, op: Eq, right: Literal }
        -> direct_if candidate (field = value)
      - ConditionName(DataRef)
        -> resolve via ConditionMap to parent field + values
   b. Collect field names referenced in then_body
   c. Match against variant_to_group
   d. Walk else_body for chained IF-ELSE (same discriminator field)
   e. If rules found: attach Discriminator

6. Confidence assignment:
   - EVALUATE with exact values -> High
   - Direct IF comparison -> High
   - 88-level indirection -> Medium
   - Nested/complex conditions -> Low
   - No match found -> Unresolved (group.discriminator stays None)
```

**Helpers** (internal):
```rust
/// Collect all DataReference names from a statement list (recursive)
fn collect_field_references(stmts: &[Statement]) -> HashSet<String>

/// Extract literal value from an Operand
fn extract_literal_value(operand: &Operand) -> Option<String>

/// Walk a Condition tree, extract (field_name, comparison_value) pairs
fn extract_comparisons(cond: &Condition) -> Vec<(String, String)>
```

### 4.6 record.rs -- Record-Level Decoding

**Types**:
```rust
#[derive(Debug, Clone, Serialize)]
pub struct DecodedRecord {
    pub fields: serde_json::Map<String, serde_json::Value>,
    pub variant: Option<String>,
    pub warnings: Vec<String>,
}
```

**Functions**:
```rust
/// Decode a single fixed-length record from binary bytes.
/// Walks the DataEntry tree, applies discriminator logic for
/// REDEFINES groups, decodes all leaf fields.
pub fn decode_record(
    data: &[u8],
    entries: &[DataEntry],
    groups: &[RedefinesGroup],
    encoding: Encoding,
) -> DecodedRecord

/// Batch decode: split data into fixed-length records and decode each.
pub fn decode_records(
    data: &[u8],
    record_length: usize,
    entries: &[DataEntry],
    groups: &[RedefinesGroup],
    encoding: Encoding,
    limit: Option<usize>,
) -> Vec<DecodedRecord>
```

**Tree-walk algorithm** (`decode_children`):
```
for each child in entry.children:
  skip 88-level conditions

  if child is base of a REDEFINES group:
    read discriminator value from data
    match value against group.rules
    if matched:
      find variant DataEntry among siblings
      recurse into variant's children
      record variant name
    else:
      warn "unresolved REDEFINES"
      decode base as raw text fallback
    skip all REDEFINES siblings

  skip standalone REDEFINES (already handled by base)

  if group with OCCURS:
    count = effective_occurs(child, record)  // DEPENDING ON resolution
    for occ in 0..count:
      recurse with offset_adj = occ * byte_length
    collect into JSON array

  if group without OCCURS:
    recurse into children

  if leaf:
    if OCCURS: decode array of values
    else: decode single value
    insert into record.fields[unique_key]
```

**OCCURS DEPENDING ON resolution**:
```
effective_occurs(entry, record):
  max = entry.occurs.unwrap_or(0)
  if entry.occurs_depending.is_some():
    look up controlling field value in record.fields
    clamp to 0..max
  else:
    return max
```

**Duplicate field name handling** (`unique_key`):
```
if name not in fields: return name
else: return name-2, name-3, ... (first unused)
```

### 4.7 discovery.rs -- File-to-Copybook Matching

**Types**:
```rust
#[derive(Debug, Clone, Serialize)]
pub struct CopybookInfo {
    pub path: String,
    pub stem: String,             // basename, uppercase
    pub record_length: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgramInfo {
    pub path: String,
    pub declarations: Vec<FileDeclaration>,
    pub copy_targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DataFileInfo {
    pub path: String,
    pub stem: String,
    pub size: u64,
}

// FileDeclaration reuses cobol-transpiler's FileDescription fields
#[derive(Debug, Clone, Serialize)]
pub struct FileDeclaration {
    pub logical_name: String,
    pub physical_name: String,
    pub recording_mode: Option<RecordingMode>,
    pub record_size: Option<RecordSize>,
}

#[derive(Debug, Clone, Serialize)]
pub enum MatchSignal {
    ProgramLink,                    // SELECT/ASSIGN + COPY chain
    RecordLengthMatch,              // file_size % record_length == 0
    TrialDecode { score: f64 },     // % of fields decoded without error
    NameSimilarity { score: f64 },  // stem edit distance
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryMatch {
    pub copybook_path: String,
    pub program_path: Option<String>,
    pub confidence: Confidence,
    pub signals: Vec<MatchSignal>,
    pub record_length: Option<usize>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileDiscoveryResult {
    pub data_file: String,
    pub file_size: u64,
    pub matches: Vec<DiscoveryMatch>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveryReport {
    pub results: Vec<FileDiscoveryResult>,
    pub fragment_copybooks: Vec<String>,   // copybooks with no 01-level
    pub parse_errors: Vec<(String, String)>,
}
```

**Functions**:
```rust
/// Match data files to copybooks via program link chains.
/// HIGH confidence: SELECT/ASSIGN -> FD -> COPY -> copybook -> record length check
pub fn match_by_program(
    programs: &[ProgramInfo],
    copybooks: &[CopybookInfo],
    dat_files: &[DataFileInfo],
) -> Vec<FileDiscoveryResult>

/// Match data files to copybooks by record length divisibility.
/// MEDIUM/LOW confidence: file_size % record_length == 0
pub fn match_by_length(
    copybooks: &[CopybookInfo],
    dat_files: &[DataFileInfo],
) -> Vec<FileDiscoveryResult>

/// Trial decode: attempt to decode N records and score success rate.
/// Adds TrialDecode signal to existing matches.
pub fn trial_decode(
    result: &mut FileDiscoveryResult,
    copybooks: &[CopybookInfo],
    sample_records: usize,
) -> Result<()>

/// Merge program-link and length-based results, deduplicate.
pub fn merge_results(
    program_matches: Vec<FileDiscoveryResult>,
    length_matches: Vec<FileDiscoveryResult>,
) -> DiscoveryReport

/// Full discovery pipeline: parse programs, compute layouts, match, merge.
pub fn discover(
    program_paths: &[&str],
    copybook_paths: &[&str],
    data_file_paths: &[&str],
) -> Result<DiscoveryReport>
```

### 4.8 session.rs -- Viewer Session API

**Types**:
```rust
/// File access abstraction (native mmap vs WASM host callbacks)
pub trait FileAccess: Send + Sync {
    fn file_size(&self, path: &str) -> Result<u64>;
    fn read_bytes(&self, path: &str, offset: u64, length: usize) -> Result<Vec<u8>>;
}

/// Native file access using memory-mapped I/O
pub struct NativeFileAccess;

/// Export format for decoded records
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    Json,
    Csv,
}

/// Stateful session wrapping dataset + schema + decoding
pub struct ViewerSession<F: FileAccess> {
    fa: F,
    dataset_path: Option<String>,
    dataset_size: Option<u64>,
    record_length: Option<usize>,
    entries: Option<Vec<DataEntry>>,
    groups: Option<Vec<RedefinesGroup>>,
    encoding: Option<Encoding>,
}
```

**Methods**:
```rust
impl<F: FileAccess> ViewerSession<F> {
    pub fn new(fa: F) -> Self

    /// Open a binary dataset file. Clears any previous schema.
    pub fn open_dataset(&mut self, path: &str) -> Result<u64>

    /// Attach a copybook schema (+ optional program source for discriminators).
    /// Parses copybook via cobol-transpiler, extracts REDEFINES groups,
    /// detects discriminators if program source provided.
    pub fn attach_schema(
        &mut self,
        copybook_src: &str,
        program_src: Option<&str>,
    ) -> Result<()>

    /// Number of records in the dataset.
    pub fn record_count(&self) -> Result<usize>

    /// Decode a window of records (start index, count).
    pub fn decode_window(&self, start: usize, count: usize) -> Result<Vec<DecodedRecord>>

    /// Read raw bytes from dataset.
    pub fn read_raw_window(&self, start: u64, length: usize) -> Result<Vec<u8>>

    /// Export a range to string (JSON or CSV).
    pub fn export_range(
        &self,
        start: usize,
        count: usize,
        format: ExportFormat,
        pretty: bool,
    ) -> Result<String>

    /// Streaming export to writer (bounded memory, batch_size=1000).
    pub fn export_to_writer<W: std::io::Write>(
        &self,
        writer: &mut W,
        format: ExportFormat,
        pretty: bool,
    ) -> Result<(usize, u64)>

    // Getters
    pub fn encoding(&self) -> Option<Encoding>
    pub fn record_length(&self) -> Option<usize>
    pub fn has_schema(&self) -> bool
    pub fn dataset_path(&self) -> Option<&str>
    pub fn dataset_size(&self) -> Option<u64>
    pub fn entries(&self) -> Option<&[DataEntry]>
    pub fn groups(&self) -> Option<&[RedefinesGroup]>
}
```

**`attach_schema` pipeline**:
```
1. parse_cobol(copybook_src) -> CobolProgram
   (cobol-transpiler wraps standalone copybooks automatically)
2. Extract DataEntry tree from data_division.working_storage
3. compute_layout() on each 01-level record
4. extract_redefines_groups(&entries)
5. If program_src provided:
   a. parse_cobol(program_src) -> CobolProgram
   b. detect_discriminators(&proc_div, &entries, &mut groups)
6. record_length = entries[0].byte_length
7. If dataset is open:
   a. Read first record bytes
   b. detect_encoding(&sample, &entries)
8. Store entries, groups, record_length, encoding
```

### 4.9 error.rs -- Error Types

```rust
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum DataError {
    #[error("No dataset open")]
    NoDataset,

    #[error("No schema attached")]
    NoSchema,

    #[error("Record index {index} out of range (total: {total})")]
    OutOfRange { index: usize, total: usize },

    #[error("Decode error at field '{field}' offset {offset}: {reason}")]
    DecodeError { field: String, offset: usize, reason: String },

    #[error("Invalid PIC clause '{clause}': {reason}")]
    InvalidPic { clause: String, reason: String },

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DataError>;
```

## 5. CLI Integration

New subcommands for `cobol2rust` CLI (in cobol-cli crate):

### 5.1 `cobol2rust decode`

```
cobol2rust decode <data-file> --copybook <path> [--program <path>]
    [--start <n>] [--count <n>] [--format json|csv] [--pretty]
    [--encoding ebcdic|ascii|auto]
```

Decode binary COBOL dataset records to JSON/CSV.

### 5.2 `cobol2rust discover`

```
cobol2rust discover <data-dir> --copybooks <dir> [--programs <dir>]
    [--format json|table] [--trial-decode]
```

Auto-match data files to copybooks.

### 5.3 `cobol2rust analyze`

```
cobol2rust analyze <copybook> [--program <path>]
    [--format json|table]
```

Show record layout, REDEFINES groups, discriminators.

## 6. NexStudio Integration

NexStudio's Tauri backend calls `cobol-data` directly:

```rust
// In src-tauri/src/commands/data.rs
use cobol_data::{ViewerSession, NativeFileAccess};

#[tauri::command]
pub fn open_dataset(path: String, state: State<AppState>) -> Result<u64> {
    let mut session = state.viewer_session.lock()?;
    session.open_dataset(&path)
}

#[tauri::command]
pub fn attach_schema(
    copybook: String,
    program: Option<String>,
    state: State<AppState>,
) -> Result<()> {
    let mut session = state.viewer_session.lock()?;
    session.attach_schema(&copybook, program.as_deref())
}

#[tauri::command]
pub fn decode_window(
    start: usize,
    count: usize,
    state: State<AppState>,
) -> Result<Vec<serde_json::Value>> {
    let session = state.viewer_session.lock()?;
    let records = session.decode_window(start, count)?;
    Ok(records.iter().map(decoded_to_json).collect())
}
```

No coqu-di dependency in NexStudio. Single dependency: `cobol-data`.

## 7. Test Strategy

### Unit Tests (inline, per module)

| Module | Test Cases | Reference |
|--------|-----------|-----------|
| encoding | CP037 table, detect EBCDIC/ASCII, edge cases | coqu-di decode.rs tests |
| layout | Sequential, REDEFINES overlay, OCCURS, 88-level, groups | coqu-di layout.rs tests |
| decode | Zoned decimal +/-, packed decimal +/-, binary 2/4/8 byte, text | coqu-di decode.rs tests |
| record | Tree walk, discriminator match, OCCURS arrays, DEPENDING ON | coqu-di record.rs tests |
| redefines | Group extraction, variant listing | coqu-di redefines.rs tests |
| discriminator | EVALUATE pattern, IF pattern, 88-level, chained ELSE | New (coqu-di proc_listener tests) |
| discovery | Program link, length match, merge, normalize | coqu-di discovery.rs tests |
| session | Open/attach/decode/export lifecycle | coqu-di viewer tests |

**Target**: 100+ tests (parity with coqu-di's 88 core + 13 viewer = 101).

### Integration Tests

- Round-trip: COBOL copybook -> parse -> layout -> decode sample binary -> verify values
- Discovery: directory of programs + copybooks + data files -> matches
- Export: decode -> JSON -> parse back -> verify fields

### Test Data

Port coqu-di's test fixtures (binary samples, copybook snippets) into
`crates/cobol-data/tests/fixtures/`.

## 8. Implementation Plan

### Session 1: Foundation (encoding, layout, decode, error)

1. Create `crates/cobol-data/` with Cargo.toml
2. Implement `encoding.rs` -- CP037 table, detect, decode_text
3. Implement `layout.rs` -- compute_layout on DataEntry trees
4. Implement `decode.rs` -- zoned, packed, binary, dispatcher
5. Implement `error.rs`
6. Tests: ~40 unit tests

### Session 2: Records and REDEFINES (record, redefines, discriminator)

1. Implement `redefines.rs` -- types + extract_redefines_groups
2. Implement `discriminator.rs` -- AST walker for ProcedureDivision
3. Implement `record.rs` -- tree walk, variant selection, OCCURS
4. Tests: ~40 unit tests

### Session 3: Discovery, Session, CLI, Integration

1. Implement `discovery.rs` -- program link + length match + merge
2. Implement `session.rs` -- ViewerSession with NativeFileAccess
3. Implement `export.rs` -- JSON/CSV formatting
4. Add CLI subcommands (decode, discover, analyze) to cobol-cli
5. Integration tests
6. Tests: ~30 unit tests + integration

### Session 4: NexStudio Wiring + Validation

1. Replace coqu-di dependency in NexStudio with cobol-data
2. Update Tauri commands to use new API
3. Validate with real client data files (if available)
4. Performance comparison vs coqu-di

## 9. Binary Size Impact

| Component | Size Impact |
|-----------|-------------|
| cobol-data (new) | ~200 KB (logic only, no grammar) |
| coqu-di ANTLR4 grammar (removed) | -5 MB (grammar + generated lexer/parser) |
| cobol-transpiler (already included) | 0 (already in NexStudio) |
| **Net change** | **~-5 MB** (improvement) |

NexStudio binary: ~25 MB -> ~20 MB after removing coqu-di.

## 10. Migration Checklist

- [ ] Create cobol-data crate with all modules
- [ ] Port test cases from coqu-di (inputs + expected outputs)
- [ ] Verify decode parity on sample binary data
- [ ] Add CLI subcommands
- [ ] Remove coqu-di dependency from NexStudio
- [ ] Update NexStudio Tauri commands
- [ ] Archive coqu-di repository
- [ ] Update NexStudio data viewer component (should need minimal changes)

## 11. Open Questions

1. **WASM target**: Do we need WASM FileAccess for browser-based decoding?
   Current answer: No, NexStudio is Tauri (native). Defer WASM to future.

2. **Variable-length records (VB/RDW)**: coqu-di assumed fixed-length.
   Should cobol-data handle RDW (4-byte prefix) for variable-length files?
   Recommendation: Add as a follow-up, not in initial implementation.

3. **Multiple record types per file**: Some datasets have multiple 01-level
   records in one file (e.g., header + detail + trailer). The current
   session model assumes uniform record length. Consider multi-schema
   support as a future enhancement.

4. **Code page variants**: CP037 (US) is hardcoded. Some clients use
   CP500 (international) or CP1047 (z/OS Unix). Add code page selection
   as a future enhancement.
