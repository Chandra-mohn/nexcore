use serde_json::Value;

use crate::models::MessageFilter;

/// Check if a decoded JSON message matches a filter.
pub fn matches_filter(value: &Value, filter: &MessageFilter) -> bool {
    let field_value = resolve_field(value, &filter.field);

    match field_value {
        Some(fv) => compare(&fv, &filter.op, &filter.value),
        None => false,
    }
}

/// Resolve a potentially nested field path (e.g. "customer.address.city").
fn resolve_field<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in path.split('.') {
        // Check for array index: "items[0]"
        if let Some(idx_start) = segment.find('[') {
            let key = &segment[..idx_start];
            let idx_str = &segment[idx_start + 1..segment.len() - 1];

            current = current.get(key)?;
            let idx: usize = idx_str.parse().ok()?;
            current = current.get(idx)?;
        } else {
            current = current.get(segment)?;
        }
    }
    Some(current)
}

fn compare(field_value: &Value, op: &str, filter_value: &str) -> bool {
    match op {
        "eq" => value_equals(field_value, filter_value),
        "neq" => !value_equals(field_value, filter_value),
        "contains" => {
            if let Some(s) = field_value.as_str() {
                s.contains(filter_value)
            } else {
                field_value.to_string().contains(filter_value)
            }
        }
        "gt" | "lt" | "gte" | "lte" => numeric_compare(field_value, op, filter_value),
        _ => false,
    }
}

fn value_equals(field_value: &Value, filter_value: &str) -> bool {
    match field_value {
        Value::String(s) => s == filter_value,
        Value::Number(n) => {
            if let Ok(fv) = filter_value.parse::<f64>() {
                n.as_f64().is_some_and(|nv| (nv - fv).abs() < f64::EPSILON)
            } else {
                n.to_string() == filter_value
            }
        }
        Value::Bool(b) => {
            matches!(
                (b, filter_value),
                (true, "true") | (false, "false")
            )
        }
        Value::Null => filter_value == "null",
        _ => field_value.to_string() == filter_value,
    }
}

fn numeric_compare(field_value: &Value, op: &str, filter_value: &str) -> bool {
    let fv = field_value.as_f64();
    let tv = filter_value.parse::<f64>().ok();

    match (fv, tv) {
        (Some(a), Some(b)) => match op {
            "gt" => a > b,
            "lt" => a < b,
            "gte" => a >= b,
            "lte" => a <= b,
            _ => false,
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_eq() {
        let msg = serde_json::json!({"status": "pending", "amount": 100});
        let filter = MessageFilter {
            field: "status".into(),
            op: "eq".into(),
            value: "pending".into(),
        };
        assert!(matches_filter(&msg, &filter));
    }

    #[test]
    fn nested_field() {
        let msg = serde_json::json!({"customer": {"city": "NYC"}});
        let filter = MessageFilter {
            field: "customer.city".into(),
            op: "eq".into(),
            value: "NYC".into(),
        };
        assert!(matches_filter(&msg, &filter));
    }

    #[test]
    fn numeric_gt() {
        let msg = serde_json::json!({"amount": 150});
        let filter = MessageFilter {
            field: "amount".into(),
            op: "gt".into(),
            value: "100".into(),
        };
        assert!(matches_filter(&msg, &filter));
    }

    #[test]
    fn contains_filter() {
        let msg = serde_json::json!({"message": "Order shipped to NYC"});
        let filter = MessageFilter {
            field: "message".into(),
            op: "contains".into(),
            value: "shipped".into(),
        };
        assert!(matches_filter(&msg, &filter));
    }
}
