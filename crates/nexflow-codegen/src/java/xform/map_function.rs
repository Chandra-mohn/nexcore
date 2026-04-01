// NexCore -- Nexflow Codegen: Flink MapFunction Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Flink `MapFunction` / `RichMapFunction` Java classes from
//! `TransformDef` and `TransformBlockDef` ASTs.
//!
//! - Simple transforms (no cache/state/lookups) -> `implements MapFunction<I, O>`
//! - Rich transforms (cache/state/lookups) -> `extends RichMapFunction<I, O>`

use std::fmt::Write;

use nexflow_parser::ast::transform::{
    FieldSpec, Statement, TransformBlockDef, TransformDef, TransformFieldType,
};

use crate::java::naming::{to_camel_case, to_pascal_case};

use super::cache;
use super::expression::translate_expression;
use super::lookup;

const DEFAULT_PACKAGE: &str = "com.nexflow.transform";

/// Generate a MapFunction Java class from a `TransformDef`.
///
/// Returns `(filename, content)`.
pub fn generate_map_function(transform: &TransformDef) -> (String, String) {
    let class_name = format!("{}Function", to_pascal_case(&transform.name));
    let filename = format!("{class_name}.java");
    let needs_rich = transform.cache.is_some()
        || transform.state.is_some()
        || !transform.lookups.is_empty();

    let (input_type, input_is_typed) = resolve_io_type(&transform.inputs, "input");
    let (output_type, _output_is_typed) = resolve_io_type(&transform.outputs, "output");

    let mut out = String::with_capacity(8192);

    // Package
    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();

    // Imports
    write_imports(&mut out, needs_rich, &input_type, &output_type, transform);
    writeln!(out).unwrap();

    // Class javadoc
    writeln!(out, "/**").unwrap();
    if let Some(desc) = &transform.description {
        writeln!(out, " * {desc}").unwrap();
    } else {
        writeln!(out, " * Transform: {}", transform.name).unwrap();
    }
    writeln!(
        out,
        " * Generated from {}.xform -- DO NOT EDIT.",
        transform.name
    )
    .unwrap();
    writeln!(out, " */").unwrap();

    // Class declaration
    if needs_rich {
        writeln!(
            out,
            "public class {class_name} extends RichMapFunction<{input_type}, {output_type}> {{"
        )
        .unwrap();
    } else {
        writeln!(
            out,
            "public class {class_name} implements MapFunction<{input_type}, {output_type}> {{"
        )
        .unwrap();
    }

    writeln!(
        out,
        "    private static final Logger LOG = LoggerFactory.getLogger({class_name}.class);"
    )
    .unwrap();
    writeln!(out).unwrap();

    // Lookup fields
    if !transform.lookups.is_empty() {
        lookup::write_lookup_fields(&mut out, &transform.lookups);
        writeln!(out).unwrap();
    }

    // Cache state fields
    if let Some(cache_decl) = &transform.cache {
        cache::write_cache_fields(&mut out, &transform.name, cache_decl);
        writeln!(out).unwrap();
    }

    // open() method (if RichMapFunction)
    if needs_rich {
        writeln!(out, "    @Override").unwrap();
        writeln!(
            out,
            "    public void open(Configuration parameters) throws Exception {{"
        )
        .unwrap();
        writeln!(out, "        super.open(parameters);").unwrap();

        if !transform.lookups.is_empty() {
            lookup::write_lookup_init(&mut out, &transform.lookups);
        }
        if let Some(cache_decl) = &transform.cache {
            cache::write_cache_init(&mut out, &transform.name, cache_decl);
        }

        writeln!(out, "    }}").unwrap();
        writeln!(out).unwrap();
    }

    // map() method
    writeln!(out, "    @Override").unwrap();
    writeln!(
        out,
        "    public {output_type} map({input_type} input) throws Exception {{"
    )
    .unwrap();
    if transform.cache.is_some() {
        writeln!(out, "        return transformWithCache(input);").unwrap();
    } else {
        writeln!(out, "        return transform(input);").unwrap();
    }
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // Cache-wrapped method (if cache)
    if let Some(cache_decl) = &transform.cache {
        cache::write_cache_transform_method(
            &mut out,
            &transform.name,
            cache_decl,
            &input_type,
            &output_type,
        );
        writeln!(out).unwrap();
    }

    // transform() method -- the core logic
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Core transformation logic.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public {output_type} transform({input_type} input) throws Exception {{"
    )
    .unwrap();

    // Input validation
    if !transform.validate_input.is_empty() {
        writeln!(out, "        // Input validation").unwrap();
        writeln!(out, "        validateInput(input);").unwrap();
        writeln!(out).unwrap();
    }

    // Execute lookups
    if !transform.lookups.is_empty() {
        writeln!(out, "        // Execute lookups").unwrap();
        for lk in &transform.lookups {
            let var_name = to_camel_case(&lk.name);
            let type_name = to_pascal_case(&lk.source);
            let method = format!("lookup{}", to_pascal_case(&lk.name));
            writeln!(
                out,
                "        {type_name} {var_name} = {method}(input);"
            )
            .unwrap();
        }
        writeln!(out).unwrap();
    }

    // Apply block -- generate local variables and assignments
    if input_is_typed {
        // Typed path: use Avro builder
        write_typed_apply_block(&mut out, &transform.apply, &output_type);
    } else {
        // Map path: use HashMap
        write_map_apply_block(&mut out, &transform.apply);
    }

    // Output validation
    if !transform.validate_output.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "        // Output validation").unwrap();
        writeln!(out, "        validateOutput(result);").unwrap();
    }

    writeln!(out).unwrap();
    writeln!(out, "        return result;").unwrap();
    writeln!(out, "    }}").unwrap();

    // Lookup accessor methods
    if !transform.lookups.is_empty() {
        writeln!(out).unwrap();
        lookup::write_lookup_methods(&mut out, &transform.lookups, &input_type);
    }

    // Validation methods
    if !transform.validate_input.is_empty() {
        writeln!(out).unwrap();
        write_validation_method(&mut out, "validateInput", &input_type, &transform.validate_input);
    }
    if !transform.validate_output.is_empty() {
        writeln!(out).unwrap();
        write_validation_method(
            &mut out,
            "validateOutput",
            &output_type,
            &transform.validate_output,
        );
    }

    // Error handling
    if let Some(err) = &transform.on_error {
        writeln!(out).unwrap();
        write_error_handler(&mut out, err);
    }

    // Close class
    writeln!(out, "}}").unwrap();

    (filename, out)
}

