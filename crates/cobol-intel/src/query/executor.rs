use serde_json::{json, Value as JsonValue};

use crate::error::{IntelError, IntelResult};
use crate::graph::edge::EdgeKind;
use crate::graph::node::{NodeId, NodeKind, PropValue};
use crate::graph::CodeGraph;

use super::ast::*;

/// Execute a parsed NexQuery statement against a CodeGraph, producing JSON.
pub fn execute(graph: &CodeGraph, statement: &Statement) -> IntelResult<JsonValue> {
    let mut ctx = ExecContext::new(graph);

    for clause in &statement.clauses {
        ctx.execute_clause(clause)?;
    }

    ctx.to_json()
}

/// Execution context -- carries the result set between clause executions.
struct ExecContext<'g> {
    graph: &'g CodeGraph,
    result_set: Vec<NodeId>,
    /// Track what kind of nodes are in the result set.
    result_kind: Option<NodeKind>,
    /// For verb results that produce custom JSON directly.
    verb_result: Option<JsonValue>,
}

impl<'g> ExecContext<'g> {
    fn new(graph: &'g CodeGraph) -> Self {
        Self {
            graph,
            result_set: Vec::new(),
            result_kind: None,
            verb_result: None,
        }
    }

    fn execute_clause(&mut self, clause: &Clause) -> IntelResult<()> {
        match clause {
            Clause::Traverse(t) => self.execute_traverse(t),
            Clause::Expand(e) => self.execute_expand(e),
            Clause::Verb(v) => self.execute_verb(v),
        }
    }

    fn execute_traverse(&mut self, t: &TraverseClause) -> IntelResult<()> {
        let result_kind = ast_node_type_to_graph_kind(t.node_type);
        let (edge_kind, is_forward) = ast_verb_to_edge_kind(t.verb);

        // Resolve the anchor nodes (the target of the query).
        let anchor_nodes = self.resolve_target(&t.target, result_kind)?;

        // The traversal direction is INVERTED from the verb perspective:
        // "programs calling X" = programs that call X = incoming Calls to X
        //   -> verb is forward (calling), so we follow INCOMING edges from anchor
        // "programs called-by X" = programs called by X = outgoing Calls from X
        //   -> verb is reverse (called-by), so we follow OUTGOING edges from anchor
        let mut results = Vec::new();
        for node_id in &anchor_nodes {
            let neighbors = if is_forward {
                // Forward verb: the subject is the source, anchor is the target
                // -> find incoming edges to anchor
                self.graph.neighbors_incoming(*node_id, edge_kind)
            } else {
                // Reverse verb: the subject is the target, anchor is the source
                // -> find outgoing edges from anchor
                self.graph.neighbors_outgoing(*node_id, edge_kind)
            };
            results.extend(neighbors);
        }

        // Deduplicate
        results.sort();
        results.dedup();

        // Apply filter if present
        if let Some(filter) = &t.filter {
            results = self.apply_filter(&results, filter)?;
        }

        self.result_set = results;
        self.result_kind = Some(result_kind);
        self.verb_result = None;
        Ok(())
    }

    fn execute_expand(&mut self, e: &ExpandClause) -> IntelResult<()> {
        let target_kind = ast_node_type_to_graph_kind(e.node_type);

        if self.result_set.is_empty() {
            // No previous result -- return all nodes of this kind
            self.result_set = self.graph.all_of_kind(target_kind);
        } else {
            // Expand: for each node in result_set, find related nodes of target_kind.
            // Uses the implicit "each" semantics.
            let mut expanded = Vec::new();
            for node_id in &self.result_set {
                // Try all edge kinds to find connections to target_kind
                let related = self.find_related(*node_id, target_kind);
                expanded.extend(related);
            }
            expanded.sort();
            expanded.dedup();
            self.result_set = expanded;
        }

        if let Some(filter) = &e.filter {
            self.result_set = self.apply_filter(&self.result_set, filter)?;
        }

        self.result_kind = Some(target_kind);
        self.verb_result = None;
        Ok(())
    }

    fn execute_verb(&mut self, v: &VerbClause) -> IntelResult<()> {
        match v.verb {
            DomainVerb::Rank => self.execute_rank(v),
            DomainVerb::FindDead => self.execute_find_dead(v),
            DomainVerb::Trace => self.execute_trace(v),
            DomainVerb::Deps => self.execute_deps(v),
            DomainVerb::Impact => self.execute_impact(v),
            DomainVerb::Similar => self.execute_similar(v),
            DomainVerb::DiscoverProcesses => self.execute_discover_processes(v),
            DomainVerb::EstimateCost => self.execute_estimate_cost(v),
            DomainVerb::Compare => self.execute_compare_stub(v),
            DomainVerb::Save | DomainVerb::Run => {
                // Save/run are REPL-level commands, not graph queries.
                // The executor just passes them through.
                Ok(())
            }
        }
    }

    // --- Domain verb implementations ---

