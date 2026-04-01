// NexCore -- Nexflow Codegen: Flink Cache Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Flink `MapState` / `ValueState` cache declarations and
//! cache-wrapped transform methods.
//!
//! Cache uses Flink's `StateTtlConfig` for automatic expiration.

use std::fmt::Write;

use nexflow_parser::ast::transform::CacheDecl;

use crate::java::naming::{to_camel_case, to_pascal_case};

/// Write cache state field declarations.
pub fn write_cache_fields(out: &mut String, transform_name: &str, cache: &CacheDecl) {
    let field_name = format!("{}CacheMap", to_camel_case(transform_name));
    if cache.key.is_empty() {
        // Simple ValueState cache
        writeln!(
            out,
            "    private transient org.apache.flink.api.common.state.ValueState<Object> {}CacheState;",
            to_camel_case(transform_name)
        )
        .unwrap();
    } else {
        // Key-based MapState cache
        writeln!(
            out,
            "    private transient MapState<String, Object> {field_name};"
        )
        .unwrap();
    }
}

/// Write cache initialization code (inside `open()` method).
pub fn write_cache_init(out: &mut String, transform_name: &str, cache: &CacheDecl) {
    let ttl_ms = parse_ttl_to_ms(cache.ttl.as_deref().unwrap_or("3600s"));

    writeln!(out, "        // Initialize cache").unwrap();
    writeln!(out, "        StateTtlConfig ttlConfig = StateTtlConfig").unwrap();
    writeln!(
        out,
        "            .newBuilder(Time.milliseconds({ttl_ms}L))"
    )
    .unwrap();
    writeln!(
        out,
        "            .setUpdateType(StateTtlConfig.UpdateType.OnCreateAndWrite)"
    )
    .unwrap();
    writeln!(
        out,
        "            .setStateVisibility(StateTtlConfig.StateVisibility.NeverReturnExpired)"
    )
    .unwrap();
    writeln!(out, "            .build();").unwrap();

    if cache.key.is_empty() {
        let state_name = format!("{}CacheState", to_camel_case(transform_name));
        let desc_name = format!("{}_cache", transform_name);
        writeln!(
            out,
            "        var {state_name}Desc = new org.apache.flink.api.common.state.ValueStateDescriptor<>(\"{desc_name}\", Object.class);"
        )
        .unwrap();
        writeln!(
            out,
            "        {state_name}Desc.enableTimeToLive(ttlConfig);"
        )
        .unwrap();
        writeln!(
            out,
            "        {state_name} = getRuntimeContext().getState({state_name}Desc);"
        )
        .unwrap();
    } else {
        let map_name = format!("{}CacheMap", to_camel_case(transform_name));
        let desc_name = format!("{}_cache", transform_name);
        writeln!(
            out,
            "        MapStateDescriptor<String, Object> {map_name}Desc = new MapStateDescriptor<>(\"{desc_name}\", String.class, Object.class);"
        )
        .unwrap();
        writeln!(
            out,
            "        {map_name}Desc.enableTimeToLive(ttlConfig);"
        )
        .unwrap();
        writeln!(
            out,
            "        {map_name} = getRuntimeContext().getMapState({map_name}Desc);"
        )
        .unwrap();
    }
}

/// Write the cache-wrapped transform method.
pub fn write_cache_transform_method(
    out: &mut String,
    transform_name: &str,
    cache: &CacheDecl,
    input_type: &str,
    output_type: &str,
) {
    let method_name = "transformWithCache";

    writeln!(
        out,
        "    public {output_type} {method_name}({input_type} input) throws Exception {{"
    )
    .unwrap();

    if cache.key.is_empty() {
        // Simple ValueState
        let state_name = format!("{}CacheState", to_camel_case(transform_name));
        writeln!(out, "        Object cached = {state_name}.value();").unwrap();
        writeln!(out, "        if (cached != null) {{").unwrap();
        writeln!(out, "            return ({output_type}) cached;").unwrap();
        writeln!(out, "        }}").unwrap();
        writeln!(out).unwrap();
        writeln!(
            out,
            "        {output_type} result = transform(input);"
        )
        .unwrap();
        writeln!(out, "        {state_name}.update(result);").unwrap();
        writeln!(out, "        return result;").unwrap();
    } else {
        // Key-based MapState
        let map_name = format!("{}CacheMap", to_camel_case(transform_name));
        let key_builder_name = format!(
            "build{}CacheKey",
            to_pascal_case(transform_name)
        );

        writeln!(
            out,
            "        String cacheKey = {key_builder_name}(input);"
        )
        .unwrap();
        writeln!(
            out,
            "        Object cached = {map_name}.get(cacheKey);"
        )
        .unwrap();
        writeln!(out, "        if (cached != null) {{").unwrap();
        writeln!(out, "            return ({output_type}) cached;").unwrap();
        writeln!(out, "        }}").unwrap();
        writeln!(out).unwrap();
        writeln!(
            out,
            "        {output_type} result = transform(input);"
        )
        .unwrap();
        writeln!(out, "        {map_name}.put(cacheKey, result);").unwrap();
        writeln!(out, "        return result;").unwrap();
    }

    writeln!(out, "    }}").unwrap();

    // Cache key builder (for key-based cache)
    if !cache.key.is_empty() {
        writeln!(out).unwrap();
        let key_builder_name = format!(
            "build{}CacheKey",
            to_pascal_case(transform_name)
        );
        writeln!(
            out,
            "    private String {key_builder_name}({input_type} input) {{"
        )
        .unwrap();
        writeln!(out, "        StringBuilder keyBuilder = new StringBuilder();").unwrap();
        for (i, key_field) in cache.key.iter().enumerate() {
            if i > 0 {
                writeln!(
                    out,
                    "        keyBuilder.append(\"::\");"
                )
                .unwrap();
            }
            let getter = format!("get{}", to_pascal_case(key_field));
            writeln!(
                out,
                "        keyBuilder.append(String.valueOf(input.{getter}()));"
            )
            .unwrap();
        }
        writeln!(out, "        return keyBuilder.toString();").unwrap();
        writeln!(out, "    }}").unwrap();
    }
}

