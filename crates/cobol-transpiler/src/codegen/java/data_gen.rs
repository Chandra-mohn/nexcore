//! Java data division code generator.
//!
//! Generates a Java `WorkingStorage` class from COBOL DATA DIVISION entries.
//! Each 01-level record becomes flattened fields in the class, with appropriate
//! Java types and initialization.

use std::collections::HashSet;

use crate::ast::{DataEntry, FileDescription, FileOrganization, Literal};
use crate::codegen::java::java_writer::JavaWriter;
use crate::symbol_table::{resolve_type, RustType};

/// Generate the Java `WorkingStorage` class and its constructor.
pub fn generate_working_storage(
    w: &mut JavaWriter,
    records: &[DataEntry],
    file_section: &[FileDescription],
    has_sql: bool,
    program_id: &str,
) {
    // Pre-pass: collect all leaf field names to detect duplicates
    let mut name_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for record in records {
        collect_leaf_names(record, &mut name_counts);
    }
    for fd in file_section {
        for record in &fd.records {
            collect_leaf_names(record, &mut name_counts);
        }
    }
    let duplicates: HashSet<String> = name_counts
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(name, _)| name.clone())
        .collect();

    let mut filler_counter = 0u32;

    // Class annotation
    w.line(&format!("@Cobol(program = \"{program_id}\")"));
    w.open_block("public class WorkingStorage {");

    // FILE SECTION: file handle fields
    for fd in file_section {
        let fname = cobol_to_java_name(&fd.file_name, "");
        let file_type = java_file_handle_type(fd);
        w.line(&format!("public {file_type} {fname};"));
    }

    // FILE SECTION: record fields
    for fd in file_section {
        for record in &fd.records {
            generate_ws_record(w, record, &duplicates, &mut filler_counter);
        }
    }

    // WORKING-STORAGE SECTION fields
    for record in records {
        generate_ws_record(w, record, &duplicates, &mut filler_counter);
    }

    // Level-66 RENAMES fields
    for fd in file_section {
        for record in &fd.records {
            generate_renames_fields(w, record);
        }
    }
    for record in records {
        generate_renames_fields(w, record);
    }

    // Auto-inject SQLCA when EXEC SQL is present
    if has_sql {
        w.line("/** SQL Communication Area (auto-injected). */");
        w.line("public Sqlca sqlca;");
        w.line("public CobolDecimal sqlcode;");
    }

    w.blank_line();

    // Generate constructor
    filler_counter = 0;
    w.open_block("public WorkingStorage() {");

    // FILE SECTION: file handle inits
    for fd in file_section {
        let fname = cobol_to_java_name(&fd.file_name, "");
        let init = java_file_handle_init(fd);
        w.line(&format!("{fname} = {init};"));
    }

    // FILE SECTION: record inits
    for fd in file_section {
        for record in &fd.records {
            generate_ws_record_init(w, record, &duplicates, &mut filler_counter);
        }
    }

    // WORKING-STORAGE SECTION inits
    for record in records {
        generate_ws_record_init(w, record, &duplicates, &mut filler_counter);
    }

    // Auto-inject SQLCA init
    if has_sql {
        w.line("sqlca = new Sqlca();");
        w.line("sqlcode = new CobolDecimal(9, 0, true);");
    }

    // Level-66 RENAMES inits
    for fd in file_section {
        for record in &fd.records {
            generate_renames_field_inits(w, record);
        }
    }
    for record in records {
        generate_renames_field_inits(w, record);
    }

    w.close_block("}"); // constructor
    w.close_block("}"); // class
}

// --- Record generation ---

fn generate_ws_record(
    w: &mut JavaWriter,
    record: &DataEntry,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
) {
    if record.level == 77 {
        generate_field(w, record, "", duplicates, filler_counter, &[]);
    } else if record.level == 1 {
        if has_data_children(record) {
            let group_size = compute_group_byte_length(record);
            if group_size > 0 {
                let group_name = cobol_to_java_name(&record.name, "");
                w.line(&format!(
                    "public CobolString {group_name}; /* GROUP {group_size} bytes */"
                ));
            }
            generate_group_fields(w, record, &record.name, duplicates, filler_counter, &[]);
        } else {
            generate_field(w, record, "", duplicates, filler_counter, &[]);
        }
    }
}

