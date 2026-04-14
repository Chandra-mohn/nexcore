//! BMS (Basic Mapping Support) parser -- extracts screen definitions from
//! CICS BMS macro files.
//!
//! Parses DFHMSD (mapset), DFHMDI (map), and DFHMDF (field) macros into
//! a typed AST. The grammar specification is in `grammar/BmsDSL.g4`.
//!
//! BMS files use assembler macro syntax with:
//! - Continuation lines ending with `-` near column 72
//! - KEY=VALUE option pairs, comma-separated
//! - Parenthesized lists: ATTRB=(PROT,BRT), POS=(3,15)
//! - Quoted strings: INITIAL='text'
//! - Comment lines starting with `*`

use std::collections::HashMap;
use std::path::Path;

use serde::Serialize;

use crate::error::{Result, TranspileError};

// ---------------------------------------------------------------------------
// AST Types
// ---------------------------------------------------------------------------

/// A complete BMS mapset definition (one .bms file).
#[derive(Debug, Clone, Serialize)]
pub struct BmsMapset {
    /// Mapset name (e.g., "COSGN00").
    pub name: String,
    /// I/O mode: Input, Output, or InOut.
    pub mode: MapMode,
    /// Target language for generated copybook.
    pub lang: String,
    /// Individual map (screen) definitions within this mapset.
    pub maps: Vec<BmsMap>,
    /// All raw options from the DFHMSD macro.
    pub options: HashMap<String, String>,
}

/// A single map (screen) definition within a mapset.
#[derive(Debug, Clone, Serialize)]
pub struct BmsMap {
    /// Map name (e.g., "COSGN0A").
    pub name: String,
    /// Screen dimensions: (rows, cols). Typically (24, 80).
    pub size: (u8, u8),
    /// Starting line (usually 1).
    pub line: u8,
    /// Starting column (usually 1).
    pub column: u8,
    /// Field definitions on this map.
    pub fields: Vec<BmsField>,
    /// All raw options from the DFHMDI macro.
    pub options: HashMap<String, String>,
}

/// A single field definition on a map.
#[derive(Debug, Clone, Serialize)]
pub struct BmsField {
    /// Field name. `None` for unnamed labels/spacers.
    pub name: Option<String>,
    /// Position: (row, col).
    pub pos: (u8, u8),
    /// Field length in characters.
    pub length: u16,
    /// Field attributes.
    pub attributes: BmsAttributes,
    /// Display color.
    pub color: Option<BmsColor>,
    /// Highlight mode.
    pub hilight: Option<BmsHilight>,
    /// Initial/default value displayed.
    pub initial: Option<String>,
    /// Input picture clause (e.g., "99999999999").
    pub pic_in: Option<String>,
    /// Output picture clause (e.g., "+ZZZ,ZZZ,ZZZ.99").
    pub pic_out: Option<String>,
    /// Justification.
    pub justify: Option<BmsJustify>,
    /// All raw options from the DFHMDF macro.
    pub options: HashMap<String, String>,
}

/// BMS field attributes (from ATTRB= option).
#[derive(Debug, Clone, Default, Serialize)]
pub struct BmsAttributes {
    /// PROT -- field is protected (display only).
    pub protected: bool,
    /// UNPROT -- field is unprotected (user can type).
    pub unprotected: bool,
    /// ASKIP -- auto-skip (cursor skips over this field).
    pub auto_skip: bool,
    /// BRT -- bright intensity.
    pub bright: bool,
    /// NORM -- normal intensity.
    pub normal: bool,
    /// DRK -- dark (invisible, used for passwords).
    pub dark: bool,
    /// NUM -- numeric only input.
    pub numeric: bool,
    /// IC -- initial cursor position.
    pub initial_cursor: bool,
    /// FSET -- modified data tag set (field treated as modified).
    pub fset: bool,
}

/// BMS display colors.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum BmsColor {
    Default,
    Blue,
    Red,
    Pink,
    Green,
    Turquoise,
    Yellow,
    Neutral,
}

/// BMS highlight modes.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum BmsHilight {
    Off,
    Blink,
    Reverse,
    Underline,
}

/// BMS field justification.
#[derive(Debug, Clone, Serialize)]
pub struct BmsJustify {
    pub right: bool,
    pub zero_fill: bool,
}

