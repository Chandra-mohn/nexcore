// NexCore -- Nexflow Codegen: Flink Pipeline Operator Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates Flink pipeline operator code for each `ProcessStatement` type.
//!
//! Each operator produces inline Java code within the `buildPipeline()` method.

use std::fmt::Write;

use nexflow_parser::ast::proc::{
    AggregateField, EnrichLookup, ProcessStatement, RouteCase,
};

use crate::java::naming::{to_camel_case, to_pascal_case};

/// Write a pipeline operator for a single `ProcessStatement`.
///
/// `prev_stream` is the variable name of the upstream DataStream.
/// Returns the variable name of the output DataStream.
pub fn write_operator(
    out: &mut String,
    stmt: &ProcessStatement,
    prev_stream: &str,
    step_idx: usize,
) -> String {
    match stmt {
        ProcessStatement::Transform { input, using, into } => {
            write_transform(out, input, using, into, prev_stream)
        }
        ProcessStatement::Route { input, routes } => {
            write_route(out, input, routes, prev_stream, step_idx)
        }
        ProcessStatement::Window {
            window_type,
            size,
            group_by,
            options,
        } => write_window(out, window_type, size, group_by, options, prev_stream, step_idx),
        ProcessStatement::Join {
            left,
            right,
            join_type,
            on,
            within,
        } => write_join(out, left, right, join_type, on, within.as_deref(), step_idx),
        ProcessStatement::Enrich { target, lookups } => {
            write_enrich(out, target, lookups, prev_stream)
        }
        ProcessStatement::Aggregate { name, fields } => {
            write_aggregate(out, name, fields, prev_stream)
        }
        ProcessStatement::Evaluate { raw } => {
            write_evaluate(out, raw, prev_stream, step_idx)
        }
        // Receive and Emit are handled by source/sink generators
        ProcessStatement::Receive { .. } | ProcessStatement::Emit { .. } => {
            prev_stream.to_string()
        }
        // Complex statements pass through as comments
        ProcessStatement::Branch { raw }
        | ProcessStatement::Correlation { raw }
        | ProcessStatement::Completion { raw } => {
            writeln!(out, "        // {}: {raw}", stmt_type_name(stmt)).unwrap();
            writeln!(out).unwrap();
            prev_stream.to_string()
        }
    }
}

/// Transform operator: delegates to L3 MapFunction.
fn write_transform(
    out: &mut String,
    _input: &str,
    using: &str,
    into: &str,
    prev_stream: &str,
) -> String {
    let function_class = format!("{}Function", to_pascal_case(using));
    let out_var = to_camel_case(into);

    writeln!(out, "        // Transform: {using} -> {into}").unwrap();
    writeln!(
        out,
        "        var {out_var} = {prev_stream}.map(new {function_class}()).name(\"transform-{using}\");"
    )
    .unwrap();
    writeln!(out).unwrap();

    out_var
}

/// Route operator: ProcessFunction with side outputs.
fn write_route(
    out: &mut String,
    _input: &str,
    routes: &[RouteCase],
    prev_stream: &str,
    step_idx: usize,
) -> String {
    let out_var = format!("routed{step_idx}");

    // Declare output tags
    for route in routes {
        let tag_name = format!("{}Tag", to_camel_case(&route.target));
        writeln!(
            out,
            "        OutputTag<Object> {tag_name} = new OutputTag<>(\"{}\") {{}};",
            route.target
        )
        .unwrap();
    }
    writeln!(out).unwrap();

    // ProcessFunction
    writeln!(
        out,
        "        var {out_var} = {prev_stream}.process(new ProcessFunction<>() {{"
    )
    .unwrap();
    writeln!(out, "            @Override").unwrap();
    writeln!(
        out,
        "            public void processElement(Object value, Context ctx, Collector<Object> collector) {{"
    )
    .unwrap();

    let mut first = true;
    for route in routes {
        let tag_name = format!("{}Tag", to_camel_case(&route.target));
        if route.is_otherwise {
            writeln!(out, "            else {{").unwrap();
            writeln!(
                out,
                "                ctx.output({tag_name}, value);"
            )
            .unwrap();
            writeln!(out, "            }}").unwrap();
        } else if let Some(condition) = &route.condition {
            let keyword = if first { "if" } else { "else if" };
            first = false;
            writeln!(
                out,
                "            {keyword} ({condition}) {{"
            )
            .unwrap();
            writeln!(
                out,
                "                ctx.output({tag_name}, value);"
            )
            .unwrap();
            writeln!(out, "            }}").unwrap();
        } else {
            writeln!(
                out,
                "            collector.collect(value); // default route to {}", route.target
            )
            .unwrap();
        }
    }

    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}).name(\"route-{step_idx}\");").unwrap();
    writeln!(out).unwrap();

    out_var
}