/// Parse a TTL string like "300s", "5m", "1h" to milliseconds.
fn parse_ttl_to_ms(ttl: &str) -> u64 {
    let trimmed = ttl.trim();
    // Check word suffixes before single-char to avoid "seconds" matching 's'
    if let Some(rest) = trimmed.strip_suffix("seconds") {
        rest.trim().parse::<u64>().unwrap_or(3600) * 1000
    } else if let Some(seconds) = trimmed.strip_suffix('s') {
        seconds.trim().parse::<u64>().unwrap_or(3600) * 1000
    } else if let Some(rest) = trimmed.strip_suffix("minutes") {
        rest.trim().parse::<u64>().unwrap_or(60) * 60_000
    } else if let Some(minutes) = trimmed.strip_suffix('m') {
        minutes.trim().parse::<u64>().unwrap_or(60) * 60_000
    } else if let Some(rest) = trimmed.strip_suffix("hours") {
        rest.trim().parse::<u64>().unwrap_or(1) * 3_600_000
    } else if let Some(hours) = trimmed.strip_suffix('h') {
        hours.trim().parse::<u64>().unwrap_or(1) * 3_600_000
    } else {
        // Default: assume seconds
        trimmed.parse::<u64>().unwrap_or(3600) * 1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_fields_keyed() {
        let cache = CacheDecl {
            ttl: Some("300s".to_string()),
            key: vec!["transaction_id".to_string(), "merchant_id".to_string()],
        };

        let mut out = String::new();
        write_cache_fields(&mut out, "enrich_transaction", &cache);

        assert!(out.contains("MapState<String, Object> enrichTransactionCacheMap"));
    }

    #[test]
    fn test_cache_fields_simple() {
        let cache = CacheDecl {
            ttl: Some("60s".to_string()),
            key: Vec::new(),
        };

        let mut out = String::new();
        write_cache_fields(&mut out, "simple_calc", &cache);

        assert!(out.contains("ValueState<Object> simpleCalcCacheState"));
    }

    #[test]
    fn test_cache_init() {
        let cache = CacheDecl {
            ttl: Some("300s".to_string()),
            key: vec!["id".to_string()],
        };

        let mut out = String::new();
        write_cache_init(&mut out, "enrich", &cache);

        assert!(out.contains("Time.milliseconds(300000L)"));
        assert!(out.contains("OnCreateAndWrite"));
        assert!(out.contains("NeverReturnExpired"));
        assert!(out.contains("getMapState("));
    }

    #[test]
    fn test_cache_transform_keyed() {
        let cache = CacheDecl {
            ttl: Some("300s".to_string()),
            key: vec!["transaction_id".to_string()],
        };

        let mut out = String::new();
        write_cache_transform_method(
            &mut out,
            "enrich",
            &cache,
            "TransactionEvent",
            "EnrichedTransaction",
        );

        assert!(out.contains("transformWithCache(TransactionEvent input)"));
        assert!(out.contains("enrichCacheMap.get(cacheKey)"));
        assert!(out.contains("enrichCacheMap.put(cacheKey, result)"));
        assert!(out.contains("buildEnrichCacheKey(TransactionEvent input)"));
        assert!(out.contains("getTransactionId()"));
    }

    #[test]
    fn test_parse_ttl() {
        assert_eq!(parse_ttl_to_ms("300s"), 300_000);
        assert_eq!(parse_ttl_to_ms("300 seconds"), 300_000);
        assert_eq!(parse_ttl_to_ms("5m"), 300_000);
        assert_eq!(parse_ttl_to_ms("1h"), 3_600_000);
        assert_eq!(parse_ttl_to_ms("3600"), 3_600_000);
    }
}
