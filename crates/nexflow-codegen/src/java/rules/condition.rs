// NexCore -- Nexflow Codegen: Rules Condition Translator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Translates RulesDSL condition expressions to Java boolean code.
//!
//! Each condition type (wildcard, exact match, range, set membership,
//! pattern, null check, comparison, complex expression) has a dedicated
//! translation path that produces null-safe, type-aware Java code.

use nexflow_parser::ast::rules::ConditionExpr;

use crate::java::naming::to_pascal_case;

/// Translate a `ConditionExpr` to a Java boolean expression string.
///
/// `accessor` is the Java getter for the column field (e.g. `input.getFicoScore()`).
/// `param_type` hints the Java type for type-aware comparisons.
pub fn translate_condition(
    condition: &ConditionExpr,
    accessor: &str,
    param_type: &str,
) -> String {
    match condition {
        ConditionExpr::ExactMatch(value) => {
            translate_exact_match(accessor, value, param_type)
        }
        ConditionExpr::Range { from, to } => {
            translate_range(accessor, from, to, param_type)
        }
        ConditionExpr::InSet { values, negated } => {
            translate_in_set(accessor, values, *negated)
        }
        ConditionExpr::Pattern { kind, pattern } => {
            translate_pattern(accessor, kind, pattern)
        }
        ConditionExpr::NullCheck { is_null } => {
            if *is_null {
                format!("({accessor} == null)")
            } else {
                format!("({accessor} != null)")
            }
        }
        ConditionExpr::Comparison { operator, value } => {
            translate_comparison(accessor, operator, value, param_type)
        }
        ConditionExpr::Expression(expr) => {
            // Complex boolean expression -- pass through with basic translation
            format!("({})", translate_boolean_expr(expr))
        }
        ConditionExpr::MarkerState { marker, state } => {
            format!("markerState(\"{marker}\").equals(\"{state}\")")
        }
    }
}

fn translate_exact_match(accessor: &str, value: &str, param_type: &str) -> String {
    if is_string_type(param_type) {
        format!("\"{value}\".equals({accessor})")
    } else if value == "true" || value == "false" {
        format!("({accessor} == {value})")
    } else {
        // Numeric -- use equals for safety
        format!("java.util.Objects.equals({accessor}, {value}L)")
    }
}

fn translate_range(accessor: &str, from: &str, to: &str, param_type: &str) -> String {
    if is_decimal_type(param_type) {
        format!(
            "({accessor} != null && {accessor}.compareTo(new java.math.BigDecimal(\"{from}\")) >= 0 && {accessor}.compareTo(new java.math.BigDecimal(\"{to}\")) <= 0)"
        )
    } else {
        format!("({accessor} != null && {accessor} >= {from}L && {accessor} <= {to}L)")
    }
}