/// Map I/O mode.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum MapMode {
    Input,
    Output,
    InOut,
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// Parse a BMS file from a file path.
pub fn parse_bms_file(path: &Path) -> Result<BmsMapset> {
    let source = std::fs::read_to_string(path).map_err(|e| TranspileError::Preprocess {
        line: 0,
        message: format!("cannot read BMS file {}: {e}", path.display()),
    })?;
    parse_bms(&source)
}

/// Parse BMS macro source into a `BmsMapset` AST.
pub fn parse_bms(source: &str) -> Result<BmsMapset> {
    // Step 1: Strip comments and join continuation lines.
    let logical_lines = preprocess_bms(source);

    // Step 2: Parse each logical line as a macro.
    let mut mapset: Option<BmsMapset> = None;
    let mut current_map: Option<BmsMap> = None;

    for (line_num, line) in logical_lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse: [NAME] MACRO_TYPE OPTIONS...
        let (name, macro_type, options_str) = split_macro_line(trimmed);

        match macro_type.as_str() {
            "DFHMSD" => {
                let opts = parse_options(&options_str);
                // Check for TYPE=FINAL (mapset terminator)
                if opts.get("TYPE").map_or(false, |v| v.eq_ignore_ascii_case("FINAL")) {
                    // End of mapset -- push last map if any
                    if let (Some(ms), Some(map)) = (&mut mapset, current_map.take()) {
                        ms.maps.push(map);
                    }
                    continue;
                }
                let mode = match opts.get("MODE").map(|s| s.as_str()) {
                    Some("INOUT") => MapMode::InOut,
                    Some("IN") => MapMode::Input,
                    Some("OUT") => MapMode::Output,
                    _ => MapMode::InOut,
                };
                let lang = opts.get("LANG").cloned().unwrap_or_else(|| "COBOL".to_string());
                mapset = Some(BmsMapset {
                    name: name.unwrap_or_default(),
                    mode,
                    lang,
                    maps: Vec::new(),
                    options: opts,
                });
            }
            "DFHMDI" => {
                // Push previous map if any
                if let (Some(ms), Some(map)) = (&mut mapset, current_map.take()) {
                    ms.maps.push(map);
                }
                let opts = parse_options(&options_str);
                let size = parse_size_tuple(opts.get("SIZE").map(|s| s.as_str()).unwrap_or("(24,80)"));
                let line_val = opts.get("LINE").and_then(|v| v.parse().ok()).unwrap_or(1);
                let col_val = opts.get("COLUMN").and_then(|v| v.parse().ok()).unwrap_or(1);
                current_map = Some(BmsMap {
                    name: name.unwrap_or_default(),
                    size,
                    line: line_val,
                    column: col_val,
                    fields: Vec::new(),
                    options: opts,
                });
            }
            "DFHMDF" => {
                let opts = parse_options(&options_str);
                let field = build_field(name, &opts);
                if let Some(ref mut map) = current_map {
                    map.fields.push(field);
                } else {
                    tracing::warn!(line = line_num + 1, "BMS: DFHMDF outside of any map, skipping");
                }
            }
            "TITLE" | "END" => {
                // Assembler directives -- ignore
            }
            other => {
                if !other.is_empty() {
                    tracing::warn!(line = line_num + 1, macro_type = %other, "BMS: unknown macro type, skipping");
                }
            }
        }
    }

    // Push last map
    if let (Some(ms), Some(map)) = (&mut mapset, current_map.take()) {
        ms.maps.push(map);
    }

    mapset.ok_or_else(|| TranspileError::Preprocess {
        line: 0,
        message: "no DFHMSD mapset definition found in BMS source".to_string(),
    })
}

// ---------------------------------------------------------------------------
// Preprocessing: strip comments, join continuations
// ---------------------------------------------------------------------------

/// Preprocess BMS source: strip comment lines and join continuation lines.
///
/// Continuation is indicated by `-` near the end of a line (columns ~71-72).
/// The `-` and everything after it on that line is removed, and the next
/// line's content is appended.
fn preprocess_bms(source: &str) -> Vec<String> {
    let mut logical_lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    for raw_line in source.lines() {
        // Skip comment lines (first non-space char is '*')
        let trimmed = raw_line.trim();
        if trimmed.starts_with('*') {
            continue;
        }

        // Check for continuation: line content ends with '-'
        // The '-' may be followed by spaces. Strip trailing spaces first.
        let content = raw_line.trim_end();
        if content.ends_with('-') {
            // Remove the continuation marker and append
            let without_cont = content.trim_end_matches('-').trim_end();
            if current_line.is_empty() {
                current_line = without_cont.to_string();
            } else {
                // Append with a space separator
                current_line.push(' ');
                current_line.push_str(trimmed);
                // Remove trailing '-' from the appended part too
                if current_line.ends_with('-') {
                    let len = current_line.trim_end_matches('-').trim_end().len();
                    current_line.truncate(len);
                }
            }
        } else if !current_line.is_empty() {
            // End of continuation -- append this line and flush
            current_line.push(' ');
            current_line.push_str(trimmed);
            logical_lines.push(std::mem::take(&mut current_line));
        } else {
            // Standalone line
            if !trimmed.is_empty() {
                logical_lines.push(trimmed.to_string());
            }
        }
    }

    // Flush any remaining continuation
    if !current_line.is_empty() {
        logical_lines.push(current_line);
    }

    logical_lines
}