fn generate_ws_record_init(
    w: &mut JavaWriter,
    record: &DataEntry,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
) {
    if record.level == 77 {
        generate_field_init(w, record, "", duplicates, filler_counter, &[]);
    } else if record.level == 1 {
        if has_data_children(record) {
            let group_size = compute_group_byte_length(record);
            if group_size > 0 {
                let group_name = cobol_to_java_name(&record.name, "");
                w.line(&format!(
                    "{group_name} = new CobolString({group_size});"
                ));
            }
            generate_group_field_inits(w, record, &record.name, duplicates, filler_counter, &[]);
        } else {
            generate_field_init(w, record, "", duplicates, filler_counter, &[]);
        }
    }
}

// --- Field generation ---

fn generate_field(
    w: &mut JavaWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    let field_name = resolve_field_name(entry, parent_group, duplicates, filler_counter);

    // Emit @Cobol annotation
    if let Some(attr) = build_java_field_annotation(entry) {
        w.line(&attr);
    }

    // REDEFINES: shared byte storage
    if entry.redefines.is_some() {
        let size = entry.byte_length.unwrap_or(0);
        w.line(&format!(
            "public RedefinesGroup {field_name}; /* REDEFINES, {size} bytes */"
        ));
        return;
    }

    let resolved = resolve_type(entry);
    let java_type = java_type_string(&resolved.rust_type);

    // OCCURS DEPENDING ON: variable-length array
    if entry.occurs.is_some() && entry.occurs_depending.is_some() {
        let inner = format!("CobolVarArray<{java_type}>");
        let wrapped = wrap_ancestor_occurs(&inner, ancestor_occurs);
        w.line(&format!(
            "public {wrapped} {field_name}; /* OCCURS DEPENDING ON */"
        ));
        return;
    }

    // OCCURS: fixed-size array
    if let Some(count) = entry.occurs {
        let inner = format!("CobolArray<{java_type}>");
        let wrapped = wrap_ancestor_occurs(&inner, ancestor_occurs);
        w.line(&format!(
            "public {wrapped} {field_name}; /* OCCURS {count} */"
        ));
        for idx_name in &entry.index_names {
            let idx_field = cobol_to_java_name(idx_name, "");
            w.line(&format!(
                "public long {idx_field}; /* INDEXED BY {idx_name} */"
            ));
        }
        return;
    }

    // Plain field
    let wrapped = wrap_ancestor_occurs(&java_type, ancestor_occurs);
    w.line(&format!("public {wrapped} {field_name};"));
}

fn generate_field_init(
    w: &mut JavaWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    let field_name = resolve_field_name(entry, parent_group, duplicates, filler_counter);

    if entry.redefines.is_some() {
        let size = entry.byte_length.unwrap_or(0);
        w.line(&format!("{field_name} = new RedefinesGroup({size});"));
        return;
    }

    let resolved = resolve_type(entry);

    if entry.occurs.is_some() && entry.occurs_depending.is_some() {
        let count = entry.occurs.unwrap_or(1);
        let jt = java_type_string(&resolved.rust_type);
        let init = java_field_init_expr(entry, &resolved.rust_type);
        w.line(&format!(
            "{field_name} = new CobolVarArray<{jt}>({count}, () -> {init});"
        ));
        return;
    }

    if let Some(count) = entry.occurs {
        let jt = java_type_string(&resolved.rust_type);
        let init = java_field_init_expr(entry, &resolved.rust_type);
        // Wrap inner type with ancestor OCCURS for proper nesting
        let inner_type = wrap_ancestor_occurs(&jt, ancestor_occurs);
        let inner_init = wrap_ancestor_occurs_init(&init, &jt, ancestor_occurs);
        w.line(&format!(
            "{field_name} = new CobolArray<{inner_type}>({count}, () -> {inner_init});"
        ));
        for idx_name in &entry.index_names {
            let idx_field = cobol_to_java_name(idx_name, "");
            w.line(&format!("{idx_field} = 0L;"));
        }
        return;
    }

    let init = java_field_init_expr(entry, &resolved.rust_type);
    if ancestor_occurs.is_empty() {
        w.line(&format!("{field_name} = {init};"));
    } else {
        // Wrap init in ancestor CobolArray factories
        let jt = java_type_string(&resolved.rust_type);
        let wrapped = wrap_ancestor_occurs_init(&init, &jt, ancestor_occurs);
        w.line(&format!("{field_name} = {wrapped};"));
    }
}