/// Generate a MapFunction from a `TransformBlockDef` (multi-field mapping).
///
/// Returns `(filename, content)`.
pub fn generate_block_function(block: &TransformBlockDef) -> (String, String) {
    let class_name = format!("{}Function", to_pascal_case(&block.name));
    let filename = format!("{class_name}.java");

    let (input_type, _) = resolve_io_type(&block.inputs, "input");
    let (output_type, _) = resolve_io_type(&block.outputs, "output");

    let mut out = String::with_capacity(8192);

    // Package + imports
    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "import org.apache.flink.api.common.functions.MapFunction;").unwrap();
    writeln!(out, "import org.slf4j.Logger;").unwrap();
    writeln!(out, "import org.slf4j.LoggerFactory;").unwrap();
    writeln!(out, "import java.util.*;").unwrap();
    writeln!(out).unwrap();

    // Class
    writeln!(out, "/**").unwrap();
    if let Some(desc) = &block.description {
        writeln!(out, " * {desc}").unwrap();
    } else {
        writeln!(out, " * Transform block: {}", block.name).unwrap();
    }
    writeln!(
        out,
        " * Generated from {}.xform -- DO NOT EDIT.",
        block.name
    )
    .unwrap();
    writeln!(out, " */").unwrap();
    writeln!(
        out,
        "public class {class_name} implements MapFunction<{input_type}, {output_type}> {{"
    )
    .unwrap();
    writeln!(
        out,
        "    private static final Logger LOG = LoggerFactory.getLogger({class_name}.class);"
    )
    .unwrap();
    writeln!(out).unwrap();

    // Used transform fields
    if !block.uses.is_empty() {
        writeln!(out, "    // Composed transforms").unwrap();
        for used in &block.uses {
            let field_name = to_camel_case(used);
            let type_name = format!("{}Function", to_pascal_case(used));
            writeln!(
                out,
                "    private final {type_name} {field_name} = new {type_name}();"
            )
            .unwrap();
        }
        writeln!(out).unwrap();
    }

    // map() method
    writeln!(out, "    @Override").unwrap();
    writeln!(
        out,
        "    public {output_type} map({input_type} input) throws Exception {{"
    )
    .unwrap();
    writeln!(out, "        return transform(input);").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // transform() method
    writeln!(
        out,
        "    public {output_type} transform({input_type} input) throws Exception {{"
    )
    .unwrap();

    // Input validation
    if !block.validate_input.is_empty() {
        writeln!(out, "        validateInput(input);").unwrap();
        writeln!(out).unwrap();
    }

    // Compose or mappings
    if let Some(compose) = &block.compose {
        super::compose::write_compose_block(&mut out, compose, &output_type);
    } else if !block.mappings.is_empty() {
        // Direct mappings
        writeln!(
            out,
            "        {output_type} result = {output_type}.newBuilder()"
        )
        .unwrap();
        for m in &block.mappings {
            let java_expr = translate_expression(&m.expression);
            let field_name = m.target.split('.').last().unwrap_or(&m.target);
            let setter = format!("set{}", to_pascal_case(field_name));
            writeln!(out, "            .{setter}({java_expr})").unwrap();
        }
        writeln!(out, "            .build();").unwrap();
    }

    // Output validation
    if !block.validate_output.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "        validateOutput(result);").unwrap();
    }

    writeln!(out).unwrap();
    writeln!(out, "        return result;").unwrap();
    writeln!(out, "    }}").unwrap();

    // Validation methods
    if !block.validate_input.is_empty() {
        writeln!(out).unwrap();
        write_validation_method(&mut out, "validateInput", &input_type, &block.validate_input);
    }
    if !block.validate_output.is_empty() {
        writeln!(out).unwrap();
        write_validation_method(
            &mut out,
            "validateOutput",
            &output_type,
            &block.validate_output,
        );
    }

    // Close class
    writeln!(out, "}}").unwrap();

    (filename, out)
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Resolve input/output type from field specs.
///
/// Returns `(java_type, is_typed)` where `is_typed` means it references a schema.
fn resolve_io_type(fields: &[FieldSpec], _context: &str) -> (String, bool) {
    if fields.len() == 1 {
        let f = &fields[0];
        match &f.field_type {
            TransformFieldType::Reference(name) => (to_pascal_case(name), true),
            TransformFieldType::AliasRef(name) => (name.clone(), true),
            TransformFieldType::Base { name, .. } => (base_type_to_java(name), false),
            TransformFieldType::Collection { .. } => {
                ("Map<String, Object>".to_string(), false)
            }
        }
    } else if fields.is_empty() {
        ("Map<String, Object>".to_string(), false)
    } else {
        // Multiple fields -> Map
        ("Map<String, Object>".to_string(), false)
    }
}