/// Window operator: tumbling, sliding, or session.
fn write_window(
    out: &mut String,
    window_type: &str,
    size: &str,
    group_by: &[String],
    options: &[(String, String)],
    prev_stream: &str,
    step_idx: usize,
) -> String {
    let out_var = format!("windowed{step_idx}");
    let size_ms = parse_duration_to_ms(size);

    writeln!(out, "        // Window: {window_type} {size}").unwrap();

    // Key-by
    if !group_by.is_empty() {
        let key_expr = if group_by.len() == 1 {
            let getter = format!("get{}()", to_pascal_case(&group_by[0]));
            format!("record -> record.{getter} != null ? record.{getter}.toString() : \"\"")
        } else {
            let getters: Vec<String> = group_by
                .iter()
                .map(|f| format!("record.get{}()", to_pascal_case(f)))
                .collect();
            format!(
                "record -> String.valueOf({}) ",
                getters.join(" + \":\" + String.valueOf(")
                    + &")".repeat(getters.len().saturating_sub(1))
            )
        };
        writeln!(
            out,
            "        var {out_var}Keyed = {prev_stream}.keyBy({key_expr});"
        )
        .unwrap();
    }

    let keyed_ref = if group_by.is_empty() {
        prev_stream.to_string()
    } else {
        format!("{out_var}Keyed")
    };

    // Window assignment
    let window_assigner = match window_type {
        "tumbling" => {
            format!(
                "org.apache.flink.streaming.api.windowing.assigners.TumblingEventTimeWindows.of(org.apache.flink.streaming.api.windowing.time.Time.milliseconds({size_ms}))"
            )
        }
        "sliding" => {
            let slide = options
                .iter()
                .find(|(k, _)| k == "every" || k == "slide")
                .map(|(_, v)| parse_duration_to_ms(v))
                .unwrap_or(size_ms / 2);
            format!(
                "org.apache.flink.streaming.api.windowing.assigners.SlidingEventTimeWindows.of(org.apache.flink.streaming.api.windowing.time.Time.milliseconds({size_ms}), org.apache.flink.streaming.api.windowing.time.Time.milliseconds({slide}))"
            )
        }
        "session" => {
            format!(
                "org.apache.flink.streaming.api.windowing.assigners.EventTimeSessionWindows.withGap(org.apache.flink.streaming.api.windowing.time.Time.milliseconds({size_ms}))"
            )
        }
        _ => format!("TumblingEventTimeWindows.of(Time.milliseconds({size_ms}))"),
    };

    writeln!(
        out,
        "        var {out_var} = {keyed_ref}.window({window_assigner})"
    )
    .unwrap();

    // Allowed lateness
    if let Some((_, lateness)) = options.iter().find(|(k, _)| k == "lateness") {
        let lateness_ms = parse_duration_to_ms(lateness);
        writeln!(
            out,
            "            .allowedLateness(org.apache.flink.streaming.api.windowing.time.Time.milliseconds({lateness_ms}))"
        )
        .unwrap();
    }

    writeln!(out, "            .reduce((a, b) -> b); // Placeholder reduce").unwrap();
    writeln!(out).unwrap();

    out_var
}

/// Join operator: interval join or cogroup.
fn write_join(
    out: &mut String,
    left: &str,
    right: &str,
    join_type: &str,
    on: &str,
    within: Option<&str>,
    step_idx: usize,
) -> String {
    let out_var = format!("joined{step_idx}");
    let left_var = to_camel_case(left);
    let right_var = to_camel_case(right);
    let key_getter = format!("get{}()", to_pascal_case(on));
    let window_ms = within.map(parse_duration_to_ms).unwrap_or(5000);

    writeln!(
        out,
        "        // {join_type} join: {left} x {right} on {on} within {window_ms}ms"
    )
    .unwrap();

    if join_type == "inner" {
        // Interval join
        writeln!(
            out,
            "        var {out_var} = {left_var}"
        )
        .unwrap();
        writeln!(
            out,
            "            .keyBy(l -> l.{key_getter} != null ? l.{key_getter}.toString() : \"\")"
        )
        .unwrap();
        writeln!(
            out,
            "            .intervalJoin({right_var}.keyBy(r -> r.{key_getter} != null ? r.{key_getter}.toString() : \"\"))"
        )
        .unwrap();
        writeln!(
            out,
            "            .between(org.apache.flink.streaming.api.windowing.time.Time.milliseconds(-{window_ms}), org.apache.flink.streaming.api.windowing.time.Time.milliseconds({window_ms}))"
        )
        .unwrap();
        writeln!(
            out,
            "            .process(new org.apache.flink.streaming.api.functions.co.ProcessJoinFunction<>() {{"
        )
        .unwrap();
        writeln!(out, "                @Override").unwrap();
        writeln!(
            out,
            "                public void processElement(Object left, Object right, Context ctx, Collector<Object> collector) {{"
        )
        .unwrap();
        writeln!(
            out,
            "                    collector.collect(java.util.Map.of(\"left\", left, \"right\", right));"
        )
        .unwrap();
        writeln!(out, "                }}").unwrap();
        writeln!(
            out,
            "            }}).name(\"{join_type}-join-{left}-{right}\");"
        )
        .unwrap();
    } else {
        // CoGroup for left/right/outer
        writeln!(
            out,
            "        var {out_var} = {left_var}"
        )
        .unwrap();
        writeln!(
            out,
            "            .coGroup({right_var})"
        )
        .unwrap();
        writeln!(
            out,
            "            .where(l -> l.{key_getter} != null ? l.{key_getter}.toString() : \"\")"
        )
        .unwrap();
        writeln!(
            out,
            "            .equalTo(r -> r.{key_getter} != null ? r.{key_getter}.toString() : \"\")"
        )
        .unwrap();
        writeln!(
            out,
            "            .window(org.apache.flink.streaming.api.windowing.assigners.TumblingEventTimeWindows.of(org.apache.flink.streaming.api.windowing.time.Time.milliseconds({window_ms})))"
        )
        .unwrap();
        writeln!(
            out,
            "            .apply((leftRecords, rightRecords, collector) -> {{"
        )
        .unwrap();
        writeln!(
            out,
            "                for (var l : leftRecords) for (var r : rightRecords) collector.collect(java.util.Map.of(\"left\", l, \"right\", r));"
        )
        .unwrap();
        writeln!(
            out,
            "            }}).name(\"{join_type}-join-{left}-{right}\");"
        )
        .unwrap();
    }

    writeln!(out).unwrap();
    out_var
}