    fn execute_rank(&mut self, v: &VerbClause) -> IntelResult<()> {
        // If we have a target that's a node type name, populate result_set
        if let Some(target) = &v.target {
            if let Target::Identifier(name) = target {
                if let Some(nt) = NodeType::from_str(name) {
                    let kind = ast_node_type_to_graph_kind(nt);
                    self.result_set = self.graph.all_of_kind(kind);
                    self.result_kind = Some(kind);
                }
            }
        }

        // Get the "by" field
        let by_field = v
            .modifiers
            .iter()
            .find(|m| m.keyword == ModifierKeyword::By)
            .map(|m| match &m.value {
                ModifierValue::Identifier(s) => s.as_str(),
                _ => "name",
            })
            .unwrap_or("name");

        // Get the "top" limit
        let top = v
            .modifiers
            .iter()
            .find(|m| m.keyword == ModifierKeyword::Top)
            .and_then(|m| match &m.value {
                ModifierValue::Number(n) => Some(*n as usize),
                _ => None,
            });

        // Sort by property (descending for numeric, ascending for string)
        let mut ranked: Vec<(NodeId, Option<f64>)> = self
            .result_set
            .iter()
            .map(|id| {
                let val = self
                    .graph
                    .node(*id)
                    .and_then(|n| n.properties.get_f64(by_field));
                (*id, val)
            })
            .collect();

        ranked.sort_by(|a, b| {
            b.1.unwrap_or(0.0)
                .partial_cmp(&a.1.unwrap_or(0.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(limit) = top {
            ranked.truncate(limit);
        }

        self.result_set = ranked.iter().map(|(id, _)| *id).collect();

        // Build JSON with the ranked field included
        let results: Vec<JsonValue> = ranked
            .iter()
            .filter_map(|(id, val)| {
                let node = self.graph.node(*id)?;
                let mut obj = json!({ "name": node.name });
                if let Some(v) = val {
                    obj[by_field] = json!(v);
                }
                // Include other common properties
                for (key, pv) in node.properties.iter() {
                    if key != by_field {
                        obj[key] = prop_value_to_json(pv);
                    }
                }
                Some(obj)
            })
            .collect();

        self.verb_result = Some(json!({
            "count": results.len(),
            "results": results,
        }));

        Ok(())
    }

    fn execute_find_dead(&mut self, v: &VerbClause) -> IntelResult<()> {
        let level = v
            .modifiers
            .iter()
            .find(|m| m.keyword == ModifierKeyword::Level)
            .and_then(|m| match &m.value {
                ModifierValue::Identifier(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("paragraph");

        let scope_program = v
            .modifiers
            .iter()
            .find(|m| m.keyword == ModifierKeyword::In)
            .and_then(|m| match &m.value {
                ModifierValue::Identifier(s) => Some(s.as_str()),
                _ => None,
            });

        match level {
            "paragraph" => {
                // Find paragraphs that are never PERFORMed by any other paragraph
                let candidates: Vec<NodeId> = if let Some(prog) = scope_program {
                    // Scoped to a program
                    self.graph
                        .nodes_in_program(prog)
                        .iter()
                        .filter(|id| {
                            self.graph
                                .node(**id)
                                .is_some_and(|n| n.kind == NodeKind::Paragraph)
                        })
                        .copied()
                        .collect()
                } else if !self.result_set.is_empty() {
                    // Previous result set may contain programs -- expand to paragraphs
                    let mut paras = Vec::new();
                    for id in &self.result_set {
                        if let Some(node) = self.graph.node(*id) {
                            if node.kind == NodeKind::Paragraph {
                                paras.push(*id);
                            } else if node.kind == NodeKind::Program {
                                // Expand to paragraphs within this program
                                let contained =
                                    self.graph.neighbors_outgoing(*id, EdgeKind::Contains);
                                for c in contained {
                                    if self
                                        .graph
                                        .node(c)
                                        .is_some_and(|n| n.kind == NodeKind::Paragraph)
                                    {
                                        paras.push(c);
                                    }
                                }
                            }
                        }
                    }
                    paras
                } else {
                    self.graph.all_of_kind(NodeKind::Paragraph)
                };

                let dead: Vec<NodeId> = candidates
                    .into_iter()
                    .filter(|id| {
                        // Dead = no incoming Performs edges
                        self.graph
                            .neighbors_incoming(*id, EdgeKind::Performs)
                            .is_empty()
                    })
                    .collect();

                self.result_set = dead;
                self.result_kind = Some(NodeKind::Paragraph);
            }
            "program" => {
                // Find programs never CALLed by any other program
                let all_programs = if !self.result_set.is_empty() {
                    self.result_set
                        .iter()
                        .filter(|id| {
                            self.graph
                                .node(**id)
                                .is_some_and(|n| n.kind == NodeKind::Program)
                        })
                        .copied()
                        .collect()
                } else {
                    self.graph.all_of_kind(NodeKind::Program)
                };

                let dead: Vec<NodeId> = all_programs
                    .into_iter()
                    .filter(|id| {
                        // Dead program = never called AND not a top-level entry point
                        // For now: never called by anything
                        let callers = self.graph.neighbors_incoming(*id, EdgeKind::Calls);
                        let callees = self.graph.neighbors_outgoing(*id, EdgeKind::Calls);
                        let contains = self.graph.neighbors_outgoing(*id, EdgeKind::Contains);
                        // A truly dead program has no callers, no callees, no paragraphs
                        // (or just: no callers -- that makes it a potential entry point OR dead)
                        // For simplicity: no incoming calls = candidate
                        callers.is_empty()
                            && callees.is_empty()
                            && contains.is_empty()
                    })
                    .collect();

                self.result_set = dead;
                self.result_kind = Some(NodeKind::Program);
            }
            other => {
                return Err(IntelError::QueryError {
                    reason: format!("find-dead: unsupported level '{other}'"),
                });
            }
        }

        self.verb_result = None;
        Ok(())
    }

    fn execute_trace(&mut self, v: &VerbClause) -> IntelResult<()> {
        let prog_name = match &v.target {
            Some(Target::Identifier(name)) => name.clone(),
            _ => {
                return Err(IntelError::QueryError {
                    reason: "trace requires a program name".to_owned(),
                });
            }
        };

        let prog_id = self.graph.lookup_one(NodeKind::Program, &prog_name)?;
        let prog_node = self.graph.node(prog_id).ok_or_else(|| IntelError::QueryError {
            reason: format!("program '{prog_name}' not found"),
        })?;

        // Collect all program properties into entry
        let mut entry = json!({ "program": prog_node.name });
        for (key, val) in prog_node.properties.iter() {
            entry[key] = prop_value_to_json(val);
        }

        // Execution tree: paragraphs with their intelligence data
        let contained = self.graph.neighbors_outgoing(prog_id, EdgeKind::Contains);
        let mut para_details: Vec<JsonValue> = Vec::new();
        let mut dead_paragraphs: Vec<String> = Vec::new();

        for child_id in &contained {
            if let Some(child) = self.graph.node(*child_id) {
                if child.kind != NodeKind::Paragraph {
                    continue;
                }
                let mut para = json!({ "name": child.name });
                if child.properties.get_bool("is_entry_point") == Some(true) {
                    para["is_entry_point"] = json!(true);
                }
                if child.properties.get_bool("is_dead") == Some(true) {
                    para["is_dead"] = json!(true);
                    dead_paragraphs.push(child.name.clone());
                }
                if child.properties.get_bool("has_rules") == Some(true) {
                    para["has_rules"] = json!(true);
                    if let Some(rc) = child.properties.get_u64("rule_count") {
                        para["rule_count"] = json!(rc);
                    }
                }
                // PERFORM targets
                let performs = self.graph.neighbors_outgoing(*child_id, EdgeKind::Performs);
                if !performs.is_empty() {
                    let targets: Vec<String> = performs
                        .iter()
                        .filter_map(|id| self.graph.node(*id).map(|n| n.name.clone()))
                        .collect();
                    para["performs"] = json!(targets);
                }
                para_details.push(para);
            }
        }

        // Call graph
        let calls = self.graph.neighbors_outgoing(prog_id, EdgeKind::Calls);
        let call_details: Vec<JsonValue> = calls
            .iter()
            .filter_map(|id| {
                let node = self.graph.node(*id)?;
                let mut obj = json!({ "target": node.name });
                if let Some(c) = node.properties.get_f64("complexity") {
                    obj["complexity"] = json!(c);
                }
                if let Some(w) = node.properties.get_u64("migration_wave") {
                    obj["migration_wave"] = json!(w);
                }
                Some(obj)
            })
            .collect();

        // Data touched
        let copybooks = self.graph.neighbors_outgoing(prog_id, EdgeKind::Uses);
        let copybook_names: Vec<String> = copybooks
            .iter()
            .filter_map(|id| self.graph.node(*id).map(|n| n.name.clone()))
            .collect();

        let accessed = self.graph.neighbors_outgoing(prog_id, EdgeKind::Accesses);
        let mut file_names = Vec::new();
        let mut table_names = Vec::new();
        for target_id in &accessed {
            if let Some(target) = self.graph.node(*target_id) {
                match target.kind {
                    NodeKind::File => file_names.push(target.name.clone()),
                    NodeKind::Table => table_names.push(target.name.clone()),
                    _ => {}
                }
            }
        }

        let mut result = json!({
            "entry": entry,
            "execution_tree": {
                "paragraph_count": para_details.len(),
                "paragraphs": para_details,
                "calls": call_details,
            },
            "data_touched": {
                "copybooks": copybook_names,
                "files": file_names,
                "tables": table_names,
            },
        });

        if !dead_paragraphs.is_empty() {
            result["dead_paragraphs"] = json!(dead_paragraphs);
        }

        self.verb_result = Some(result);

        // Populate result_set with all involved programs (for chaining)
        self.result_set = std::iter::once(prog_id).chain(calls).collect();
        self.result_kind = Some(NodeKind::Program);

        Ok(())
    }

    fn execute_deps(&mut self, v: &VerbClause) -> IntelResult<()> {
        let prog_name = match &v.target {
            Some(Target::Identifier(name)) => name.clone(),
            _ => {
                return Err(IntelError::QueryError {
                    reason: "deps requires a program name".to_owned(),
                });
            }
        };

        let prog_id = self.graph.lookup_one(NodeKind::Program, &prog_name)?;

        // BFS to find all transitive dependencies
        let mut visited = Vec::new();
        let mut queue = vec![prog_id];

        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.push(current);
            let callees = self.graph.neighbors_outgoing(current, EdgeKind::Calls);
            queue.extend(callees);
        }

        self.result_set = visited;
        self.result_kind = Some(NodeKind::Program);
        self.verb_result = None;

        Ok(())
    }

    fn execute_impact(&mut self, v: &VerbClause) -> IntelResult<()> {
        let target_name = match &v.target {
            Some(Target::Identifier(name)) => name.clone(),
            _ => {
                return Err(IntelError::QueryError {
                    reason: "impact requires a target name".to_owned(),
                });
            }
        };

        // Try to find as copybook first, then program, then field
        let mut impacted = Vec::new();

        if let Ok(id) = self.graph.lookup_one(NodeKind::Copybook, &target_name) {
            // Programs using this copybook
            let users = self.graph.neighbors_incoming(id, EdgeKind::Uses);
            impacted.extend(users);
        } else if let Ok(id) = self.graph.lookup_one(NodeKind::Program, &target_name) {
            // Programs that call this program (reverse call graph)
            let mut queue = vec![id];
            let mut visited = Vec::new();
            while let Some(current) = queue.pop() {
                if visited.contains(&current) {
                    continue;
                }
                visited.push(current);
                let callers = self.graph.neighbors_incoming(current, EdgeKind::Calls);
                queue.extend(callers);
            }
            impacted = visited;
        }

        self.result_set = impacted;
        self.result_kind = Some(NodeKind::Program);
        self.verb_result = None;

        Ok(())
    }

    fn execute_similar(&mut self, v: &VerbClause) -> IntelResult<()> {
        let prog_name = match &v.target {
            Some(Target::Identifier(name)) => name.clone(),
            _ => {
                return Err(IntelError::QueryError {
                    reason: "similar requires a program name".to_owned(),
                });
            }
        };

        let prog_id = self.graph.lookup_one(NodeKind::Program, &prog_name)?;
        let prog_node = self.graph.node(prog_id).ok_or_else(|| IntelError::QueryError {
            reason: format!("program '{prog_name}' not found"),
        })?;

        // Use the fingerprint from Layer 8 patterns pass
        let fingerprint = prog_node
            .properties
            .get_str("fingerprint")
            .unwrap_or("")
            .to_owned();

        // Find all programs with the same fingerprint
        let mut similar = Vec::new();
        if !fingerprint.is_empty() {
            for id in self.graph.all_of_kind(NodeKind::Program) {
                if id == prog_id {
                    continue;
                }
                if let Some(node) = self.graph.node(id) {
                    if node
                        .properties
                        .get_str("fingerprint")
                        .is_some_and(|fp| fp == fingerprint)
                    {
                        similar.push(id);
                    }
                }
            }
        }

        self.result_set = similar;
        self.result_kind = Some(NodeKind::Program);
        self.verb_result = None;
        Ok(())
    }

    fn execute_discover_processes(&mut self, _v: &VerbClause) -> IntelResult<()> {
        // Group programs by their `process` property (set by ProcessDiscoveryPass)
        let all_programs = self.graph.all_of_kind(NodeKind::Program);

        let mut processes: std::collections::HashMap<String, Vec<serde_json::Value>> =
            std::collections::HashMap::new();

        for prog_id in &all_programs {
            if let Some(node) = self.graph.node(*prog_id) {
                let process_name = node
                    .properties
                    .get_str("process")
                    .unwrap_or("UNASSIGNED")
                    .to_owned();
                let role = node
                    .properties
                    .get_str("process_role")
                    .unwrap_or("unknown");
                let complexity = node.properties.get_f64("complexity").unwrap_or(0.0);

                processes.entry(process_name).or_default().push(json!({
                    "program": node.name,
                    "role": role,
                    "complexity": complexity,
                }));
            }
        }

        // Build summary
        let mut process_list: Vec<serde_json::Value> = processes
            .into_iter()
            .map(|(name, members)| {
                let program_count = members.len();
                let total_complexity: f64 = members
                    .iter()
                    .filter_map(|m| m["complexity"].as_f64())
                    .sum();
                let core_count = members
                    .iter()
                    .filter(|m| m["role"].as_str() == Some("core"))
                    .count();
                let coupling = if program_count > 0 {
                    core_count as f64 / program_count as f64
                } else {
                    0.0
                };

                json!({
                    "process": name,
                    "program_count": program_count,
                    "total_complexity": (total_complexity * 10.0).round() / 10.0,
                    "coupling_score": (coupling * 100.0).round() / 100.0,
                    "programs": members,
                })
            })
            .collect();

        // Sort by program count descending
        process_list.sort_by(|a, b| {
            b["program_count"]
                .as_u64()
                .cmp(&a["program_count"].as_u64())
        });

        self.verb_result = Some(json!({
            "process_count": process_list.len(),
            "processes": process_list,
        }));

        Ok(())
    }

    fn execute_estimate_cost(&mut self, v: &VerbClause) -> IntelResult<()> {
        // Parse scope modifier: estimate-cost scope all / scope process CLRG / scope wave 0
        let scope = v
            .modifiers
            .iter()
            .find(|m| m.keyword == ModifierKeyword::Scope)
            .and_then(|m| match &m.value {
                ModifierValue::Identifier(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("all");

        // The scope value is the target (e.g., process name or wave number)
        let scope_value = v.target.as_ref().and_then(|t| match t {
            Target::Identifier(s) => Some(s.as_str()),
            _ => None,
        });

        let config = crate::intel::cost::config::CostConfig::default();
        let result = crate::intel::cost::aggregate_estimates(
            self.graph,
            scope,
            scope_value,
            &config,
        );

        self.verb_result = Some(result);
        Ok(())
    }

    fn execute_compare_stub(&mut self, _v: &VerbClause) -> IntelResult<()> {
        self.verb_result = Some(json!({
            "error": "compare: not yet implemented",
        }));
        Ok(())
    }

    // --- Helpers ---

    fn resolve_target(
        &self,
        target: &Target,
        _hint_kind: NodeKind,
    ) -> IntelResult<Vec<NodeId>> {
        match target {
            Target::Identifier(name) => {
                // Try to find by name across all node kinds
                for kind in ALL_NODE_KINDS {
                    let ids = self.graph.index().lookup(*kind, name);
                    if !ids.is_empty() {
                        return Ok(ids.to_vec());
                    }
                }
                // Not found -- return empty (not an error, just no matches)
                Ok(Vec::new())
            }
            Target::Each => {
                // "each" references the previous result set
                Ok(self.result_set.clone())
            }
            Target::List(names) => {
                let mut ids = Vec::new();
                for name in names {
                    for kind in ALL_NODE_KINDS {
                        let found = self.graph.index().lookup(*kind, name);
                        ids.extend(found.iter().copied());
                    }
                }
                Ok(ids)
            }
        }
    }

    fn find_related(&self, node_id: NodeId, target_kind: NodeKind) -> Vec<NodeId> {
        let node = match self.graph.node(node_id) {
            Some(n) => n,
            None => return Vec::new(),
        };

        // For Program -> Field: fields don't have a direct edge to programs,
        // they have a `program` property. Use the index.
        if node.kind == NodeKind::Program && target_kind == NodeKind::Field {
            return self
                .graph
                .nodes_in_program(&node.name)
                .iter()
                .filter(|id| {
                    self.graph
                        .node(**id)
                        .is_some_and(|n| n.kind == NodeKind::Field)
                })
                .copied()
                .collect();
        }

        // Pick the appropriate edge kind based on source/target node kinds
        let edges_to_try = match (node.kind, target_kind) {
            (NodeKind::Program, NodeKind::Paragraph) => vec![(EdgeKind::Contains, true)],
            (NodeKind::Program, NodeKind::Copybook) => vec![(EdgeKind::Uses, true)],
            (NodeKind::Program, NodeKind::File) | (NodeKind::Program, NodeKind::Table) => {
                vec![(EdgeKind::Accesses, true)]
            }
            (NodeKind::Program, NodeKind::Program) => vec![(EdgeKind::Calls, true)],
            (NodeKind::Paragraph, NodeKind::Field) => {
                vec![(EdgeKind::Reads, true), (EdgeKind::Writes, true)]
            }
            (NodeKind::Paragraph, NodeKind::Paragraph) => vec![(EdgeKind::Performs, true)],
            (NodeKind::Paragraph, NodeKind::Program) => vec![(EdgeKind::Contains, false)],
            (NodeKind::Copybook, NodeKind::Program) => vec![(EdgeKind::Uses, false)],
            (NodeKind::Field, NodeKind::Paragraph) => {
                vec![(EdgeKind::Reads, false), (EdgeKind::Writes, false)]
            }
            (NodeKind::File, NodeKind::Program) | (NodeKind::Table, NodeKind::Program) => {
                vec![(EdgeKind::Accesses, false)]
            }
            _ => {
                // Try all edges and filter by target kind
                return self.find_related_any_edge(node_id, target_kind);
            }
        };

        let mut results = Vec::new();
        for (edge_kind, is_forward) in edges_to_try {
            let neighbors = if is_forward {
                self.graph.neighbors_outgoing(node_id, edge_kind)
            } else {
                self.graph.neighbors_incoming(node_id, edge_kind)
            };
            for n in neighbors {
                if self
                    .graph
                    .node(n)
                    .is_some_and(|node| node.kind == target_kind)
                {
                    results.push(n);
                }
            }
        }
        results
    }

    fn find_related_any_edge(&self, node_id: NodeId, target_kind: NodeKind) -> Vec<NodeId> {
        let mut results = Vec::new();
        for edge_kind in ALL_EDGE_KINDS {
            for n in self.graph.neighbors_outgoing(node_id, *edge_kind) {
                if self
                    .graph
                    .node(n)
                    .is_some_and(|node| node.kind == target_kind)
                {
                    results.push(n);
                }
            }
            for n in self.graph.neighbors_incoming(node_id, *edge_kind) {
                if self
                    .graph
                    .node(n)
                    .is_some_and(|node| node.kind == target_kind)
                {
                    results.push(n);
                }
            }
        }
        results.sort();
        results.dedup();
        results
    }

    fn apply_filter(
        &self,
        nodes: &[NodeId],
        filter: &Filter,
    ) -> IntelResult<Vec<NodeId>> {
        let mut result = Vec::new();
        for id in nodes {
            if let Some(node) = self.graph.node(*id) {
                let passes = filter
                    .predicates
                    .iter()
                    .all(|expr| eval_filter_expr(expr, &node.properties));
                if passes {
                    result.push(*id);
                }
            }
        }
        Ok(result)
    }

    fn to_json(&self) -> IntelResult<JsonValue> {
        // If a verb produced custom JSON, return that
        if let Some(ref result) = self.verb_result {
            return Ok(result.clone());
        }

        // Otherwise, serialize the result set
        let results: Vec<JsonValue> = self
            .result_set
            .iter()
            .filter_map(|id| {
                let node = self.graph.node(*id)?;
                let mut obj = json!({
                    "name": node.name,
                    "kind": node.kind.as_str(),
                });
                if let Some(prog) = &node.program {
                    obj["program"] = json!(prog);
                }
                for (key, val) in node.properties.iter() {
                    obj[key] = prop_value_to_json(val);
                }
                Some(obj)
            })
            .collect();

        Ok(json!({
            "count": results.len(),
            "results": results,
        }))
    }
}

// --- Filter evaluation ---

fn eval_filter_expr(
    expr: &FilterExpr,
    props: &crate::graph::node::Properties,
) -> bool {
    match expr {
        FilterExpr::Predicate(p) => eval_predicate(p, props),
        FilterExpr::And(a, b) => eval_filter_expr(a, props) && eval_filter_expr(b, props),
        FilterExpr::Or(a, b) => eval_filter_expr(a, props) || eval_filter_expr(b, props),
        FilterExpr::Not(inner) => !eval_filter_expr(inner, props),
    }
}

fn eval_predicate(
    pred: &Predicate,
    props: &crate::graph::node::Properties,
) -> bool {
    let prop = props.get(&pred.field);

    match pred.op {
        CompareOp::Eq => match (&pred.value, prop) {
            (Value::String(s), Some(PropValue::String(ps))) => s == ps,
            (Value::Number(n), Some(PropValue::F64(pn))) => (*n - pn).abs() < f64::EPSILON,
            (Value::Number(n), Some(PropValue::U64(pn))) => (*n as u64) == *pn,
            _ => false,
        },
        CompareOp::NotEq => !eval_predicate(
            &Predicate {
                field: pred.field.clone(),
                op: CompareOp::Eq,
                value: pred.value.clone(),
            },
            props,
        ),
        CompareOp::Gt => match (&pred.value, prop) {
            (Value::Number(n), Some(PropValue::F64(pn))) => *pn > *n,
            (Value::Number(n), Some(PropValue::U64(pn))) => (*pn as f64) > *n,
            _ => false,
        },
        CompareOp::Lt => match (&pred.value, prop) {
            (Value::Number(n), Some(PropValue::F64(pn))) => *pn < *n,
            (Value::Number(n), Some(PropValue::U64(pn))) => (*pn as f64) < *n,
            _ => false,
        },
        CompareOp::Gte => match (&pred.value, prop) {
            (Value::Number(n), Some(PropValue::F64(pn))) => *pn >= *n,
            (Value::Number(n), Some(PropValue::U64(pn))) => (*pn as f64) >= *n,
            _ => false,
        },
        CompareOp::Lte => match (&pred.value, prop) {
            (Value::Number(n), Some(PropValue::F64(pn))) => *pn <= *n,
            (Value::Number(n), Some(PropValue::U64(pn))) => (*pn as f64) <= *n,
            _ => false,
        },
        CompareOp::Glob => match (&pred.value, prop) {
            (Value::String(pattern), Some(PropValue::String(ps))) => glob_match(pattern, ps),
            _ => false,
        },
        CompareOp::Regex => match (&pred.value, prop) {
            (Value::String(_pattern), Some(PropValue::String(_ps))) => {
                // Regex support deferred -- return true for now
                true
            }
            _ => false,
        },
        CompareOp::In => match (&pred.value, prop) {
            (Value::List(items), Some(PropValue::String(ps))) => items.contains(ps),
            _ => false,
        },
        CompareOp::Has => match (&pred.value, prop) {
            (Value::String(s), Some(PropValue::List(list))) => list.contains(s),
            _ => false,
        },
    }
}

/// Simple glob matching: only supports * as wildcard.
fn glob_match(pattern: &str, text: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(prefix) = pattern.strip_suffix('*') {
        return text.starts_with(prefix);
    }
    if let Some(suffix) = pattern.strip_prefix('*') {
        return text.ends_with(suffix);
    }
    pattern == text
}

fn prop_value_to_json(val: &PropValue) -> JsonValue {
    match val {
        PropValue::String(s) => json!(s),
        PropValue::U64(n) => json!(n),
        PropValue::F64(n) => json!(n),
        PropValue::Bool(b) => json!(b),
        PropValue::List(v) => json!(v),
    }
}

fn ast_node_type_to_graph_kind(nt: NodeType) -> NodeKind {
    match nt {
        NodeType::Programs => NodeKind::Program,
        NodeType::Paragraphs => NodeKind::Paragraph,
        NodeType::Fields => NodeKind::Field,
        NodeType::Copybooks => NodeKind::Copybook,
        NodeType::Files => NodeKind::File,
        NodeType::Tables => NodeKind::Table,
        NodeType::Rules => NodeKind::Rule,
    }
}

fn ast_verb_to_edge_kind(verb: TraversalVerb) -> (EdgeKind, bool) {
    match verb {
        TraversalVerb::Calling => (EdgeKind::Calls, true),
        TraversalVerb::CalledBy => (EdgeKind::Calls, false),
        TraversalVerb::Performing => (EdgeKind::Performs, true),
        TraversalVerb::PerformedBy => (EdgeKind::Performs, false),
        TraversalVerb::Reading => (EdgeKind::Reads, true),
        TraversalVerb::ReadBy => (EdgeKind::Reads, false),
        TraversalVerb::Writing => (EdgeKind::Writes, true),
        TraversalVerb::WrittenBy => (EdgeKind::Writes, false),
        TraversalVerb::Using => (EdgeKind::Uses, true),
        TraversalVerb::UsedBy => (EdgeKind::Uses, false),
        TraversalVerb::Accessing => (EdgeKind::Accesses, true),
        TraversalVerb::AccessedBy => (EdgeKind::Accesses, false),
        TraversalVerb::Containing => (EdgeKind::Contains, true),
        TraversalVerb::Within => (EdgeKind::Contains, false),
    }
}

const ALL_NODE_KINDS: &[NodeKind] = &[
    NodeKind::Program,
    NodeKind::Paragraph,
    NodeKind::Field,
    NodeKind::Copybook,
    NodeKind::File,
    NodeKind::Table,
    NodeKind::Rule,
];

const ALL_EDGE_KINDS: &[EdgeKind] = &[
    EdgeKind::Calls,
    EdgeKind::Performs,
    EdgeKind::Reads,
    EdgeKind::Writes,
    EdgeKind::Uses,
    EdgeKind::Accesses,
    EdgeKind::Contains,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::Edge;
    use crate::graph::node::Node;

    /// Build a test graph matching the spec example:
    /// CLRG0100 calls VALUTIL, BALUPD
    /// CLRG0100 contains 1000-MAIN, 2000-VALIDATE, 3000-UPDATE
    /// 1000-MAIN performs 2000-VALIDATE, 3000-UPDATE
    /// 2000-VALIDATE reads WS-ACCT-NBR
    /// 3000-UPDATE writes WS-ACCT-BAL
    /// CLRG0100 uses CPYCLRG, CPYACCT
    /// VALUTIL uses CPYACCT
    fn make_test_graph() -> CodeGraph {
        let mut g = CodeGraph::new();

        let clrg = g.add_node(
            Node::new(NodeKind::Program, "CLRG0100")
                .with_property("type", PropValue::from("batch"))
                .with_property("complexity", PropValue::from(4.2f64))
                .with_property("loc", PropValue::from(1847u64)),
        );
        let valutil = g.add_node(
            Node::new(NodeKind::Program, "VALUTIL")
                .with_property("type", PropValue::from("subprogram"))
                .with_property("complexity", PropValue::from(2.1f64))
                .with_property("loc", PropValue::from(500u64)),
        );
        let balupd = g.add_node(
            Node::new(NodeKind::Program, "BALUPD")
                .with_property("type", PropValue::from("subprogram"))
                .with_property("complexity", PropValue::from(3.5f64))
                .with_property("loc", PropValue::from(800u64)),
        );
        // A dead program (no edges at all)
        let _dead_prog = g.add_node(
            Node::new(NodeKind::Program, "DEADPROG")
                .with_property("type", PropValue::from("batch"))
                .with_property("complexity", PropValue::from(1.0f64)),
        );

        let main_para = g.add_node(
            Node::new(NodeKind::Paragraph, "1000-MAIN").with_program("CLRG0100"),
        );
        let validate = g.add_node(
            Node::new(NodeKind::Paragraph, "2000-VALIDATE").with_program("CLRG0100"),
        );
        let update = g.add_node(
            Node::new(NodeKind::Paragraph, "3000-UPDATE").with_program("CLRG0100"),
        );

        let acct_nbr = g.add_node(
            Node::new(NodeKind::Field, "WS-ACCT-NBR").with_program("CLRG0100"),
        );
        let acct_bal = g.add_node(
            Node::new(NodeKind::Field, "WS-ACCT-BAL").with_program("CLRG0100"),
        );

        let cpyclrg = g.add_node(Node::new(NodeKind::Copybook, "CPYCLRG"));
        let cpyacct = g.add_node(Node::new(NodeKind::Copybook, "CPYACCT"));

        // Edges
        g.add_edge(clrg, valutil, Edge::new(EdgeKind::Calls));
        g.add_edge(clrg, balupd, Edge::new(EdgeKind::Calls));
        g.add_edge(clrg, main_para, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, validate, Edge::new(EdgeKind::Contains));
        g.add_edge(clrg, update, Edge::new(EdgeKind::Contains));
        g.add_edge(main_para, validate, Edge::new(EdgeKind::Performs));
        g.add_edge(main_para, update, Edge::new(EdgeKind::Performs));
        g.add_edge(validate, acct_nbr, Edge::new(EdgeKind::Reads));
        g.add_edge(update, acct_bal, Edge::new(EdgeKind::Writes));
        g.add_edge(clrg, cpyclrg, Edge::new(EdgeKind::Uses));
        g.add_edge(clrg, cpyacct, Edge::new(EdgeKind::Uses));
        g.add_edge(valutil, cpyacct, Edge::new(EdgeKind::Uses));

        g.index_copybook_usage("CPYCLRG", clrg);
        g.index_copybook_usage("CPYACCT", clrg);
        g.index_copybook_usage("CPYACCT", valutil);

        g
    }

    fn exec(graph: &CodeGraph, input: &str) -> JsonValue {
        let script = crate::query::parse(input).unwrap();
        execute(graph, &script.statements[0]).unwrap()
    }

    // === Traverse queries ===

    #[test]
    fn traverse_programs_calling() {
        let g = make_test_graph();
        let result = exec(&g, "programs called-by CLRG0100;");
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 2); // VALUTIL, BALUPD
    }

    #[test]
    fn traverse_copybooks_used_by() {
        let g = make_test_graph();
        let result = exec(&g, "copybooks used-by CLRG0100;");
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 2); // CPYCLRG, CPYACCT
    }

    #[test]
    fn traverse_with_filter() {
        let g = make_test_graph();
        let result = exec(
            &g,
            "programs called-by CLRG0100(complexity > 3.0);",
        );
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 1); // Only BALUPD has complexity > 3.0
        assert_eq!(result["results"][0]["name"], "BALUPD");
    }

    #[test]
    fn traverse_with_list_target() {
        let g = make_test_graph();
        let result = exec(&g, "copybooks used-by [CLRG0100, VALUTIL];");
        // CLRG0100 uses CPYCLRG, CPYACCT; VALUTIL uses CPYACCT
        // Deduplicated: CPYCLRG, CPYACCT
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 2);
    }

    // === Multi-clause pipeline ===

    #[test]
    fn pipeline_traverse_then_expand() {
        let g = make_test_graph();
        let result = exec(&g, "programs called-by CLRG0100\ncopybooks;");
        // CLRG0100 calls VALUTIL, BALUPD.
        // VALUTIL uses CPYACCT. BALUPD uses nothing.
        // So copybooks of called programs = CPYACCT
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 1);
        assert_eq!(result["results"][0]["name"], "CPYACCT");
    }

    #[test]
    fn pipeline_traverse_then_each() {
        let g = make_test_graph();
        let result = exec(
            &g,
            "copybooks used-by CLRG0100\nprograms using each;",
        );
        // CLRG0100 uses CPYCLRG, CPYACCT
        // Programs using CPYCLRG = CLRG0100
        // Programs using CPYACCT = CLRG0100, VALUTIL
        // Deduplicated: CLRG0100, VALUTIL
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 2);
    }

    // === Domain verbs ===

    #[test]
    fn rank_by_complexity() {
        let g = make_test_graph();
        let result = exec(&g, "rank programs by complexity top 2;");
        let results = result["results"].as_array().unwrap();
        assert_eq!(results.len(), 2);
        // Should be sorted descending by complexity
        assert_eq!(results[0]["name"], "CLRG0100"); // 4.2
        assert_eq!(results[1]["name"], "BALUPD"); // 3.5
    }

    #[test]
    fn find_dead_paragraphs() {
        let g = make_test_graph();
        let result = exec(&g, "find-dead level paragraph in CLRG0100;");
        let count = result["count"].as_u64().unwrap();
        // 1000-MAIN is never PERFORMed by anything -> dead
        // 2000-VALIDATE and 3000-UPDATE are performed by 1000-MAIN -> alive
        assert_eq!(count, 1);
        assert_eq!(result["results"][0]["name"], "1000-MAIN");
    }

    #[test]
    fn find_dead_programs() {
        let g = make_test_graph();
        let result = exec(&g, "find-dead level program scope all;");
        let count = result["count"].as_u64().unwrap();
        // DEADPROG has no edges at all -> dead
        assert_eq!(count, 1);
        assert_eq!(result["results"][0]["name"], "DEADPROG");
    }

    #[test]
    fn trace_program() {
        let g = make_test_graph();
        let result = exec(&g, "trace CLRG0100 depth full;");
        assert_eq!(result["entry"]["program"], "CLRG0100");
        assert_eq!(result["execution_tree"]["paragraph_count"], 3);
        let paras = result["execution_tree"]["paragraphs"].as_array().unwrap();
        assert_eq!(paras.len(), 3);
        let calls = result["execution_tree"]["calls"].as_array().unwrap();
        assert_eq!(calls.len(), 2);
        let copybooks = result["data_touched"]["copybooks"].as_array().unwrap();
        assert_eq!(copybooks.len(), 2);
    }

    #[test]
    fn deps_transitive() {
        let g = make_test_graph();
        let result = exec(&g, "deps CLRG0100 order topological;");
        let count = result["count"].as_u64().unwrap();
        // CLRG0100 + VALUTIL + BALUPD = 3
        assert_eq!(count, 3);
    }

    #[test]
    fn impact_copybook() {
        let g = make_test_graph();
        let result = exec(&g, "impact CPYACCT;");
        let count = result["count"].as_u64().unwrap();
        // CPYACCT is used by CLRG0100 and VALUTIL
        assert_eq!(count, 2);
    }

    // === Composed queries from the spec ===

    #[test]
    fn spec_trace_then_find_dead() {
        let g = make_test_graph();
        let result = exec(&g, "trace CLRG0100\nfind-dead level paragraph;");
        // After trace, result_set has programs. find-dead works on paragraphs
        // within those programs. 1000-MAIN is not performed by anything.
        let count = result["count"].as_u64().unwrap();
        assert!(count >= 1);
    }

    #[test]
    fn spec_trace_then_fields() {
        let g = make_test_graph();
        let result = exec(&g, "trace CLRG0100\nfields;");
        // After trace, result_set has programs (CLRG0100, VALUTIL, BALUPD)
        // Expand to fields: WS-ACCT-NBR, WS-ACCT-BAL (belonging to CLRG0100)
        let count = result["count"].as_u64().unwrap();
        assert!(count >= 1);
    }

    // === Expand ===

    #[test]
    fn expand_all_programs() {
        let g = make_test_graph();
        let result = exec(&g, "programs;");
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 4); // CLRG0100, VALUTIL, BALUPD, DEADPROG
    }

    // === Filter operators ===

    #[test]
    fn filter_glob() {
        let g = make_test_graph();
        let result = exec(&g, "programs called-by CLRG0100(type ~ 'sub*');");
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 2); // both VALUTIL and BALUPD have type "subprogram"
    }

    #[test]
    fn filter_not_eq() {
        let g = make_test_graph();
        let result = exec(&g, "programs called-by CLRG0100(not type = 'subprogram');");
        let count = result["count"].as_u64().unwrap();
        assert_eq!(count, 0); // all called programs are subprograms
    }

    // === Error cases ===

    #[test]
    fn trace_missing_target() {
        let g = make_test_graph();
        let script = crate::query::parse("trace NONEXISTENT;").unwrap();
        let err = execute(&g, &script.statements[0]).unwrap_err();
        assert!(err.to_string().contains("not found"), "got: {err}");
    }
}
