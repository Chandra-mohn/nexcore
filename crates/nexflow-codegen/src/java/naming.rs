// NexCore -- Nexflow Codegen: Java Naming Utilities
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Java naming conventions for code generation.
//!
//! - Schema names (snake_case) -> Java class names (PascalCase)
//! - Field names (snake_case) -> Java field names (camelCase)
//! - Constants -> UPPER_SNAKE_CASE

/// Convert snake_case to PascalCase for Java class names.
/// `"account_detail"` -> `"AccountDetail"`
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let mut result = c.to_uppercase().to_string();
                    result.extend(chars);
                    result
                }
                None => String::new(),
            }
        })
        .collect()
}

/// Convert snake_case to camelCase for Java field/method names.
/// `"holder_name"` -> `"holderName"`
pub fn to_camel_case(s: &str) -> String {
    let mut parts = s.split('_');
    let first = parts.next().unwrap_or("");
    let rest: String = parts
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let mut result = c.to_uppercase().to_string();
                    result.extend(chars);
                    result
                }
                None => String::new(),
            }
        })
        .collect();
    format!("{first}{rest}")
}

/// Convert snake_case to UPPER_SNAKE_CASE for Java constants.
/// `"holder_name"` -> `"HOLDER_NAME"`
pub fn to_constant_case(s: &str) -> String {
    s.to_uppercase()
}

/// Generate a Java getter name from a snake_case field.
/// `"holder_name"` -> `"getHolderName"`
pub fn to_getter(s: &str) -> String {
    format!("get{}", to_pascal_case(s))
}

/// Generate a Java setter name from a snake_case field.
/// `"holder_name"` -> `"setHolderName"`
pub fn to_setter(s: &str) -> String {
    format!("set{}", to_pascal_case(s))
}

/// Convert a schema name to its Java package fragment.
/// `"account_detail"` -> `"accountdetail"`
pub fn to_package_fragment(s: &str) -> String {
    s.replace('_', "").to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("account_detail"), "AccountDetail");
        assert_eq!(to_pascal_case("transfer_request"), "TransferRequest");
        assert_eq!(to_pascal_case("uuid"), "Uuid");
        assert_eq!(to_pascal_case(""), "");
        assert_eq!(to_pascal_case("a"), "A");
        assert_eq!(
            to_pascal_case("account_created_event"),
            "AccountCreatedEvent"
        );
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("holder_name"), "holderName");
        assert_eq!(to_camel_case("account_id"), "accountId");
        assert_eq!(to_camel_case("id"), "id");
        assert_eq!(to_camel_case("created_at"), "createdAt");
        assert_eq!(to_camel_case("pii_phone"), "piiPhone");
    }

    #[test]
    fn test_to_constant_case() {
        assert_eq!(to_constant_case("holder_name"), "HOLDER_NAME");
        assert_eq!(to_constant_case("schema_version"), "SCHEMA_VERSION");
    }

    #[test]
    fn test_getters_setters() {
        assert_eq!(to_getter("holder_name"), "getHolderName");
        assert_eq!(to_setter("holder_name"), "setHolderName");
        assert_eq!(to_getter("id"), "getId");
        assert_eq!(to_setter("account_id"), "setAccountId");
    }

    #[test]
    fn test_to_package_fragment() {
        assert_eq!(to_package_fragment("account_detail"), "accountdetail");
        assert_eq!(to_package_fragment("transfer"), "transfer");
    }
}
