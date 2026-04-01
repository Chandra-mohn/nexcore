// NexCore -- Nexflow Parser: ProcDSL AST
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Typed AST for `.proc` files (ProcDSL grammar).

use serde::{Deserialize, Serialize};

use super::common::ImportPath;

/// Top-level proc program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcProgram {
    pub imports: Vec<ImportPath>,
    pub processes: Vec<ProcessDef>,
}

/// A process definition (streaming or batch pipeline).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessDef {
    pub name: String,
    pub execution: Option<ExecutionBlock>,
    pub business_date: Option<String>,
    pub processing_date: Option<String>,
    pub markers: Vec<MarkerDecl>,
    pub state_machine: Option<String>,
    pub body: Vec<ProcessStatement>,
    pub phases: Vec<PhaseBlock>,
    pub state: Option<StateBlock>,
    pub metrics: Option<MetricsBlock>,
    pub resilience: Option<ResilienceBlock>,
}

/// Execution configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionBlock {
    pub mode: Option<String>,
    pub parallelism: Option<i64>,
    pub checkpoint_interval: Option<String>,
}

/// Marker declaration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerDecl {
    pub name: String,
    pub raw: String,
}

/// A process statement (receive, enrich, transform, route, emit, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProcessStatement {
    Receive {
        name: String,
        source_type: String,
        source: String,
        schema: Option<String>,
        key: Option<String>,
        options: Vec<(String, String)>,
    },
    Enrich {
        target: String,
        lookups: Vec<EnrichLookup>,
    },
    Transform {
        input: String,
        using: String,
        into: String,
    },
    Route {
        input: String,
        routes: Vec<RouteCase>,
    },
    Emit {
        name: String,
        sink_type: String,
        sink: String,
        schema: Option<String>,
        key: Option<String>,
    },
    Window {
        window_type: String,
        size: String,
        group_by: Vec<String>,
        options: Vec<(String, String)>,
    },
    Aggregate {
        name: String,
        fields: Vec<AggregateField>,
    },
    Join {
        left: String,
        right: String,
        join_type: String,
        on: String,
        within: Option<String>,
    },
    Evaluate {
        raw: String,
    },
    Branch {
        raw: String,
    },
    Correlation {
        raw: String,
    },
    Completion {
        raw: String,
    },
}

/// An enrich lookup.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnrichLookup {
    pub field: String,
    pub source: String,
    pub key: String,
}

/// A route case.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteCase {
    pub condition: Option<String>,
    pub target: String,
    pub is_otherwise: bool,
}

/// An aggregate field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateField {
    pub name: String,
    pub function: String,
}

/// Phase block (for phased processing).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhaseBlock {
    pub name: String,
    pub statements: Vec<ProcessStatement>,
}

/// State block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateBlock {
    pub raw: String,
}

/// Metrics block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricsBlock {
    pub raw: String,
}

/// Resilience block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResilienceBlock {
    pub raw: String,
}