fn translate_in_set(accessor: &str, values: &[String], negated: bool) -> String {
    let items: String = values
        .iter()
        .map(|v| format!("\"{v}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let contains = format!("java.util.Arrays.asList({items}).contains({accessor})");
    if negated {
        format!("(!{contains})")
    } else {
        format!("({contains})")
    }
}

fn translate_pattern(accessor: &str, kind: &str, pattern: &str) -> String {
    match kind {
        "matches" => format!("({accessor} != null && {accessor}.matches(\"{pattern}\"))"),
        "starts_with" => format!("({accessor} != null && {accessor}.startsWith(\"{pattern}\"))"),
        "ends_with" => format!("({accessor} != null && {accessor}.endsWith(\"{pattern}\"))"),
        "contains" => format!("({accessor} != null && {accessor}.contains(\"{pattern}\"))"),
        _ => format!("({accessor} != null && {accessor}.matches(\"{pattern}\"))"),
    }
}

fn translate_comparison(
    accessor: &str,
    operator: &str,
    value: &str,
    param_type: &str,
) -> String {
    if is_decimal_type(param_type) {
        let cmp_op = match operator {
            ">=" => ">= 0",
            ">" => "> 0",
            "<=" => "<= 0",
            "<" => "< 0",
            "==" | "=" => "== 0",
            "!=" => "!= 0",
            _ => ">= 0",
        };
        format!(
            "({accessor} != null && {accessor}.compareTo(new java.math.BigDecimal(\"{value}\")) {cmp_op})"
        )
    } else if is_string_type(param_type) {
        match operator {
            "==" | "=" => format!("\"{value}\".equals({accessor})"),
            "!=" => format!("!\"{value}\".equals({accessor})"),
            _ => format!("({accessor} {operator} \"{value}\")"),
        }
    } else {
        // Numeric/boolean -- direct comparison
        let java_value = if value.contains('.') {
            value.to_string()
        } else {
            format!("{value}L")
        };
        format!("({accessor} != null && {accessor} {operator} {java_value})")
    }
}

/// Basic boolean expression translation for complex expressions.
fn translate_boolean_expr(expr: &str) -> String {
    expr.replace(" and ", " && ")
        .replace(" or ", " || ")
        .replace(" not ", " !")
        .replace("is null", "== null")
        .replace("is not null", "!= null")
}

/// Translate a RulesDSL type name to a Java type string.
pub fn rules_type_to_java(type_name: &str) -> String {
    match type_name {
        "text" | "string" => "String".to_string(),
        "number" | "integer" => "Long".to_string(),
        "money" | "decimal" | "percentage" => "java.math.BigDecimal".to_string(),
        "boolean" => "Boolean".to_string(),
        "date" | "bizdate" => "java.time.LocalDate".to_string(),
        "timestamp" => "java.time.Instant".to_string(),
        other => to_pascal_case(other),
    }
}

fn is_string_type(param_type: &str) -> bool {
    matches!(param_type, "text" | "string" | "String")
}

fn is_decimal_type(param_type: &str) -> bool {
    matches!(
        param_type,
        "money" | "decimal" | "percentage" | "BigDecimal" | "java.math.BigDecimal"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match_string() {
        let cond = ConditionExpr::ExactMatch("excellent".to_string());
        let result = translate_condition(&cond, "input.getCreditHistory()", "text");
        assert_eq!(
            result,
            "\"excellent\".equals(input.getCreditHistory())"
        );
    }

    #[test]
    fn test_exact_match_numeric() {
        let cond = ConditionExpr::ExactMatch("0".to_string());
        let result = translate_condition(&cond, "input.getCount()", "number");
        assert_eq!(
            result,
            "java.util.Objects.equals(input.getCount(), 0L)"
        );
    }

    #[test]
    fn test_exact_match_boolean() {
        let cond = ConditionExpr::ExactMatch("true".to_string());
        let result = translate_condition(&cond, "input.getActive()", "boolean");
        assert_eq!(result, "(input.getActive() == true)");
    }

    #[test]
    fn test_range_numeric() {
        let cond = ConditionExpr::Range {
            from: "700".to_string(),
            to: "799".to_string(),
        };
        let result = translate_condition(&cond, "input.getScore()", "number");
        assert!(result.contains(">= 700L"));
        assert!(result.contains("<= 799L"));
    }

    #[test]
    fn test_range_decimal() {
        let cond = ConditionExpr::Range {
            from: "0.01".to_string(),
            to: "0.50".to_string(),
        };
        let result = translate_condition(&cond, "input.getRatio()", "money");
        assert!(result.contains("compareTo(new java.math.BigDecimal(\"0.01\")) >= 0"));
        assert!(result.contains("compareTo(new java.math.BigDecimal(\"0.50\")) <= 0"));
    }

    #[test]
    fn test_in_set() {
        let cond = ConditionExpr::InSet {
            values: vec!["A".to_string(), "B".to_string(), "C".to_string()],
            negated: false,
        };
        let result = translate_condition(&cond, "input.getType()", "text");
        assert!(result.contains("Arrays.asList(\"A\", \"B\", \"C\")"));
        assert!(result.contains(".contains(input.getType())"));
    }

    #[test]
    fn test_not_in_set() {
        let cond = ConditionExpr::InSet {
            values: vec!["X".to_string()],
            negated: true,
        };
        let result = translate_condition(&cond, "input.getType()", "text");
        assert!(result.starts_with("(!"));
    }

    #[test]
    fn test_pattern_starts_with() {
        let cond = ConditionExpr::Pattern {
            kind: "starts_with".to_string(),
            pattern: "ACC".to_string(),
        };
        let result = translate_condition(&cond, "input.getCode()", "text");
        assert!(result.contains(".startsWith(\"ACC\")"));
    }

    #[test]
    fn test_null_check() {
        let cond_null = ConditionExpr::NullCheck { is_null: true };
        assert_eq!(
            translate_condition(&cond_null, "input.getX()", "text"),
            "(input.getX() == null)"
        );

        let cond_not_null = ConditionExpr::NullCheck { is_null: false };
        assert_eq!(
            translate_condition(&cond_not_null, "input.getX()", "text"),
            "(input.getX() != null)"
        );
    }

    #[test]
    fn test_comparison_gte_decimal() {
        let cond = ConditionExpr::Comparison {
            operator: ">=".to_string(),
            value: "50000".to_string(),
        };
        let result = translate_condition(&cond, "input.getIncome()", "money");
        assert!(result.contains("compareTo(new java.math.BigDecimal(\"50000\")) >= 0"));
    }

    #[test]
    fn test_comparison_lt_numeric() {
        let cond = ConditionExpr::Comparison {
            operator: "<".to_string(),
            value: "10000".to_string(),
        };
        let result = translate_condition(&cond, "input.getDebt()", "number");
        assert!(result.contains("< 10000L"));
    }

    #[test]
    fn test_marker_state() {
        let cond = ConditionExpr::MarkerState {
            marker: "eod_1".to_string(),
            state: "fired".to_string(),
        };
        let result = translate_condition(&cond, "", "");
        assert!(result.contains("markerState(\"eod_1\").equals(\"fired\")"));
    }

    #[test]
    fn test_rules_type_mapping() {
        assert_eq!(rules_type_to_java("text"), "String");
        assert_eq!(rules_type_to_java("number"), "Long");
        assert_eq!(rules_type_to_java("money"), "java.math.BigDecimal");
        assert_eq!(rules_type_to_java("boolean"), "Boolean");
        assert_eq!(rules_type_to_java("date"), "java.time.LocalDate");
    }
}
