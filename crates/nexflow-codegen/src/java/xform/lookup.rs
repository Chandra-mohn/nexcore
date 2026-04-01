// NexCore -- Nexflow Codegen: Lookup Service Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates `LookupService` field declarations, initialization, and accessor methods
//! for transforms that reference external data sources.

use std::fmt::Write;

use nexflow_parser::ast::transform::LookupFieldDecl;

use crate::java::naming::{to_camel_case, to_pascal_case};

/// Write `LookupService` field declarations.
pub fn write_lookup_fields(out: &mut String, lookups: &[LookupFieldDecl]) {
    for lk in lookups {
        let field_name = format!("{}Lookup", to_camel_case(&lk.name));
        let type_name = to_pascal_case(&lk.source);
        writeln!(
            out,
            "    private transient LookupService<{type_name}> {field_name};"
        )
        .unwrap();
    }
}

/// Write lookup initialization code (inside `open()` method).
pub fn write_lookup_init(out: &mut String, lookups: &[LookupFieldDecl]) {
    writeln!(out, "        // Initialize lookups").unwrap();
    for lk in lookups {
        let field_name = format!("{}Lookup", to_camel_case(&lk.name));
        let source = &lk.source;
        writeln!(
            out,
            "        {field_name} = LookupServiceFactory.create(\"{source}\");"
        )
        .unwrap();
    }
}

/// Write lookup accessor methods.
pub fn write_lookup_methods(
    out: &mut String,
    lookups: &[LookupFieldDecl],
    input_type: &str,
) {
    for lk in lookups {
        let method_name = format!("lookup{}", to_pascal_case(&lk.name));
        let return_type = to_pascal_case(&lk.source);
        let field_name = format!("{}Lookup", to_camel_case(&lk.name));

        writeln!(out, "    /**").unwrap();
        writeln!(
            out,
            "     * Lookup {} from {}.",
            lk.name, lk.source
        )
        .unwrap();
        writeln!(out, "     */").unwrap();
        writeln!(
            out,
            "    protected {return_type} {method_name}({input_type} input) throws Exception {{"
        )
        .unwrap();
        writeln!(out, "        // Extract lookup key from input").unwrap();
        writeln!(out, "        Object key = input; // TODO: extract key field").unwrap();
        writeln!(out, "        if (key == null) {{").unwrap();
        writeln!(out, "            return null;").unwrap();
        writeln!(out, "        }}").unwrap();
        writeln!(out, "        return {field_name}.lookup(key);").unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_fields() {
        let lookups = vec![
            LookupFieldDecl {
                name: "customer".to_string(),
                source: "customer_profile".to_string(),
            },
            LookupFieldDecl {
                name: "merchant".to_string(),
                source: "merchants".to_string(),
            },
        ];

        let mut out = String::new();
        write_lookup_fields(&mut out, &lookups);

        assert!(out.contains("LookupService<CustomerProfile> customerLookup;"));
        assert!(out.contains("LookupService<Merchants> merchantLookup;"));
        assert!(out.contains("transient"));
    }

    #[test]
    fn test_lookup_init() {
        let lookups = vec![LookupFieldDecl {
            name: "customer".to_string(),
            source: "customer_profile".to_string(),
        }];

        let mut out = String::new();
        write_lookup_init(&mut out, &lookups);

        assert!(out.contains("LookupServiceFactory.create(\"customer_profile\")"));
    }

    #[test]
    fn test_lookup_methods() {
        let lookups = vec![LookupFieldDecl {
            name: "customer".to_string(),
            source: "customer_profile".to_string(),
        }];

        let mut out = String::new();
        write_lookup_methods(&mut out, &lookups, "TransactionEvent");

        assert!(out.contains("protected CustomerProfile lookupCustomer(TransactionEvent input)"));
        assert!(out.contains("customerLookup.lookup(key)"));
    }
}