// --- Group handling ---

fn generate_group_fields(
    w: &mut JavaWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        if child.redefines.is_some() {
            generate_field(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
            continue;
        }
        if is_true_group(child) {
            let sub_size = compute_group_byte_length(child);
            if sub_size > 0 {
                let sub_name = resolve_field_name(child, parent_group, duplicates, filler_counter);
                let sub_type = wrap_ancestor_occurs("CobolString", ancestor_occurs);
                w.line(&format!(
                    "public {sub_type} {sub_name}; /* sub-GROUP {sub_size} bytes */"
                ));
            }
            let mut child_ancestors = ancestor_occurs.to_vec();
            if let Some(count) = child.occurs {
                child_ancestors.push(count as usize);
            }
            generate_group_fields(
                w,
                child,
                &child.name,
                duplicates,
                filler_counter,
                &child_ancestors,
            );
        } else {
            generate_field(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
        }
    }
}

fn generate_group_field_inits(
    w: &mut JavaWriter,
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
    ancestor_occurs: &[usize],
) {
    for child in &entry.children {
        if child.level == 88 || child.level == 66 {
            continue;
        }
        if child.redefines.is_some() {
            generate_field_init(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
            continue;
        }
        if is_true_group(child) {
            let sub_size = compute_group_byte_length(child);
            if sub_size > 0 {
                let sub_name = resolve_field_name(child, parent_group, duplicates, filler_counter);
                w.line(&format!("{sub_name} = new CobolString({sub_size});"));
            }
            let mut child_ancestors = ancestor_occurs.to_vec();
            if let Some(count) = child.occurs {
                child_ancestors.push(count as usize);
            }
            generate_group_field_inits(
                w,
                child,
                &child.name,
                duplicates,
                filler_counter,
                &child_ancestors,
            );
        } else {
            generate_field_init(w, child, parent_group, duplicates, filler_counter, ancestor_occurs);
        }
    }
}

// --- Type mapping ---

fn java_type_string(rt: &RustType) -> String {
    match rt {
        RustType::PackedDecimal { .. } | RustType::DisplayNumeric { .. } => {
            "CobolDecimal".to_string()
        }
        RustType::PicX { .. } => "CobolString".to_string(),
        RustType::PicA { .. } => "CobolString".to_string(),
        RustType::AlphanumericEdited { .. } => "CobolString".to_string(),
        RustType::CompBinary { .. } => "CobolBinary".to_string(),
        RustType::Float32 => "float".to_string(),
        RustType::Float64 => "double".to_string(),
        RustType::Index => "long".to_string(),
        RustType::Pointer => "long".to_string(),
        RustType::Group => "byte[]".to_string(),
    }
}

fn java_field_init_expr(entry: &DataEntry, rt: &RustType) -> String {
    // Check for VALUE clause
    if let Some(ref value) = entry.value {
        return java_value_to_init(value, rt);
    }

    // Default initialization
    match rt {
        RustType::PackedDecimal {
            precision,
            scale,
            signed,
        }
        | RustType::DisplayNumeric {
            precision,
            scale,
            signed,
        } => {
            format!("new CobolDecimal({precision}, {scale}, {signed})")
        }
        RustType::PicX { length } => format!("new CobolString({length})"),
        RustType::PicA { length } => format!("new CobolString({length})"),
        RustType::AlphanumericEdited { length } => format!("new CobolString({length})"),
        RustType::CompBinary {
            precision,
            scale,
            signed,
            pic_limited,
        } => {
            format!("new CobolBinary({precision}, {scale}, {signed}, {pic_limited})")
        }
        RustType::Float32 => "0.0f".to_string(),
        RustType::Float64 => "0.0d".to_string(),
        RustType::Index => "0L".to_string(),
        RustType::Pointer => "0L".to_string(),
        RustType::Group => "new byte[0]".to_string(),
    }
}

fn java_value_to_init(lit: &Literal, rt: &RustType) -> String {
    match (lit, rt) {
        (
            Literal::Numeric(n),
            RustType::PackedDecimal {
                precision,
                scale,
                signed,
            }
            | RustType::DisplayNumeric {
                precision,
                scale,
                signed,
            },
        ) => {
            format!(
                "new CobolDecimal({precision}, {scale}, {signed}).set(\"{n}\")"
            )
        }
        (
            Literal::Numeric(n),
            RustType::CompBinary {
                precision,
                scale,
                signed,
                pic_limited,
            },
        ) => {
            format!(
                "new CobolBinary({precision}, {scale}, {signed}, {pic_limited}).set(\"{n}\")"
            )
        }
        (Literal::Numeric(n), RustType::Float32) => format!("{n}f"),
        (Literal::Numeric(n), RustType::Float64) => format!("{n}d"),
        (Literal::Numeric(n), RustType::Index) => format!("{n}L"),
        (Literal::Alphanumeric(s), RustType::PicX { length })
        | (Literal::Alphanumeric(s), RustType::PicA { length })
        | (Literal::Alphanumeric(s), RustType::AlphanumericEdited { length }) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("new CobolString({length}, \"{escaped}\")")
        }
        (Literal::Figurative(fig), RustType::PicX { length }) => {
            figurative_init(*length, fig)
        }
        (Literal::Figurative(fig), RustType::PicA { length }) => {
            figurative_init(*length, fig)
        }
        // Fallback: numeric value for any numeric type
        (Literal::Numeric(n), _) => format!("/* VALUE {n} */ {}", java_field_init_expr_default(rt)),
        // Fallback: alphanumeric for any type
        (Literal::Alphanumeric(s), _) => {
            format!("/* VALUE \"{s}\" */ {}", java_field_init_expr_default(rt))
        }
        (Literal::Figurative(_), _) => java_field_init_expr_default(rt),
    }
}

fn java_field_init_expr_default(rt: &RustType) -> String {
    match rt {
        RustType::PackedDecimal { precision, scale, signed }
        | RustType::DisplayNumeric { precision, scale, signed } => {
            format!("new CobolDecimal({precision}, {scale}, {signed})")
        }
        RustType::PicX { length } => format!("new CobolString({length})"),
        RustType::PicA { length } => format!("new CobolString({length})"),
        RustType::AlphanumericEdited { length } => format!("new CobolString({length})"),
        RustType::CompBinary { precision, scale, signed, pic_limited } => {
            format!("new CobolBinary({precision}, {scale}, {signed}, {pic_limited})")
        }
        RustType::Float32 => "0.0f".to_string(),
        RustType::Float64 => "0.0d".to_string(),
        RustType::Index | RustType::Pointer => "0L".to_string(),
        RustType::Group => "new byte[0]".to_string(),
    }
}

fn figurative_init(length: u32, fig: &crate::ast::FigurativeConstant) -> String {
    use crate::ast::FigurativeConstant;
    match fig {
        FigurativeConstant::Spaces => format!("new CobolString({length})"),
        FigurativeConstant::Zeros => format!("new CobolString({length}, \"0\")"),
        FigurativeConstant::HighValues => format!("CobolString.highValues({length})"),
        FigurativeConstant::LowValues => format!("CobolString.lowValues({length})"),
        FigurativeConstant::Quotes => format!("new CobolString({length}, \"\\\"\")"),
        FigurativeConstant::Nulls => format!("CobolString.lowValues({length})"),
        FigurativeConstant::All(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("CobolString.allOf({length}, \"{escaped}\")")
        }
    }
}

// --- File handle types ---

fn java_file_handle_type(fd: &FileDescription) -> &'static str {
    match fd.organization {
        FileOrganization::Indexed => "IndexedFile",
        FileOrganization::Relative => "RelativeFile",
        FileOrganization::Sequential | FileOrganization::LineSequential => "SequentialFile",
    }
}

