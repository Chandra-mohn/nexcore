// NexCore -- Nexflow Codegen: Rust Expression Translator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Translates DSL expression strings to Rust code.
//!
//! Key differences from Java translator:
//! - Field access: `input.field` stays as `input.field` (Rust struct access)
//! - No getter boilerplate
//! - `upper(x)` -> `x.to_uppercase()`
//! - `null` -> `None`, null coalesce `??` -> `.unwrap_or()`
//! - `and`/`or` -> `&&`/`||`

/// Translate a DSL expression to Rust code.
pub fn translate_rust_expr(expr: &str) -> String {
    let trimmed = expr.trim();

    if trimmed.is_empty() {
        return "()".to_string();
    }

    // String literal
    if trimmed.starts_with('"') && trimmed.ends_with('"') {
        return format!("{trimmed}.to_string()");
    }

    // Numeric literal
    if trimmed.parse::<f64>().is_ok() {
        return trimmed.to_string();
    }

    // Boolean literal
    if trimmed == "true" || trimmed == "false" {
        return trimmed.to_string();
    }

    // null -> None
    if trimmed == "null" {
        return "None".to_string();
    }

    // Null coalesce: expr ?? default
    if let Some((left, right)) = split_at_operator(trimmed, "??") {
        let left_rs = translate_rust_expr(left);
        let right_rs = translate_rust_expr(right);
        return format!("{left_rs}.unwrap_or_else(|| {right_rs})");
    }

    // Known function calls
    if let Some(result) = translate_rust_function(trimmed) {
        return result;
    }

    // Field path translation + operator replacement
    translate_rust_field_paths(trimmed)
}

/// Translate known DSL functions to Rust equivalents.
fn translate_rust_function(expr: &str) -> Option<String> {
    let paren_start = expr.find('(')?;
    if !expr.ends_with(')') {
        return None;
    }
    let func_name = expr[..paren_start].trim();
    let args_str = &expr[paren_start + 1..expr.len() - 1];

    if func_name.contains(' ') || func_name.contains('.') {
        return None;
    }

    let args: Vec<String> = split_args(args_str)
        .iter()
        .map(|a| translate_rust_expr(a))
        .collect();

    let result = match func_name {
        // String
        "upper" | "uppercase" => format!("{}.to_uppercase()", first_arg(&args)),
        "lower" | "lowercase" => format!("{}.to_lowercase()", first_arg(&args)),
        "trim" => format!("{}.trim().to_string()", first_arg(&args)),
        "length" => format!("{}.len()", first_arg(&args)),
        "concat" => args.join(" + &"),
        "substring" => {
            if args.len() >= 3 {
                format!("{}[{}..{}].to_string()", args[0], args[1], args[2])
            } else if args.len() == 2 {
                format!("{}[{}..].to_string()", args[0], args[1])
            } else {
                return None;
            }
        }
        "replace" if args.len() >= 3 => {
            format!("{}.replace(&{}, &{})", args[0], args[1], args[2])
        }

        // Math
        "round" if args.len() >= 2 => {
            format!("{}.round_dp({})", args[0], args[1])
        }
        "round" => format!("{}.round()", first_arg(&args)),
        "abs" => format!("{}.abs()", first_arg(&args)),
        "max" if args.len() == 2 => format!("{}.max({})", args[0], args[1]),
        "min" if args.len() == 2 => format!("{}.min({})", args[0], args[1]),

        // Type conversion
        "string" => format!("{}.to_string()", first_arg(&args)),
        "integer" => format!("{} as i64", first_arg(&args)),
        "decimal" => format!("Decimal::from_str_exact(&{})?", first_arg(&args)),

        // Date/time
        "now" | "processing_date" => "chrono::Utc::now()".to_string(),
        "current_date" | "current_business_date" => "chrono::Utc::now().date_naive()".to_string(),
        "generate_uuid" => "uuid::Uuid::new_v4()".to_string(),

        // Collection operations
        "filter" if args.len() == 2 => {
            format!(
                "{}.iter().filter(|item| {}).cloned().collect::<Vec<_>>()",
                args[0], args[1]
            )
        }
        "any" if args.len() == 2 => {
            format!("{}.iter().any(|item| {})", args[0], args[1])
        }
        "all" if args.len() == 2 => {
            format!("{}.iter().all(|item| {})", args[0], args[1])
        }
        "sum" if args.len() == 2 => {
            format!("{}.iter().map(|item| {}).sum::<Decimal>()", args[0], args[1])
        }
        "count" if args.len() == 1 => format!("{}.len()", args[0]),
        "find" if args.len() == 2 => {
            format!("{}.iter().find(|item| {}).cloned()", args[0], args[1])
        }
        "distinct" if args.len() == 1 => {
            format!(
                "{}.iter().cloned().collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>()",
                args[0]
            )
        }

        _ => return None,
    };

    Some(result)
}

