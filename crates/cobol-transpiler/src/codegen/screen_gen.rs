//! Screen DSL generator -- converts BMS map definitions to `.screen` DSL files.
//!
//! Takes a parsed `BmsMapset` and produces:
//! - `.screen` files: view definitions with fields, groups, labels, actions
//! - `.schema` files: companion data schemas for the screen fields
//!
//! The output follows the ScreenDSL grammar defined in
//! `grammar/nexflow/ScreenDSL.g4`.

use std::fmt::Write;
use std::path::Path;

use crate::parser::bms_parser::{BmsField, BmsMap, BmsMapset};

// ---------------------------------------------------------------------------
// Output types
// ---------------------------------------------------------------------------

/// A generated DSL file (screen or schema).
#[derive(Debug, Clone)]
pub struct GeneratedDslFile {
    /// Relative file path (e.g., "screen/COSGN0A.screen").
    pub path: String,
    /// File content.
    pub content: String,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Generate `.screen` and `.schema` DSL files from a parsed BMS mapset.
///
/// Returns one `.screen` file and one `.schema` file per map in the mapset.
pub fn generate_screen_dsl(mapset: &BmsMapset) -> Vec<GeneratedDslFile> {
    let mut files = Vec::new();

    for map in &mapset.maps {
        // Generate companion schema
        let schema_content = generate_schema(map);
        files.push(GeneratedDslFile {
            path: format!("schema/{}.schema", map.name),
            content: schema_content,
        });

        // Generate screen view
        let screen_content = generate_view(map, mapset);
        files.push(GeneratedDslFile {
            path: format!("screen/{}.screen", map.name),
            content: screen_content,
        });
    }

    files
}

/// Write generated DSL files to an output directory.
pub fn write_dsl_files(files: &[GeneratedDslFile], output_dir: &Path) -> std::io::Result<()> {
    for file in files {
        let path = output_dir.join(&file.path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, &file.content)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Schema generation
// ---------------------------------------------------------------------------

/// Generate a `.schema` file for a BMS map's data fields.
fn generate_schema(map: &BmsMap) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "// Generated from BMS map: {}", map.name);
    let _ = writeln!(out, "// Screen size: {}x{}", map.size.0, map.size.1);
    let _ = writeln!(out);
    let _ = writeln!(out, "schema {}", map.name);
    let _ = writeln!(out, "    pattern master_data");
    let _ = writeln!(out);

    // Identity: use the first input field as identity (if any)
    let input_fields = map.input_fields();
    if let Some(first_input) = input_fields.first() {
        if let Some(ref name) = first_input.name {
            let _ = writeln!(out, "    identity");
            let _ = writeln!(out, "        {} {} required", name, infer_schema_type(first_input));
            let _ = writeln!(out, "    end");
            let _ = writeln!(out);
        }
    }

    // Fields block: all named data fields
    let data_fields = map.data_fields();
    if !data_fields.is_empty() {
        let _ = writeln!(out, "    fields");
        for field in &data_fields {
            if let Some(ref name) = field.name {
                let field_type = infer_schema_type(field);
                let required = if field.is_input() { " required" } else { "" };
                let _ = writeln!(out, "        // BMS: POS=({},{}) LENGTH={}", field.pos.0, field.pos.1, field.length);
                let _ = writeln!(out, "        {name} {field_type}{required}");
            }
        }
        let _ = writeln!(out, "    end");
    }

    // Constraints: add enum constraints for fields with known values
    let fields_with_pic = data_fields.iter()
        .filter(|f| f.pic_in.is_some() || f.pic_out.is_some())
        .collect::<Vec<_>>();
    if !fields_with_pic.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "    // Note: PICIN/PICOUT hints available for type refinement");
        for field in &fields_with_pic {
            if let Some(ref name) = field.name {
                if let Some(ref pic_in) = field.pic_in {
                    let _ = writeln!(out, "    // {name} PICIN='{pic_in}'");
                }
                if let Some(ref pic_out) = field.pic_out {
                    let _ = writeln!(out, "    // {name} PICOUT='{pic_out}'");
                }
            }
        }
    }

    let _ = writeln!(out, "end");
    out
}