fn java_file_handle_init(fd: &FileDescription) -> String {
    let default_assign = fd.file_name.to_lowercase();
    let assign = fd.assign_to.as_deref().unwrap_or(&default_assign);
    let org = match fd.organization {
        FileOrganization::Indexed => "Indexed",
        FileOrganization::Relative => "Relative",
        FileOrganization::Sequential | FileOrganization::LineSequential => "Sequential",
    };
    let access = match fd.access_mode {
        crate::ast::AccessMode::Sequential => "Sequential",
        crate::ast::AccessMode::Random => "Random",
        crate::ast::AccessMode::Dynamic => "Dynamic",
    };
    format!(
        "new {org}File(\"{assign}\", AccessMode.{access})",
    )
}

// --- Name translation ---

/// Convert COBOL name to Java camelCase.
pub fn cobol_to_java_name(cobol_name: &str, prefix: &str) -> String {
    let parts: Vec<&str> = cobol_name.split('-').collect();
    let base = if parts.is_empty() {
        return "unnamed".to_string();
    } else {
        let mut result = parts[0].to_lowercase();
        for part in &parts[1..] {
            let lower = part.to_lowercase();
            // Capitalize first letter
            let mut chars = lower.chars();
            if let Some(first) = chars.next() {
                result.push(first.to_uppercase().next().unwrap_or(first));
                result.extend(chars);
            }
        }
        result
    };

    let name = if prefix.is_empty() {
        base
    } else {
        let pfx = cobol_to_java_name(prefix, "");
        if base.starts_with(&pfx) {
            base
        } else {
            format!("{pfx}{}", capitalize_first(&base))
        }
    };

    // Java identifiers cannot start with a digit
    let name = if name.starts_with(|c: char| c.is_ascii_digit()) {
        format!("n{name}")
    } else {
        name
    };

    escape_java_keyword(&name)
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => format!("{}{}", c.to_uppercase().next().unwrap_or(c), chars.as_str()),
        None => String::new(),
    }
}