// ---------------------------------------------------------------------------
// Macro line splitting
// ---------------------------------------------------------------------------

/// Split a logical BMS line into (optional_name, macro_type, options_text).
///
/// BMS format: `[NAME] MACRO_TYPE OPTION1=VAL1,OPTION2=VAL2,...`
/// Name is in columns 1-7 (if present). Macro type follows.
fn split_macro_line(line: &str) -> (Option<String>, String, String) {
    // Split on whitespace runs (not individual chars) to handle multi-space gaps
    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.is_empty() {
        return (None, String::new(), String::new());
    }

    // Check if first token is a macro keyword
    let macros = ["DFHMSD", "DFHMDI", "DFHMDF", "TITLE", "END"];

    if macros.iter().any(|m| tokens[0].eq_ignore_ascii_case(m)) {
        // No name, first token is the macro type
        let macro_type = tokens[0].to_uppercase();
        let options = if tokens.len() > 1 {
            tokens[1..].join(" ")
        } else {
            String::new()
        };
        (None, macro_type, options)
    } else {
        // First token is the name
        let name = Some(tokens[0].to_string());
        let macro_type = if tokens.len() > 1 {
            tokens[1].to_uppercase()
        } else {
            String::new()
        };
        let options = if tokens.len() > 2 {
            tokens[2..].join(" ")
        } else {
            String::new()
        };
        (name, macro_type, options)
    }
}

// ---------------------------------------------------------------------------
// Option parsing
// ---------------------------------------------------------------------------

/// Parse comma-separated KEY=VALUE options from a BMS macro.
///
/// Handles parenthesized values: `ATTRB=(PROT,BRT)`, `POS=(3,15)`, `SIZE=(24,80)`
/// Handles quoted strings: `INITIAL='text'`
/// Handles ampersand values: `TYPE=&&SYSPARM`
fn parse_options(text: &str) -> HashMap<String, String> {
    let mut opts = HashMap::new();
    let text = text.trim();
    if text.is_empty() {
        return opts;
    }

    // Split on commas, but respect parentheses and quotes
    let mut key = String::new();
    let mut value = String::new();
    let mut in_key = true;
    let mut paren_depth: i32 = 0;
    let mut in_quote = false;

    for ch in text.chars() {
        if in_quote {
            value.push(ch);
            if ch == '\'' {
                in_quote = false;
            }
            continue;
        }

        match ch {
            '\'' => {
                in_quote = true;
                value.push(ch);
            }
            '(' => {
                paren_depth += 1;
                if in_key {
                    // Shouldn't happen, but handle gracefully
                    key.push(ch);
                } else {
                    value.push(ch);
                }
            }
            ')' => {
                paren_depth -= 1;
                if in_key {
                    key.push(ch);
                } else {
                    value.push(ch);
                }
            }
            '=' if in_key && paren_depth == 0 => {
                in_key = false;
            }
            ',' if paren_depth == 0 && !in_quote => {
                // End of this option
                let k = key.trim().to_uppercase();
                let v = value.trim().to_string();
                if !k.is_empty() {
                    opts.insert(k, v);
                }
                key.clear();
                value.clear();
                in_key = true;
            }
            ' ' | '\t' if paren_depth == 0 && !in_quote => {
                // Skip whitespace between options (from continuation joining)
                if !in_key && !value.is_empty() {
                    value.push(ch);
                }
            }
            _ => {
                if in_key {
                    key.push(ch);
                } else {
                    value.push(ch);
                }
            }
        }
    }

    // Flush last option
    let k = key.trim().to_uppercase();
    let v = value.trim().to_string();
    if !k.is_empty() {
        opts.insert(k, v);
    }

    opts
}