fn base_type_to_java(name: &str) -> String {
    match name {
        "string" => "String".to_string(),
        "integer" => "Long".to_string(),
        "decimal" => "java.math.BigDecimal".to_string(),
        "boolean" => "Boolean".to_string(),
        _ => "Object".to_string(),
    }
}

/// Write the apply block for typed (Avro SpecificRecord) transforms.
fn write_typed_apply_block(out: &mut String, stmts: &[Statement], output_type: &str) {
    // Separate locals from assignments
    let locals: Vec<&Statement> = stmts.iter().filter(|s| s.is_let).collect();
    let assignments: Vec<&Statement> = stmts.iter().filter(|s| !s.is_let).collect();

    // Local variables
    if !locals.is_empty() {
        writeln!(out, "        // Local variables").unwrap();
        for s in &locals {
            let var_name = to_camel_case(&s.target);
            let java_expr = translate_expression(&s.expression);
            writeln!(out, "        var {var_name} = {java_expr};").unwrap();
        }
        writeln!(out).unwrap();
    }

    // Build output using Avro builder
    writeln!(
        out,
        "        {output_type} result = {output_type}.newBuilder()"
    )
    .unwrap();
    for s in &assignments {
        let java_expr = translate_expression(&s.expression);
        let field_name = s.target.split('.').last().unwrap_or(&s.target);
        let setter = format!("set{}", to_pascal_case(field_name));
        writeln!(out, "            .{setter}({java_expr})").unwrap();
    }
    writeln!(out, "            .build();").unwrap();
}