/// Infer a schema field type from BMS field attributes.
fn infer_schema_type(field: &BmsField) -> String {
    // If PICIN is numeric, use decimal/integer
    if let Some(ref pic) = field.pic_in {
        if pic.chars().all(|c| c == '9' || c == 'V' || c == ',' || c == '.') {
            if pic.contains('V') || pic.contains('.') {
                let digits = pic.chars().filter(|c| *c == '9').count();
                let decimals = pic.chars().skip_while(|c| *c != 'V' && *c != '.').filter(|c| *c == '9').count();
                return format!("decimal({}, {})", digits, decimals);
            }
            let digits = pic.chars().filter(|c| *c == '9').count();
            return format!("integer({})", digits);
        }
    }

    // If numeric attribute, use integer
    if field.attributes.numeric {
        return format!("integer({})", field.length);
    }

    // Default: string
    if field.length <= 1 {
        format!("char({})", field.length)
    } else {
        format!("string({})", field.length)
    }
}

// ---------------------------------------------------------------------------
// View generation
// ---------------------------------------------------------------------------

/// Generate a `.screen` DSL file for a BMS map.
fn generate_view(map: &BmsMap, mapset: &BmsMapset) -> String {
    let mut out = String::new();

    // Header comment
    let _ = writeln!(out, "// Generated from BMS mapset: {}, map: {}", mapset.name, map.name);
    let _ = writeln!(out, "// Screen size: {}x{}", map.size.0, map.size.1);
    let _ = writeln!(out);

    // Import companion schema
    let _ = writeln!(out, "import ../schema/{}.schema", map.name);
    let _ = writeln!(out);

    // View definition
    let title = derive_title(map);
    let _ = writeln!(out, "view {} \"{}\"", map.name, title);

    // Schema reference
    let _ = writeln!(out, "    schema {}", map.name);

    // Focus field
    if let Some(focus) = map.focus_field() {
        if let Some(ref name) = focus.name {
            let _ = writeln!(out, "    focus {name}");
        }
    }
    let _ = writeln!(out);

    // Group fields by screen region
    let groups = group_fields(map);

    for group in &groups {
        match group {
            FieldGroup::Header(fields) => {
                emit_header_group(&mut out, fields);
            }
            FieldGroup::LabelFieldPairs(pairs) => {
                emit_field_group(&mut out, pairs);
            }
            FieldGroup::StandaloneLabels(labels) => {
                emit_label_group(&mut out, labels);
            }
            FieldGroup::DataFields(fields) => {
                emit_data_group(&mut out, fields);
            }
            FieldGroup::Footer(fields) => {
                emit_footer_group(&mut out, fields);
            }
        }
    }

    // Actions from footer key hints
    let _ = writeln!(out);
    emit_actions(&mut out, map);

    let _ = writeln!(out, "end");
    out
}

// ---------------------------------------------------------------------------
// Field grouping
// ---------------------------------------------------------------------------

