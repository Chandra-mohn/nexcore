// NexCore -- Nexflow Codegen: Naming Utilities
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Name conversion between DSL conventions and Rust conventions.
//!
//! - SchemaDSL uses snake_case: `account_detail`
//! - ApiDSL references PascalCase: `AccountDetail`
//! - Rust struct names are PascalCase, fn names are snake_case

/// Convert snake_case to PascalCase.
/// `"account_detail"` -> `"AccountDetail"`
pub fn snake_to_pascal(s: &str) -> String {
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

/// Convert PascalCase or camelCase to snake_case.
/// `"AccountDetail"` -> `"account_detail"`
/// `"listAccounts"` -> `"list_accounts"`
pub fn pascal_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert an endpoint name (camelCase) to a Rust function name (snake_case).
/// `"listAccounts"` -> `"list_accounts"`
pub fn endpoint_to_fn_name(name: &str) -> String {
    pascal_to_snake(name)
}

/// Convert a SchemaRef name to a Rust type name (PascalCase).
/// Already PascalCase in the API DSL, but schema names are snake_case.
/// This ensures consistent PascalCase output.
pub fn schema_ref_to_type_name(name: &str) -> String {
    if name.chars().next().is_some_and(|c| c.is_uppercase()) {
        // Already PascalCase
        name.to_string()
    } else {
        // snake_case -> PascalCase
        snake_to_pascal(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snake_to_pascal() {
        assert_eq!(snake_to_pascal("account_detail"), "AccountDetail");
        assert_eq!(snake_to_pascal("transfer_request"), "TransferRequest");
        assert_eq!(snake_to_pascal("uuid"), "Uuid");
        assert_eq!(snake_to_pascal(""), "");
        assert_eq!(snake_to_pascal("a"), "A");
        assert_eq!(
            snake_to_pascal("account_created_event"),
            "AccountCreatedEvent"
        );
    }

    #[test]
    fn test_pascal_to_snake() {
        assert_eq!(pascal_to_snake("AccountDetail"), "account_detail");
        assert_eq!(pascal_to_snake("listAccounts"), "list_accounts");
        assert_eq!(pascal_to_snake("getBalance"), "get_balance");
        assert_eq!(pascal_to_snake("API"), "a_p_i");
        assert_eq!(pascal_to_snake("transferFunds"), "transfer_funds");
    }

    #[test]
    fn test_endpoint_to_fn_name() {
        assert_eq!(endpoint_to_fn_name("listAccounts"), "list_accounts");
        assert_eq!(endpoint_to_fn_name("getAccount"), "get_account");
        assert_eq!(endpoint_to_fn_name("createAccount"), "create_account");
        assert_eq!(endpoint_to_fn_name("transferFunds"), "transfer_funds");
    }

    #[test]
    fn test_schema_ref_to_type_name() {
        assert_eq!(schema_ref_to_type_name("AccountDetail"), "AccountDetail");
        assert_eq!(schema_ref_to_type_name("account_detail"), "AccountDetail");
        assert_eq!(
            schema_ref_to_type_name("validation_error"),
            "ValidationError"
        );
    }
}