/// Write the apply block for untyped (Map<String, Object>) transforms.
fn write_map_apply_block(out: &mut String, stmts: &[Statement]) {
    writeln!(
        out,
        "        Map<String, Object> result = new HashMap<>();"
    )
    .unwrap();
    writeln!(out).unwrap();

    for s in stmts {
        let java_expr = translate_expression(&s.expression);
        if s.is_let {
            let var_name = to_camel_case(&s.target);
            writeln!(out, "        var {var_name} = {java_expr};").unwrap();
        } else {
            let key = s
                .target
                .split('.')
                .last()
                .unwrap_or(&s.target);
            writeln!(out, "        result.put(\"{key}\", {java_expr});").unwrap();
        }
    }
}

fn write_imports(
    out: &mut String,
    needs_rich: bool,
    input_type: &str,
    output_type: &str,
    transform: &TransformDef,
) {
    if needs_rich {
        writeln!(
            out,
            "import org.apache.flink.api.common.functions.RichMapFunction;"
        )
        .unwrap();
        writeln!(out, "import org.apache.flink.configuration.Configuration;").unwrap();
    } else {
        writeln!(
            out,
            "import org.apache.flink.api.common.functions.MapFunction;"
        )
        .unwrap();
    }

    if transform.cache.is_some() {
        writeln!(
            out,
            "import org.apache.flink.api.common.state.MapState;"
        )
        .unwrap();
        writeln!(
            out,
            "import org.apache.flink.api.common.state.MapStateDescriptor;"
        )
        .unwrap();
        writeln!(
            out,
            "import org.apache.flink.api.common.state.StateTtlConfig;"
        )
        .unwrap();
        writeln!(out, "import org.apache.flink.api.common.time.Time;").unwrap();
    }

    writeln!(out, "import org.slf4j.Logger;").unwrap();
    writeln!(out, "import org.slf4j.LoggerFactory;").unwrap();
    writeln!(out, "import java.util.*;").unwrap();

    // Schema imports
    for type_name in [input_type, output_type] {
        if type_name != "Map<String, Object>"
            && type_name != "String"
            && type_name != "Long"
            && type_name != "Object"
        {
            writeln!(
                out,
                "import com.nexflow.schemas.{type_name};"
            )
            .unwrap();
        }
    }

    // Lookup type imports
    for lk in &transform.lookups {
        let type_name = to_pascal_case(&lk.source);
        writeln!(
            out,
            "import com.nexflow.schemas.{type_name};"
        )
        .unwrap();
    }
}