/// Parse a parenthesized size/position tuple: "(24,80)" -> (24, 80).
fn parse_size_tuple(s: &str) -> (u8, u8) {
    let cleaned = s.trim_matches(|c| c == '(' || c == ')' || c == ' ');
    let parts: Vec<&str> = cleaned.split(',').collect();
    let a = parts.first().and_then(|p| p.trim().parse().ok()).unwrap_or(24);
    let b = parts.get(1).and_then(|p| p.trim().parse().ok()).unwrap_or(80);
    (a, b)
}

// ---------------------------------------------------------------------------
// Field construction
// ---------------------------------------------------------------------------

/// Build a `BmsField` from parsed options.
fn build_field(name: Option<String>, opts: &HashMap<String, String>) -> BmsField {
    let pos = opts
        .get("POS")
        .map(|s| parse_size_tuple(s))
        .unwrap_or((1, 1));

    let length = opts
        .get("LENGTH")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let attributes = opts
        .get("ATTRB")
        .map(|s| parse_attributes(s))
        .unwrap_or_default();

    let color = opts.get("COLOR").map(|s| parse_color(s));

    let hilight = opts.get("HILIGHT").map(|s| parse_hilight(s));

    let initial = opts
        .get("INITIAL")
        .map(|s| strip_quotes(s));

    let pic_in = opts.get("PICIN").map(|s| strip_quotes(s));
    let pic_out = opts.get("PICOUT").map(|s| strip_quotes(s));

    let justify = opts.get("JUSTIFY").map(|s| parse_justify(s));

    BmsField {
        name,
        pos,
        length,
        attributes,
        color,
        hilight,
        initial,
        pic_in,
        pic_out,
        justify,
        options: opts.clone(),
    }
}

/// Parse ATTRB=(PROT,BRT,NUM,...) into `BmsAttributes`.
fn parse_attributes(s: &str) -> BmsAttributes {
    let inner = s.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
    let mut attrs = BmsAttributes::default();

    for part in parts {
        match part.to_uppercase().as_str() {
            "PROT" => attrs.protected = true,
            "UNPROT" => attrs.unprotected = true,
            "ASKIP" => attrs.auto_skip = true,
            "BRT" => attrs.bright = true,
            "NORM" => attrs.normal = true,
            "DRK" => attrs.dark = true,
            "NUM" => attrs.numeric = true,
            "IC" => attrs.initial_cursor = true,
            "FSET" => attrs.fset = true,
            _ => {} // Unknown attributes silently ignored
        }
    }
    attrs
}

/// Parse COLOR=BLUE etc.
fn parse_color(s: &str) -> BmsColor {
    match s.trim().to_uppercase().as_str() {
        "BLUE" => BmsColor::Blue,
        "RED" => BmsColor::Red,
        "PINK" => BmsColor::Pink,
        "GREEN" => BmsColor::Green,
        "TURQUOISE" => BmsColor::Turquoise,
        "YELLOW" => BmsColor::Yellow,
        "NEUTRAL" => BmsColor::Neutral,
        _ => BmsColor::Default,
    }
}

/// Parse HILIGHT=UNDERLINE etc.
fn parse_hilight(s: &str) -> BmsHilight {
    match s.trim().to_uppercase().as_str() {
        "OFF" => BmsHilight::Off,
        "BLINK" => BmsHilight::Blink,
        "REVERSE" => BmsHilight::Reverse,
        "UNDERLINE" => BmsHilight::Underline,
        _ => BmsHilight::Off,
    }
}

/// Parse JUSTIFY=(RIGHT,ZERO) or JUSTIFY=(RIGHT).
fn parse_justify(s: &str) -> BmsJustify {
    let inner = s.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
    BmsJustify {
        right: parts.iter().any(|p| p.eq_ignore_ascii_case("RIGHT")),
        zero_fill: parts.iter().any(|p| p.eq_ignore_ascii_case("ZERO")),
    }
}

/// Strip surrounding single quotes from a value.
fn strip_quotes(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2 {
        trimmed[1..trimmed.len() - 1].replace("''", "'")
    } else {
        trimmed.to_string()
    }
}

// ---------------------------------------------------------------------------
// Convenience methods
// ---------------------------------------------------------------------------

impl BmsField {
    /// Returns true if this is a named data field (accessible by the program).
    pub fn is_named(&self) -> bool {
        self.name.is_some()
    }

    /// Returns true if this is an input field (user can type).
    pub fn is_input(&self) -> bool {
        self.attributes.unprotected
    }