/// Enrich operator: async lookup.
fn write_enrich(
    out: &mut String,
    target: &str,
    lookups: &[EnrichLookup],
    prev_stream: &str,
) -> String {
    let out_var = to_camel_case(target);

    writeln!(out, "        // Enrich: {target}").unwrap();
    for lk in lookups {
        let source_class = format!("{}LookupFunction", to_pascal_case(&lk.source));
        writeln!(
            out,
            "        var {out_var} = org.apache.flink.streaming.api.datastream.AsyncDataStream.unorderedWait("
        )
        .unwrap();
        writeln!(out, "            {prev_stream},").unwrap();
        writeln!(
            out,
            "            new {source_class}(),"
        )
        .unwrap();
        writeln!(out, "            30, java.util.concurrent.TimeUnit.SECONDS,").unwrap();
        writeln!(out, "            100").unwrap();
        writeln!(
            out,
            "        ).name(\"enrich-{}\");",
            lk.source
        )
        .unwrap();
    }
    writeln!(out).unwrap();

    out_var
}

/// Aggregate operator.
fn write_aggregate(
    out: &mut String,
    name: &str,
    fields: &[AggregateField],
    prev_stream: &str,
) -> String {
    let out_var = to_camel_case(name);

    writeln!(out, "        // Aggregate: {name}").unwrap();
    writeln!(
        out,
        "        // Fields: {}",
        fields
            .iter()
            .map(|f| format!("{}({})", f.function, f.name))
            .collect::<Vec<_>>()
            .join(", ")
    )
    .unwrap();
    writeln!(
        out,
        "        var {out_var} = {prev_stream}.map(new {}AggregateFunction()).name(\"aggregate-{name}\");",
        to_pascal_case(name)
    )
    .unwrap();
    writeln!(out).unwrap();

    out_var
}

/// Evaluate operator: delegates to L4 rules.
fn write_evaluate(
    out: &mut String,
    raw: &str,
    prev_stream: &str,
    step_idx: usize,
) -> String {
    let out_var = format!("evaluated{step_idx}");

    writeln!(out, "        // Evaluate: {raw}").unwrap();
    writeln!(
        out,
        "        var {out_var} = {prev_stream}.map(input -> {{"
    )
    .unwrap();
    writeln!(
        out,
        "            // L4 rules evaluation: {raw}"
    )
    .unwrap();
    writeln!(
        out,
        "            return input; // Placeholder -- wire L4 table here"
    )
    .unwrap();
    writeln!(out, "        }}).name(\"evaluate-{step_idx}\");").unwrap();
    writeln!(out).unwrap();

    out_var
}

fn stmt_type_name(stmt: &ProcessStatement) -> &'static str {
    match stmt {
        ProcessStatement::Branch { .. } => "branch",
        ProcessStatement::Correlation { .. } => "correlation",
        ProcessStatement::Completion { .. } => "completion",
        _ => "unknown",
    }
}

/// Parse a duration string to milliseconds (public for cross-module use).
pub fn parse_duration_to_ms_pub(duration: &str) -> u64 {
    parse_duration_to_ms(duration)
}