fn escape_java_keyword(name: &str) -> String {
    match name {
        "abstract" | "assert" | "boolean" | "break" | "byte" | "case" | "catch"
        | "char" | "class" | "const" | "continue" | "default" | "do" | "double"
        | "else" | "enum" | "extends" | "final" | "finally" | "float" | "for"
        | "goto" | "if" | "implements" | "import" | "instanceof" | "int" | "interface"
        | "long" | "native" | "new" | "null" | "package" | "private" | "protected"
        | "public" | "return" | "short" | "static" | "strictfp" | "super" | "switch"
        | "synchronized" | "this" | "throw" | "throws" | "transient" | "try" | "void"
        | "volatile" | "while" | "var" | "yield" | "record" | "sealed" | "permits" => {
            format!("{name}_")
        }
        _ => name.to_string(),
    }
}

// --- Helpers (shared logic with Rust data_gen) ---

fn resolve_field_name(
    entry: &DataEntry,
    parent_group: &str,
    duplicates: &HashSet<String>,
    filler_counter: &mut u32,
) -> String {
    if entry.name == "FILLER" || entry.name.is_empty() {
        *filler_counter += 1;
        return format!("filler{}", filler_counter);
    }
    let upper = entry.name.to_uppercase();
    if duplicates.contains(&upper) && !parent_group.is_empty() {
        cobol_to_java_name(&entry.name, parent_group)
    } else {
        cobol_to_java_name(&entry.name, "")
    }
}

/// Compute the set of field names that appear more than once (need disambiguation).
pub fn compute_duplicate_names(records: &[DataEntry], file_section: &[crate::ast::FileDescription]) -> HashSet<String> {
    let mut name_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for record in records {
        collect_leaf_names(record, &mut name_counts);
    }
    for fd in file_section {
        for record in &fd.records {
            collect_leaf_names(record, &mut name_counts);
        }
    }
    name_counts
        .iter()
        .filter(|(_, count)| **count > 1)
        .map(|(name, _)| name.clone())
        .collect()
}

fn collect_leaf_names(entry: &DataEntry, counts: &mut std::collections::HashMap<String, usize>) {
    if entry.name != "FILLER" && !entry.name.is_empty() {
        *counts.entry(entry.name.to_uppercase()).or_default() += 1;
    }
    for child in &entry.children {
        collect_leaf_names(child, counts);
    }
}

fn has_data_children(entry: &DataEntry) -> bool {
    entry
        .children
        .iter()
        .any(|c| c.level != 88 && c.level != 66)
}

fn is_true_group(entry: &DataEntry) -> bool {
    entry.pic.is_none() && has_data_children(entry)
}

fn compute_group_byte_length(entry: &DataEntry) -> usize {
    if let Some(len) = entry.byte_length {
        return len;
    }
    entry.children.iter().map(compute_group_byte_length).sum()
}