    /// Returns true if this is display-only (protected or auto-skip).
    pub fn is_display_only(&self) -> bool {
        self.attributes.protected || self.attributes.auto_skip
    }

    /// Returns true if this is a label (unnamed field with INITIAL text).
    pub fn is_label(&self) -> bool {
        self.name.is_none() && self.initial.is_some()
    }
}

impl BmsMap {
    /// Returns only the named data fields (not labels/spacers).
    pub fn data_fields(&self) -> Vec<&BmsField> {
        self.fields.iter().filter(|f| f.is_named()).collect()
    }

    /// Returns only the input (editable) fields.
    pub fn input_fields(&self) -> Vec<&BmsField> {
        self.fields.iter().filter(|f| f.is_named() && f.is_input()).collect()
    }

    /// Returns the field with initial_cursor set, if any.
    pub fn focus_field(&self) -> Option<&BmsField> {
        self.fields.iter().find(|f| f.attributes.initial_cursor)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_mapset() {
        let source = r#"
* Comment line
TESTMS  DFHMSD LANG=COBOL,MODE=INOUT,TYPE=&&SYSPARM
TESTMA  DFHMDI SIZE=(24,80),LINE=1,COLUMN=1
        DFHMDF ATTRB=(ASKIP,NORM),LENGTH=10,POS=(1,1),INITIAL='Hello'
FIELD1  DFHMDF ATTRB=(UNPROT,IC),LENGTH=20,POS=(3,15),COLOR=GREEN
        DFHMSD TYPE=FINAL
        END
"#;
        let mapset = parse_bms(source).unwrap();
        assert_eq!(mapset.name, "TESTMS");
        assert_eq!(mapset.maps.len(), 1);

        let map = &mapset.maps[0];
        assert_eq!(map.name, "TESTMA");
        assert_eq!(map.size, (24, 80));
        assert_eq!(map.fields.len(), 2);

        // First field: unnamed label
        assert!(map.fields[0].name.is_none());
        assert_eq!(map.fields[0].initial.as_deref(), Some("Hello"));
        assert!(map.fields[0].is_label());

        // Second field: named input
        assert_eq!(map.fields[1].name.as_deref(), Some("FIELD1"));
        assert!(map.fields[1].attributes.unprotected);
        assert!(map.fields[1].attributes.initial_cursor);
        assert!(map.fields[1].is_input());
        assert_eq!(map.fields[1].pos, (3, 15));
        assert_eq!(map.fields[1].length, 20);
    }

    #[test]
    fn parse_continuation_lines() {
        let source = r#"
TESTMS  DFHMSD CTRL=(ALARM,FREEKB),                                    -
               LANG=COBOL,                                             -
               MODE=INOUT,                                             -
               TYPE=&&SYSPARM
TESTMA  DFHMDI SIZE=(24,80)
FIELD1  DFHMDF ATTRB=(FSET,IC,NORM,UNPROT),                            -
               COLOR=GREEN,                                            -
               LENGTH=8,                                               -
               POS=(19,43)
        DFHMSD TYPE=FINAL
        END
"#;
        let mapset = parse_bms(source).unwrap();
        assert_eq!(mapset.name, "TESTMS");
        assert_eq!(mapset.lang, "COBOL");

        let field = &mapset.maps[0].fields[0];
        assert_eq!(field.name.as_deref(), Some("FIELD1"));
        assert_eq!(field.length, 8);
        assert_eq!(field.pos, (19, 43));
        assert!(field.attributes.initial_cursor);
        assert!(field.attributes.unprotected);
    }

    #[test]
    fn parse_quoted_initial_with_continuation() {
        let source = r#"
TESTMS  DFHMSD LANG=COBOL,MODE=INOUT,TYPE=&&SYSPARM
TESTMA  DFHMDI SIZE=(24,80)
        DFHMDF ATTRB=(ASKIP,NORM),LENGTH=49,POS=(17,16),               -
               INITIAL='Type your User ID and Password'
        DFHMSD TYPE=FINAL
        END
"#;
        let mapset = parse_bms(source).unwrap();
        let field = &mapset.maps[0].fields[0];
        assert!(field.initial.as_ref().unwrap().contains("Type your User ID"));
    }

    #[test]
    fn parse_attributes() {
        let attrs = super::parse_attributes("(ASKIP,BRT,FSET,NUM)");
        assert!(attrs.auto_skip);
        assert!(attrs.bright);
        assert!(attrs.fset);
        assert!(attrs.numeric);
        assert!(!attrs.protected);
        assert!(!attrs.unprotected);
    }

    #[test]
    fn parse_options_map() {
        let opts = super::parse_options("LANG=COBOL,MODE=INOUT,STORAGE=AUTO");
        assert_eq!(opts.get("LANG").unwrap(), "COBOL");
        assert_eq!(opts.get("MODE").unwrap(), "INOUT");
        assert_eq!(opts.get("STORAGE").unwrap(), "AUTO");
    }

    #[test]
    fn parse_options_with_parens() {
        let opts = super::parse_options("ATTRB=(PROT,BRT),POS=(3,15),LENGTH=10");
        assert_eq!(opts.get("ATTRB").unwrap(), "(PROT,BRT)");
        assert_eq!(opts.get("POS").unwrap(), "(3,15)");
        assert_eq!(opts.get("LENGTH").unwrap(), "10");
    }

    #[test]
    fn parse_carddemo_login() {
        let bms_path = Path::new(env!("HOME"))
            .join("workspace/aws-mainframe-modernization-carddemo/app/bms/COSGN00.bms");
        if !bms_path.exists() {
            eprintln!("Skipping CardDemo test: {bms_path:?} not found");
            return;
        }
        let mapset = parse_bms_file(&bms_path).unwrap();
        assert_eq!(mapset.name, "COSGN00");
        assert_eq!(mapset.lang, "COBOL");
        assert_eq!(mapset.maps.len(), 1);

        let map = &mapset.maps[0];
        assert_eq!(map.name, "COSGN0A");
        assert_eq!(map.size, (24, 80));

        // Should have named data fields
        let data_fields = map.data_fields();
        assert!(!data_fields.is_empty(), "expected named data fields");

        // Check known fields
        let field_names: Vec<&str> = data_fields.iter()
            .filter_map(|f| f.name.as_deref())
            .collect();
        assert!(field_names.contains(&"USERID"), "missing USERID field: {field_names:?}");
        assert!(field_names.contains(&"PASSWD"), "missing PASSWD field: {field_names:?}");
        assert!(field_names.contains(&"ERRMSG"), "missing ERRMSG field: {field_names:?}");

        // USERID should be input with initial cursor
        let userid = data_fields.iter().find(|f| f.name.as_deref() == Some("USERID")).unwrap();
        assert!(userid.is_input());
        assert!(userid.attributes.initial_cursor);
        assert_eq!(userid.length, 8);
        assert_eq!(userid.pos, (19, 43));

        // PASSWD should be dark (password field)
        let passwd = data_fields.iter().find(|f| f.name.as_deref() == Some("PASSWD")).unwrap();
        assert!(passwd.is_input());
        assert!(passwd.attributes.dark);

        // ERRMSG should be display-only
        let errmsg = data_fields.iter().find(|f| f.name.as_deref() == Some("ERRMSG")).unwrap();
        assert!(!errmsg.is_input());

        // Should have labels
        let labels: Vec<&BmsField> = map.fields.iter().filter(|f| f.is_label()).collect();
        assert!(!labels.is_empty(), "expected label fields");

        // Focus field should be USERID
        let focus = map.focus_field();
        assert!(focus.is_some());
        assert_eq!(focus.unwrap().name.as_deref(), Some("USERID"));

        eprintln!("CardDemo login: {} total fields, {} data fields, {} labels",
            map.fields.len(), data_fields.len(), labels.len());
    }

    #[test]
    fn field_classification() {
        let label = BmsField {
            name: None,
            pos: (1, 1),
            length: 10,
            attributes: BmsAttributes { auto_skip: true, ..Default::default() },
            color: None,
            hilight: None,
            initial: Some("Header".to_string()),
            pic_in: None,
            pic_out: None,
            justify: None,
            options: HashMap::new(),
        };
        assert!(label.is_label());
        assert!(label.is_display_only());
        assert!(!label.is_input());
        assert!(!label.is_named());

        let input = BmsField {
            name: Some("USERID".to_string()),
            pos: (19, 43),
            length: 8,
            attributes: BmsAttributes { unprotected: true, initial_cursor: true, ..Default::default() },
            color: Some(BmsColor::Green),
            hilight: None,
            initial: None,
            pic_in: None,
            pic_out: None,
            justify: None,
            options: HashMap::new(),
        };
        assert!(input.is_named());
        assert!(input.is_input());
        assert!(!input.is_display_only());
        assert!(!input.is_label());
    }

}