/// Translate field paths and operators.
fn translate_rust_field_paths(expr: &str) -> String {
    // Replace DSL boolean operators with Rust
    let result = expr
        .replace(" and ", " && ")
        .replace(" or ", " || ")
        .replace(" not ", " !")
        .replace("is null", ".is_none()")
        .replace("is not null", ".is_some()");

    // Field paths: `input.field` stays as-is in Rust (direct struct field access)
    // No getter translation needed (unlike Java)

    // Clone string fields accessed from references
    result
}

fn first_arg(args: &[String]) -> &str {
    args.first().map(|s| s.as_str()).unwrap_or("()")
}

fn split_at_operator<'a>(expr: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let mut depth = 0;
    let mut in_string = false;
    let bytes = expr.as_bytes();
    let op_bytes = op.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'"' {
            in_string = !in_string;
        } else if !in_string {
            if bytes[i] == b'(' {
                depth += 1;
            } else if bytes[i] == b')' {
                depth -= 1;
            } else if depth == 0
                && i + op_bytes.len() <= bytes.len()
                && &bytes[i..i + op_bytes.len()] == op_bytes
            {
                return Some((&expr[..i], &expr[i + op.len()..]));
            }
        }
        i += 1;
    }
    None
}

fn split_args(args: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut start = 0;

    for (i, c) in args.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '(' | '[' | '{' if !in_string => depth += 1,
            ')' | ']' | '}' if !in_string => depth -= 1,
            ',' if !in_string && depth == 0 => {
                result.push(args[start..i].trim());
                start = i + 1;
            }
            _ => {}
        }
    }
    let last = args[start..].trim();
    if !last.is_empty() {
        result.push(last);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_literal() {
        assert_eq!(
            translate_rust_expr("\"hello\""),
            "\"hello\".to_string()"
        );
    }

    #[test]
    fn test_numeric() {
        assert_eq!(translate_rust_expr("42"), "42");
        assert_eq!(translate_rust_expr("3.14"), "3.14");
    }

    #[test]
    fn test_boolean() {
        assert_eq!(translate_rust_expr("true"), "true");
    }

    #[test]
    fn test_null() {
        assert_eq!(translate_rust_expr("null"), "None");
    }

    #[test]
    fn test_null_coalesce() {
        let result = translate_rust_expr("input.name ?? \"unknown\"");
        assert!(result.contains("unwrap_or_else"));
    }

    #[test]
    fn test_upper() {
        assert_eq!(
            translate_rust_expr("upper(name)"),
            "name.to_uppercase()"
        );
    }

    #[test]
    fn test_concat() {
        let result = translate_rust_expr("concat(first, last)");
        assert!(result.contains("first + &last"));
    }

    #[test]
    fn test_uuid() {
        assert!(translate_rust_expr("generate_uuid()").contains("Uuid::new_v4()"));
    }

    #[test]
    fn test_field_path_stays_as_is() {
        // Rust doesn't need getters
        assert_eq!(
            translate_rust_expr("input.account_id"),
            "input.account_id"
        );
    }

    #[test]
    fn test_boolean_operators() {
        let result = translate_rust_expr("input.active and input.verified");
        assert!(result.contains("&&"));
    }

    #[test]
    fn test_collection_filter() {
        let result = translate_rust_expr("filter(items, x -> x.active)");
        assert!(result.contains(".iter().filter("));
    }

    #[test]
    fn test_collection_any() {
        let result = translate_rust_expr("any(items, x -> x.amount > 1000)");
        assert!(result.contains(".iter().any("));
    }
}