fn wrap_ancestor_occurs(inner: &str, ancestors: &[usize]) -> String {
    let mut result = inner.to_string();
    for _ in ancestors.iter().rev() {
        result = format!("CobolArray<{result}>");
    }
    result
}

fn wrap_ancestor_occurs_init(leaf_init: &str, leaf_type: &str, ancestors: &[usize]) -> String {
    if ancestors.is_empty() {
        return leaf_init.to_string();
    }
    // Build nested factory from inside out
    let mut result = leaf_init.to_string();
    let mut inner_type = leaf_type.to_string();
    for count in ancestors.iter().rev() {
        result = format!("new CobolArray<{inner_type}>({count}, () -> {result})");
        inner_type = format!("CobolArray<{inner_type}>");
    }
    result
}

// --- Level-66 RENAMES field generation ---

fn generate_renames_fields(w: &mut JavaWriter, record: &DataEntry) {
    for child in &record.children {
        if child.level == 66 {
            let field_name = cobol_to_java_name(&child.name, "");
            let resolved = resolve_renames_type(child, record);
            let java_type = java_type_string(&resolved);
            let comment = if child.renames_thru.is_some() {
                format!(
                    " /* RENAMES {} THRU {} */",
                    child.renames_target.as_deref().unwrap_or("?"),
                    child.renames_thru.as_deref().unwrap_or("?"),
                )
            } else {
                format!(
                    " /* RENAMES {} */",
                    child.renames_target.as_deref().unwrap_or("?"),
                )
            };
            w.line(&format!("public {java_type} {field_name};{comment}"));
        }
    }
}

fn generate_renames_field_inits(w: &mut JavaWriter, record: &DataEntry) {
    for child in &record.children {
        if child.level == 66 {
            let field_name = cobol_to_java_name(&child.name, "");
            let resolved = resolve_renames_type(child, record);
            let init = java_field_init_expr(child, &resolved);
            w.line(&format!("{field_name} = {init};"));
        }
    }
}

/// Resolve the type for a level-66 RENAMES entry.
fn resolve_renames_type(renames_entry: &DataEntry, record: &DataEntry) -> RustType {
    let target_name = match &renames_entry.renames_target {
        Some(name) => name.to_uppercase(),
        None => return RustType::PicX { length: 1 },
    };

    let target = find_entry_by_name(record, &target_name);

    if let Some(ref _thru_name) = renames_entry.renames_thru {
        // RENAMES THRU: span of bytes -> treat as PicX
        let total_len = renames_entry.byte_length.unwrap_or(1) as u32;
        return RustType::PicX { length: total_len };
    }

    match target {
        Some(entry) => resolve_type(entry).rust_type,
        None => RustType::PicX { length: 1 },
    }
}

fn find_entry_by_name<'a>(record: &'a DataEntry, name: &str) -> Option<&'a DataEntry> {
    for child in &record.children {
        if child.name.eq_ignore_ascii_case(name) {
            return Some(child);
        }
        if let Some(found) = find_entry_by_name(child, name) {
            return Some(found);
        }
    }
    None
}

// --- @Cobol annotation generation ---