/// Logical grouping of BMS fields for screen layout.
enum FieldGroup<'a> {
    /// Header row fields (row 1-3, typically tran/date/time/program).
    Header(Vec<&'a BmsField>),
    /// Adjacent label + data field pairs.
    LabelFieldPairs(Vec<(&'a BmsField, &'a BmsField)>),
    /// Standalone label fields (decorative text, borders).
    StandaloneLabels(Vec<&'a BmsField>),
    /// Data fields without adjacent labels.
    DataFields(Vec<&'a BmsField>),
    /// Footer row fields (last 1-2 rows, typically key legends).
    Footer(Vec<&'a BmsField>),
}

/// Group BMS fields into logical screen regions.
fn group_fields(map: &BmsMap) -> Vec<FieldGroup<'_>> {
    let mut groups: Vec<FieldGroup<'_>> = Vec::new();
    let rows = map.size.0;

    // Header: rows 1-3
    let header_fields: Vec<&BmsField> = map.fields.iter()
        .filter(|f| f.pos.0 <= 3)
        .collect();
    if !header_fields.is_empty() {
        groups.push(FieldGroup::Header(header_fields));
    }

    // Body: rows 4 to (rows-2)
    let body_fields: Vec<&BmsField> = map.fields.iter()
        .filter(|f| f.pos.0 > 3 && f.pos.0 < rows - 1)
        .collect();

    // Detect label-field pairs: unnamed label immediately before named field on same row
    let mut paired = Vec::new();
    let mut standalone_labels = Vec::new();
    let mut unpaired_data = Vec::new();
    let mut used_indices = std::collections::HashSet::new();

    for (i, field) in body_fields.iter().enumerate() {
        if used_indices.contains(&i) {
            continue;
        }

        if field.is_label() {
            // Check if next field on same/adjacent position is a named data field
            if let Some((j, data_field)) = body_fields.iter().enumerate().skip(i + 1).find(|(_, f)| {
                f.is_named() && f.pos.0 == field.pos.0
            }) {
                paired.push((*field, *data_field));
                used_indices.insert(i);
                used_indices.insert(j);
            } else {
                standalone_labels.push(*field);
                used_indices.insert(i);
            }
        } else if field.is_named() {
            unpaired_data.push(*field);
            used_indices.insert(i);
        }
    }

    if !standalone_labels.is_empty() {
        groups.push(FieldGroup::StandaloneLabels(standalone_labels));
    }
    if !paired.is_empty() {
        groups.push(FieldGroup::LabelFieldPairs(paired));
    }
    if !unpaired_data.is_empty() {
        groups.push(FieldGroup::DataFields(unpaired_data));
    }

    // Footer: last 2 rows
    let footer_fields: Vec<&BmsField> = map.fields.iter()
        .filter(|f| f.pos.0 >= rows - 1)
        .collect();
    if !footer_fields.is_empty() {
        groups.push(FieldGroup::Footer(footer_fields));
    }

    groups
}

// ---------------------------------------------------------------------------
// Emission helpers
// ---------------------------------------------------------------------------

fn emit_header_group(out: &mut String, fields: &[&BmsField]) {
    let _ = writeln!(out, "    group \"Header\"");
    for field in fields {
        if let Some(ref name) = field.name {
            let _ = writeln!(out, "        field {name} readonly");
        }
    }
    let _ = writeln!(out, "    end");
    let _ = writeln!(out);
}

fn emit_field_group(out: &mut String, pairs: &[(&BmsField, &BmsField)]) {
    let _ = writeln!(out, "    group \"Details\"");
    for (label, data) in pairs {
        let label_text = label.initial.as_deref().unwrap_or("").trim();
        let label_text = label_text.trim_end_matches(':').trim();
        if let Some(ref name) = data.name {
            let mods = field_modifiers(data);
            let _ = writeln!(out, "        field {name}{mods} \"{label_text}\"");
        }
    }
    let _ = writeln!(out, "    end");
    let _ = writeln!(out);
}

fn emit_label_group(out: &mut String, labels: &[&BmsField]) {
    // Only emit non-trivial labels (skip decorative borders, spacers)
    let meaningful: Vec<&&BmsField> = labels.iter()
        .filter(|l| {
            l.initial.as_ref().map_or(false, |t| {
                let trimmed = t.trim();
                !trimmed.is_empty()
                    && !trimmed.chars().all(|c| "=+-|%$#*~".contains(c))
                    && trimmed.len() > 2
            })
        })
        .collect();

    if meaningful.is_empty() {
        return;
    }

    for label in &meaningful {
        if let Some(ref text) = label.initial {
            let cleaned = text.trim();
            let _ = writeln!(out, "    label \"{cleaned}\"");
        }
    }
    let _ = writeln!(out);
}

fn emit_data_group(out: &mut String, fields: &[&BmsField]) {
    if fields.is_empty() {
        return;
    }
    let _ = writeln!(out, "    group \"Fields\"");
    for field in fields {
        if let Some(ref name) = field.name {
            let mods = field_modifiers(field);
            let _ = writeln!(out, "        field {name}{mods}");
        }
    }
    let _ = writeln!(out, "    end");
    let _ = writeln!(out);
}

fn emit_footer_group(out: &mut String, fields: &[&BmsField]) {
    // Extract error message field
    for field in fields {
        if let Some(ref name) = field.name {
            let mods = field_modifiers(field);
            let _ = writeln!(out, "    field {name}{mods}");
        }
    }
}

/// Generate field modifiers based on BMS attributes.
fn field_modifiers(field: &BmsField) -> String {
    let mut mods = Vec::new();

    if field.is_display_only() {
        mods.push("readonly");
    }
    if field.attributes.bright {
        mods.push("emphasis");
    }
    if field.attributes.dark {
        mods.push("hidden");
    }

    if mods.is_empty() {
        String::new()
    } else {
        format!(" {}", mods.join(" "))
    }
}

/// Derive a human-readable title from the map name or content.
fn derive_title(map: &BmsMap) -> String {
    // Look for a title field in the first few rows
    for field in &map.fields {
        if field.pos.0 <= 2 {
            if let Some(ref name) = field.name {
                if name.starts_with("TITLE") {
                    // Title fields are usually filled at runtime
                    continue;
                }
            }
            // Large centered label is likely the screen title
            if field.is_label() && field.length > 20 {
                if let Some(ref text) = field.initial {
                    let trimmed = text.trim();
                    if !trimmed.is_empty()
                        && !trimmed.chars().all(|c| "=+-|%$#*~".contains(c))
                    {
                        return trimmed.to_string();
                    }
                }
            }
        }
    }

    // Fallback: use map name
    map.name.clone()
}

/// Emit action declarations from footer key legends.
fn emit_actions(out: &mut String, map: &BmsMap) {
    let rows = map.size.0;

    // Look for key legend text in the last 2 rows
    let mut found_keys = Vec::new();
    for field in &map.fields {
        if field.pos.0 >= rows - 1 {
            if let Some(ref text) = field.initial {
                // Parse key hints like "ENTER=Sign-on  F3=Exit  F10=Help"
                for part in text.split_whitespace() {
                    if let Some((key, action)) = parse_key_hint(part) {
                        found_keys.push((key, action));
                    }
                }
            }
        }
    }

    if found_keys.is_empty() {
        // Default actions
        let _ = writeln!(out, "    action submit \"Submit\" -> submit");
        let _ = writeln!(out, "    action cancel \"Exit\" -> back");
    } else {
        for (key, action_label) in &found_keys {
            let (intent, target) = map_key_to_intent(key, action_label);
            let _ = writeln!(out, "    action {intent} \"{action_label}\" -> {target}");
        }
    }
}

/// Parse a key hint like "ENTER=Sign-on" or "F3=Exit".
fn parse_key_hint(text: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = text.splitn(2, '=').collect();
    if parts.len() == 2 {
        let key = parts[0].trim().to_uppercase();
        let action = parts[1].trim().to_string();
        if (key.starts_with('F') && key[1..].parse::<u8>().is_ok())
            || key == "ENTER"
            || key == "CLEAR"
            || key == "PF1"
        {
            return Some((key, action));
        }
    }
    None
}

/// Map a BMS function key to a ScreenDSL action intent and target.
fn map_key_to_intent(key: &str, label: &str) -> (String, String) {
    let label_lower = label.to_lowercase();

    // Infer intent from label text
    if label_lower.contains("exit") || label_lower.contains("quit") {
        return ("cancel".to_string(), "back".to_string());
    }
    if label_lower.contains("help") {
        return ("help".to_string(), "help".to_string());
    }
    if label_lower.contains("sign") || label_lower.contains("login") || label_lower.contains("submit") {
        return ("submit".to_string(), "submit".to_string());
    }
    if label_lower.contains("next") || label_lower.contains("forward") || label_lower.contains("down") {
        return ("paginate".to_string(), "next".to_string());
    }
    if label_lower.contains("prev") || label_lower.contains("back") || label_lower.contains("up") {
        return ("paginate".to_string(), "previous".to_string());
    }

    // Infer from key
    match key {
        "ENTER" => ("submit".to_string(), "submit".to_string()),
        "F3" => ("cancel".to_string(), "back".to_string()),
        "F1" => ("help".to_string(), "help".to_string()),
        "F7" => ("paginate".to_string(), "previous".to_string()),
        "F8" => ("paginate".to_string(), "next".to_string()),
        _ => ("navigate".to_string(), label.to_lowercase().replace(' ', "_")),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::bms_parser;
    use std::path::Path;

    #[test]
    fn generate_simple_screen() {
        let source = r#"
* Test BMS
TESTMS  DFHMSD LANG=COBOL,MODE=INOUT,TYPE=&&SYSPARM
TESTMA  DFHMDI SIZE=(24,80),LINE=1,COLUMN=1
        DFHMDF ATTRB=(ASKIP,NORM),LENGTH=12,POS=(5,10),INITIAL='Account ID:'
ACCTID  DFHMDF ATTRB=(UNPROT,IC),LENGTH=10,POS=(5,23),COLOR=GREEN
        DFHMDF ATTRB=(ASKIP,NORM),LENGTH=8,POS=(7,10),INITIAL='Balance:'
BALANCE DFHMDF ATTRB=(PROT,BRT),LENGTH=12,POS=(7,23)
ERRMSG  DFHMDF ATTRB=(ASKIP,BRT),COLOR=RED,LENGTH=60,POS=(23,1)
        DFHMDF ATTRB=(ASKIP),LENGTH=22,POS=(24,1),INITIAL='ENTER=Submit  F3=Exit'
        DFHMSD TYPE=FINAL
        END
"#;
        let mapset = bms_parser::parse_bms(source).unwrap();
        let files = generate_screen_dsl(&mapset);

        assert_eq!(files.len(), 2, "expected schema + screen files");

        // Check schema
        let schema = files.iter().find(|f| f.path.ends_with(".schema")).unwrap();
        assert!(schema.content.contains("schema TESTMA"), "missing schema name: {}", schema.content);
        assert!(schema.content.contains("ACCTID"), "missing ACCTID field: {}", schema.content);
        assert!(schema.content.contains("BALANCE"), "missing BALANCE field: {}", schema.content);

        // Check screen
        let screen = files.iter().find(|f| f.path.ends_with(".screen")).unwrap();
        assert!(screen.content.contains("view TESTMA"), "missing view: {}", screen.content);
        assert!(screen.content.contains("schema TESTMA"), "missing schema ref: {}", screen.content);
        assert!(screen.content.contains("focus ACCTID"), "missing focus: {}", screen.content);
        assert!(screen.content.contains("ACCTID"), "missing ACCTID: {}", screen.content);
        assert!(screen.content.contains("BALANCE"), "missing BALANCE: {}", screen.content);
        assert!(screen.content.contains("readonly"), "missing readonly for BALANCE: {}", screen.content);
        assert!(screen.content.contains("action submit"), "missing submit action: {}", screen.content);
        assert!(screen.content.contains("action cancel"), "missing cancel action: {}", screen.content);
    }

    #[test]
    fn generate_carddemo_login() {
        let bms_path = Path::new(env!("HOME"))
            .join("workspace/aws-mainframe-modernization-carddemo/app/bms/COSGN00.bms");
        if !bms_path.exists() {
            eprintln!("Skipping: CardDemo not found");
            return;
        }

        let mapset = bms_parser::parse_bms_file(&bms_path).unwrap();
        let files = generate_screen_dsl(&mapset);

        assert_eq!(files.len(), 2);

        let screen = files.iter().find(|f| f.path.ends_with(".screen")).unwrap();
        assert!(screen.content.contains("view COSGN0A"), "missing view name");
        assert!(screen.content.contains("focus USERID"), "missing focus USERID");
        assert!(screen.content.contains("PASSWD"), "missing PASSWD field");

        // PASSWD should be hidden (DRK attribute)
        assert!(screen.content.contains("PASSWD hidden"), "PASSWD should be hidden (dark): {}", screen.content);

        // Should have actions from footer
        assert!(screen.content.contains("action"), "missing actions");

        eprintln!("Generated login screen:\n{}", screen.content);
        eprintln!("Generated schema:\n{}", files[0].content);
    }

    #[test]
    fn parse_key_hints() {
        assert_eq!(
            parse_key_hint("ENTER=Sign-on"),
            Some(("ENTER".to_string(), "Sign-on".to_string()))
        );
        assert_eq!(
            parse_key_hint("F3=Exit"),
            Some(("F3".to_string(), "Exit".to_string()))
        );
        assert_eq!(parse_key_hint("not-a-key"), None);
    }
}