fn write_validation_method(
    out: &mut String,
    method_name: &str,
    param_type: &str,
    rules: &[nexflow_parser::ast::transform::ValidationRule],
) {
    writeln!(
        out,
        "    private void {method_name}({param_type} value) throws Exception {{"
    )
    .unwrap();
    writeln!(
        out,
        "        List<String> errors = new ArrayList<>();"
    )
    .unwrap();

    for rule in rules {
        let java_cond = translate_expression(&rule.expression);
        let msg = rule
            .message
            .as_deref()
            .unwrap_or("Validation failed");
        writeln!(out, "        if (!({java_cond})) {{").unwrap();
        writeln!(out, "            errors.add(\"{msg}\");").unwrap();
        writeln!(out, "        }}").unwrap();
    }

    writeln!(out, "        if (!errors.isEmpty()) {{").unwrap();
    writeln!(
        out,
        "            throw new IllegalArgumentException(\"Validation failed: \" + errors);"
    )
    .unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
}

fn write_error_handler(out: &mut String, err: &nexflow_parser::ast::transform::ErrorBlock) {
    writeln!(out, "    // Error handling configuration").unwrap();
    if let Some(action) = &err.action {
        writeln!(
            out,
            "    // Action: {action}"
        )
        .unwrap();
    }
    if let Some(log_level) = &err.log_level {
        writeln!(
            out,
            "    // Log level: {log_level}"
        )
        .unwrap();
    }
    if let Some(error_code) = &err.error_code {
        writeln!(
            out,
            "    private static final String ERROR_CODE = \"{error_code}\";"
        )
        .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::transform::*;

    fn make_simple_transform() -> TransformDef {
        TransformDef {
            name: "normalize_name".to_string(),
            version: None,
            description: Some("Normalize a name to uppercase".to_string()),
            previous_version: None,
            compatibility: None,
            pure: Some(true),
            idempotent: Some(true),
            cache: None,
            inputs: vec![FieldSpec {
                name: Some("name".to_string()),
                field_type: TransformFieldType::Base {
                    name: "string".to_string(),
                    constraints: Vec::new(),
                },
                nullable: false,
                required: true,
                default: None,
            }],
            outputs: vec![FieldSpec {
                name: Some("normalized".to_string()),
                field_type: TransformFieldType::Base {
                    name: "string".to_string(),
                    constraints: Vec::new(),
                },
                nullable: false,
                required: true,
                default: None,
            }],
            lookup: None,
            lookups: Vec::new(),
            state: None,
            params: Vec::new(),
            validate_input: Vec::new(),
            apply: vec![Statement {
                target: "output.normalized".to_string(),
                expression: "upper(input.name)".to_string(),
                is_let: false,
            }],
            validate_output: Vec::new(),
            on_error: None,
        }
    }

    fn make_typed_transform() -> TransformDef {
        TransformDef {
            name: "enrich_account".to_string(),
            version: Some("1.0".to_string()),
            description: None,
            previous_version: None,
            compatibility: None,
            pure: None,
            idempotent: None,
            cache: None,
            inputs: vec![FieldSpec {
                name: None,
                field_type: TransformFieldType::Reference("account_detail".to_string()),
                nullable: false,
                required: true,
                default: None,
            }],
            outputs: vec![FieldSpec {
                name: None,
                field_type: TransformFieldType::Reference("account_summary".to_string()),
                nullable: false,
                required: true,
                default: None,
            }],
            lookup: None,
            lookups: vec![LookupFieldDecl {
                name: "customer".to_string(),
                source: "customer_profile".to_string(),
            }],
            state: None,
            params: Vec::new(),
            validate_input: vec![ValidationRule {
                expression: "input.balance > 0".to_string(),
                message: Some("Balance must be positive".to_string()),
            }],
            apply: vec![
                Statement {
                    target: "customer".to_string(),
                    expression: "lookups.customer(input.account_id)".to_string(),
                    is_let: true,
                },
                Statement {
                    target: "output.account_id".to_string(),
                    expression: "input.account_id".to_string(),
                    is_let: false,
                },
                Statement {
                    target: "output.holder_name".to_string(),
                    expression: "customer.name".to_string(),
                    is_let: false,
                },
            ],
            validate_output: Vec::new(),
            on_error: Some(ErrorBlock {
                action: Some("reject".to_string()),
                default: None,
                log_level: Some("error".to_string()),
                emit_to: None,
                error_code: Some("ENRICH_FAILED".to_string()),
            }),
        }
    }

    #[test]
    fn test_simple_map_function() {
        let (filename, content) = generate_map_function(&make_simple_transform());

        assert_eq!(filename, "NormalizeNameFunction.java");
        assert!(content.contains("implements MapFunction<String, String>"));
        assert!(content.contains("public String transform(String input)"));
        assert!(!content.contains("RichMapFunction")); // No cache/state/lookups
    }

    #[test]
    fn test_typed_map_function() {
        let (filename, content) = generate_map_function(&make_typed_transform());

        assert_eq!(filename, "EnrichAccountFunction.java");
        // Has lookups -> RichMapFunction
        assert!(content.contains("extends RichMapFunction<AccountDetail, AccountSummary>"));
        assert!(content.contains("public void open(Configuration parameters)"));
    }

    #[test]
    fn test_lookup_fields() {
        let (_, content) = generate_map_function(&make_typed_transform());

        assert!(content.contains("LookupService<CustomerProfile>"));
        assert!(content.contains("lookupCustomer("));
    }

    #[test]
    fn test_validation_method() {
        let (_, content) = generate_map_function(&make_typed_transform());

        assert!(content.contains("private void validateInput(AccountDetail value)"));
        assert!(content.contains("Balance must be positive"));
    }

    #[test]
    fn test_error_handler() {
        let (_, content) = generate_map_function(&make_typed_transform());

        assert!(content.contains("ERROR_CODE = \"ENRICH_FAILED\""));
    }

    #[test]
    fn test_avro_builder_pattern() {
        let (_, content) = generate_map_function(&make_typed_transform());

        assert!(content.contains("AccountSummary.newBuilder()"));
        assert!(content.contains(".setAccountId("));
        assert!(content.contains(".setHolderName("));
        assert!(content.contains(".build();"));
    }

    #[test]
    fn test_schema_imports() {
        let (_, content) = generate_map_function(&make_typed_transform());

        assert!(content.contains("import com.nexflow.schemas.AccountDetail;"));
        assert!(content.contains("import com.nexflow.schemas.AccountSummary;"));
        assert!(content.contains("import com.nexflow.schemas.CustomerProfile;"));
    }

    #[test]
    fn test_block_function() {
        let block = TransformBlockDef {
            name: "format_account".to_string(),
            version: None,
            description: Some("Format account for display".to_string()),
            uses: vec!["normalize_name".to_string()],
            inputs: vec![FieldSpec {
                name: None,
                field_type: TransformFieldType::Reference("account_detail".to_string()),
                nullable: false,
                required: true,
                default: None,
            }],
            outputs: vec![FieldSpec {
                name: None,
                field_type: TransformFieldType::Reference("account_summary".to_string()),
                nullable: false,
                required: true,
                default: None,
            }],
            validate_input: Vec::new(),
            invariants: Vec::new(),
            mappings: vec![
                Mapping {
                    target: "summary.account_id".to_string(),
                    expression: "input.account_id".to_string(),
                },
                Mapping {
                    target: "summary.holder_name".to_string(),
                    expression: "input.holder_name".to_string(),
                },
            ],
            compose: None,
            validate_output: Vec::new(),
            on_change: None,
            on_error: None,
        };

        let (filename, content) = generate_block_function(&block);

        assert_eq!(filename, "FormatAccountFunction.java");
        assert!(content.contains("implements MapFunction<AccountDetail, AccountSummary>"));
        assert!(content.contains("NormalizeNameFunction normalizeName = new NormalizeNameFunction()"));
        assert!(content.contains("AccountSummary.newBuilder()"));
        assert!(content.contains(".setAccountId(input.getAccountId())"));
    }
}