fn build_java_field_annotation(entry: &DataEntry) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();

    parts.push(format!("level = {}", entry.level));

    if let Some(pic) = &entry.pic {
        let raw = pic.raw.replace('"', "\\\"");
        parts.push(format!("pic = \"{raw}\""));
    }

    match entry.usage {
        crate::ast::Usage::Comp3 => parts.push("usage = \"COMP-3\"".to_string()),
        crate::ast::Usage::Comp => parts.push("usage = \"COMP\"".to_string()),
        crate::ast::Usage::Comp5 => parts.push("usage = \"COMP-5\"".to_string()),
        crate::ast::Usage::Comp1 => parts.push("usage = \"COMP-1\"".to_string()),
        crate::ast::Usage::Comp2 => parts.push("usage = \"COMP-2\"".to_string()),
        crate::ast::Usage::Index => parts.push("usage = \"INDEX\"".to_string()),
        crate::ast::Usage::Pointer => parts.push("usage = \"POINTER\"".to_string()),
        crate::ast::Usage::Display => {}
    }

    if let Some(offset) = entry.byte_offset {
        parts.push(format!("offset = {offset}"));
    }
    if let Some(len) = entry.byte_length {
        parts.push(format!("len = {len}"));
    }
    if entry.pic.as_ref().is_some_and(|p| p.signed) {
        parts.push("signed = true".to_string());
    }
    if let Some(target) = &entry.redefines {
        parts.push(format!("redefines = \"{target}\""));
    }
    if let Some(count) = entry.occurs {
        parts.push(format!("occurs = {count}"));
    }

    if parts.len() <= 1 && entry.name == "FILLER" {
        return None;
    }

    Some(format!("@Cobol({})", parts.join(", ")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Usage;

    fn test_entry(name: &str, value: Option<Literal>) -> DataEntry {
        DataEntry {
            level: 1,
            name: name.to_string(),
            pic: None,
            usage: Usage::Display,
            value,
            redefines: None,
            occurs: None,
            occurs_depending: None,
            sign: None,
            justified_right: false,
            blank_when_zero: false,
            children: vec![],
            condition_values: vec![],
            byte_offset: None,
            byte_length: None,
            renames_target: None,
            renames_thru: None,
            index_names: vec![],
        }
    }

    #[test]
    fn cobol_to_java_name_simple() {
        assert_eq!(cobol_to_java_name("WS-ACCT-NBR", ""), "wsAcctNbr");
        assert_eq!(cobol_to_java_name("WS-BALANCE", ""), "wsBalance");
        assert_eq!(cobol_to_java_name("FILLER", ""), "filler");
    }

    #[test]
    fn cobol_to_java_name_with_prefix() {
        assert_eq!(cobol_to_java_name("STATUS", "ACCT"), "acctStatus");
    }

    #[test]
    fn cobol_to_java_name_keyword_escape() {
        assert_eq!(cobol_to_java_name("CLASS", ""), "class_");
        assert_eq!(cobol_to_java_name("RETURN", ""), "return_");
        assert_eq!(cobol_to_java_name("IMPORT", ""), "import_");
    }

    #[test]
    fn cobol_to_java_name_leading_digit() {
        assert_eq!(cobol_to_java_name("1000-MAIN", ""), "n1000Main");
    }

    #[test]
    fn java_type_mapping() {
        assert_eq!(
            java_type_string(&RustType::PackedDecimal {
                precision: 5,
                scale: 2,
                signed: true
            }),
            "CobolDecimal"
        );
        assert_eq!(
            java_type_string(&RustType::PicX { length: 10 }),
            "CobolString"
        );
        assert_eq!(
            java_type_string(&RustType::CompBinary {
                precision: 9,
                scale: 0,
                signed: true,
                pic_limited: false
            }),
            "CobolBinary"
        );
        assert_eq!(java_type_string(&RustType::Float32), "float");
        assert_eq!(java_type_string(&RustType::Float64), "double");
        assert_eq!(java_type_string(&RustType::Index), "long");
    }

    #[test]
    fn java_init_default() {
        let entry = test_entry("TEST", None);
        let init = java_field_init_expr(
            &entry,
            &RustType::PackedDecimal {
                precision: 5,
                scale: 2,
                signed: true,
            },
        );
        assert_eq!(init, "new CobolDecimal(5, 2, true)");
    }

    #[test]
    fn java_init_with_value() {
        let entry = test_entry("SCORE", Some(Literal::Numeric("750".to_string())));
        let init = java_field_init_expr(
            &entry,
            &RustType::PackedDecimal {
                precision: 3,
                scale: 0,
                signed: false,
            },
        );
        assert_eq!(init, "new CobolDecimal(3, 0, false).set(\"750\")");
    }

    #[test]
    fn java_init_string_value() {
        let entry = test_entry("NAME", Some(Literal::Alphanumeric("HELLO".to_string())));
        let init = java_field_init_expr(&entry, &RustType::PicX { length: 10 });
        assert_eq!(init, "new CobolString(10, \"HELLO\")");
    }

    #[test]
    fn generate_empty_working_storage() {
        let mut w = JavaWriter::new();
        generate_working_storage(&mut w, &[], &[], false, "TESTPROG");
        let output = w.finish();
        assert!(output.contains("@Cobol(program = \"TESTPROG\")"));
        assert!(output.contains("public class WorkingStorage"));
        assert!(output.contains("public WorkingStorage()"));
    }
}