/// Parse a duration string to milliseconds.
fn parse_duration_to_ms(duration: &str) -> u64 {
    let trimmed = duration.trim();
    if let Some(rest) = trimmed.strip_suffix("seconds") {
        rest.trim().parse::<u64>().unwrap_or(5) * 1000
    } else if let Some(v) = trimmed.strip_suffix('s') {
        v.trim().parse::<u64>().unwrap_or(5) * 1000
    } else if let Some(rest) = trimmed.strip_suffix("minutes") {
        rest.trim().parse::<u64>().unwrap_or(1) * 60_000
    } else if let Some(v) = trimmed.strip_suffix('m') {
        v.trim().parse::<u64>().unwrap_or(1) * 60_000
    } else if let Some(rest) = trimmed.strip_suffix("hours") {
        rest.trim().parse::<u64>().unwrap_or(1) * 3_600_000
    } else if let Some(v) = trimmed.strip_suffix('h') {
        v.trim().parse::<u64>().unwrap_or(1) * 3_600_000
    } else {
        trimmed.parse::<u64>().unwrap_or(5000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_operator() {
        let mut out = String::new();
        let stmt = ProcessStatement::Transform {
            input: "raw_orders".to_string(),
            using: "normalize_order".to_string(),
            into: "normalized_orders".to_string(),
        };
        let result = write_operator(&mut out, &stmt, "rawOrders", 0);

        assert_eq!(result, "normalizedOrders");
        assert!(out.contains("NormalizeOrderFunction"));
        assert!(out.contains("rawOrders.map("));
    }

    #[test]
    fn test_route_operator() {
        let mut out = String::new();
        let stmt = ProcessStatement::Route {
            input: "events".to_string(),
            routes: vec![
                RouteCase {
                    condition: Some("value.getType().equals(\"retail\")".to_string()),
                    target: "retail_stream".to_string(),
                    is_otherwise: false,
                },
                RouteCase {
                    condition: None,
                    target: "default_stream".to_string(),
                    is_otherwise: true,
                },
            ],
        };
        let result = write_operator(&mut out, &stmt, "events", 0);

        assert_eq!(result, "routed0");
        assert!(out.contains("OutputTag<Object> retailStreamTag"));
        assert!(out.contains("OutputTag<Object> defaultStreamTag"));
        assert!(out.contains("ctx.output(retailStreamTag, value)"));
    }

    #[test]
    fn test_window_tumbling() {
        let mut out = String::new();
        let stmt = ProcessStatement::Window {
            window_type: "tumbling".to_string(),
            size: "5m".to_string(),
            group_by: vec!["account_id".to_string()],
            options: Vec::new(),
        };
        let result = write_operator(&mut out, &stmt, "stream", 0);

        assert_eq!(result, "windowed0");
        assert!(out.contains("TumblingEventTimeWindows"));
        assert!(out.contains("300000")); // 5 minutes
        assert!(out.contains("keyBy("));
    }

    #[test]
    fn test_join_inner() {
        let mut out = String::new();
        let stmt = ProcessStatement::Join {
            left: "orders".to_string(),
            right: "customers".to_string(),
            join_type: "inner".to_string(),
            on: "customer_id".to_string(),
            within: Some("10s".to_string()),
        };
        let result = write_operator(&mut out, &stmt, "stream", 0);

        assert_eq!(result, "joined0");
        assert!(out.contains("intervalJoin"));
        assert!(out.contains("getCustomerId()"));
        assert!(out.contains("10000")); // 10 seconds
    }

    #[test]
    fn test_join_left() {
        let mut out = String::new();
        let stmt = ProcessStatement::Join {
            left: "orders".to_string(),
            right: "customers".to_string(),
            join_type: "left".to_string(),
            on: "order_id".to_string(),
            within: Some("5s".to_string()),
        };
        write_operator(&mut out, &stmt, "stream", 1);

        assert!(out.contains("coGroup"));
        assert!(out.contains("TumblingEventTimeWindows"));
    }

    #[test]
    fn test_enrich_operator() {
        let mut out = String::new();
        let stmt = ProcessStatement::Enrich {
            target: "enriched_orders".to_string(),
            lookups: vec![EnrichLookup {
                field: "customer".to_string(),
                source: "customer_profile".to_string(),
                key: "customer_id".to_string(),
            }],
        };
        let result = write_operator(&mut out, &stmt, "stream", 0);

        assert_eq!(result, "enrichedOrders");
        assert!(out.contains("AsyncDataStream.unorderedWait"));
        assert!(out.contains("CustomerProfileLookupFunction"));
    }

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration_to_ms("5s"), 5000);
        assert_eq!(parse_duration_to_ms("5m"), 300_000);
        assert_eq!(parse_duration_to_ms("1h"), 3_600_000);
        assert_eq!(parse_duration_to_ms("30 seconds"), 30_000);
    }
}
