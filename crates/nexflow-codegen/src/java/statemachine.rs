// NexCore -- Nexflow Codegen: Java StateMachine Generator
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates `*StateMachine.java` classes for schemas with state_machine blocks.
//!
//! Provides state validation, transition checking, and state query methods.

use std::fmt::Write;

use nexflow_parser::ast::schema::{SchemaDefinition, StateQualifier, TransitionDecl};

use super::naming::{to_constant_case, to_pascal_case};

const DEFAULT_PACKAGE: &str = "com.nexflow.schemas";

/// Check whether a schema has a state machine block.
pub fn has_state_machine(schema: &SchemaDefinition) -> bool {
    schema.state_machine.is_some()
}

/// Generate a StateMachine Java class for the given schema.
///
/// Returns `Some((filename, content))` if the schema has a state machine, `None` otherwise.
pub fn generate_state_machine(schema: &SchemaDefinition) -> Option<(String, String)> {
    let sm = schema.state_machine.as_ref()?;

    let class_name = to_pascal_case(&schema.name);
    let sm_name = format!("{class_name}StateMachine");
    let filename = format!("{sm_name}.java");

    let mut out = String::with_capacity(4096);

    writeln!(out, "package {DEFAULT_PACKAGE};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "import java.util.Map;").unwrap();
    writeln!(out, "import java.util.Set;").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "/**").unwrap();
    writeln!(
        out,
        " * State machine for {{@link {class_name}}}."
    )
    .unwrap();
    writeln!(out, " * Generated from {}.schema -- DO NOT EDIT.", schema.name).unwrap();
    writeln!(out, " */").unwrap();
    writeln!(out, "public final class {sm_name} {{").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "    private {sm_name}() {{}}").unwrap();
    writeln!(out).unwrap();

    // State constants
    writeln!(out, "    // =========================================================================").unwrap();
    writeln!(out, "    // State Constants").unwrap();
    writeln!(out, "    // =========================================================================").unwrap();
    writeln!(out).unwrap();

    for state in &sm.states {
        let const_name = to_constant_case(&state.name);
        writeln!(
            out,
            "    public static final String STATE_{const_name} = \"{}\";",
            state.name
        )
        .unwrap();
    }
    writeln!(out).unwrap();

    // Initial state
    let initial = sm
        .initial_state
        .as_deref()
        .or_else(|| {
            sm.states
                .iter()
                .find(|s| s.qualifier == Some(StateQualifier::Initial))
                .map(|s| s.name.as_str())
        })
        .unwrap_or("unknown");
    let initial_const = to_constant_case(initial);
    writeln!(
        out,
        "    public static final String INITIAL_STATE = STATE_{initial_const};"
    )
    .unwrap();
    writeln!(out).unwrap();

    // Terminal states set
    let terminal_states: Vec<&str> = sm
        .states
        .iter()
        .filter(|s| s.qualifier == Some(StateQualifier::Terminal))
        .map(|s| s.name.as_str())
        .collect();

    if terminal_states.is_empty() {
        writeln!(out, "    public static final Set<String> TERMINAL_STATES = Set.of();").unwrap();
    } else {
        let terminal_refs: Vec<String> = terminal_states
            .iter()
            .map(|s| format!("STATE_{}", to_constant_case(s)))
            .collect();
        writeln!(
            out,
            "    public static final Set<String> TERMINAL_STATES = Set.of({});",
            terminal_refs.join(", ")
        )
        .unwrap();
    }
    writeln!(out).unwrap();

    // All states set
    let all_state_refs: Vec<String> = sm
        .states
        .iter()
        .map(|s| format!("STATE_{}", to_constant_case(&s.name)))
        .collect();
    writeln!(
        out,
        "    public static final Set<String> ALL_STATES = Set.of({});",
        all_state_refs.join(", ")
    )
    .unwrap();
    writeln!(out).unwrap();

    // Transition map
    writeln!(out, "    // =========================================================================").unwrap();
    writeln!(out, "    // Transitions").unwrap();
    writeln!(out, "    // =========================================================================").unwrap();
    writeln!(out).unwrap();

    // Build transition entries
    let mut transition_entries: Vec<(String, Vec<String>)> = Vec::new();
    for t in &sm.transitions {
        match t {
            TransitionDecl::From { state, targets } => {
                let from_ref = format!("STATE_{}", to_constant_case(state));
                let to_refs: Vec<String> = targets
                    .iter()
                    .map(|t| format!("STATE_{}", to_constant_case(t)))
                    .collect();
                transition_entries.push((from_ref, to_refs));
            }
            TransitionDecl::Arrow { from, to, .. } => {
                let from_ref = format!("STATE_{}", to_constant_case(from));
                let to_ref = format!("STATE_{}", to_constant_case(to));
                // Merge into existing entry or create new
                if let Some(entry) = transition_entries.iter_mut().find(|(f, _)| f == &from_ref) {
                    if !entry.1.contains(&to_ref) {
                        entry.1.push(to_ref);
                    }
                } else {
                    transition_entries.push((from_ref, vec![to_ref]));
                }
            }
        }
    }

    writeln!(
        out,
        "    private static final Map<String, Set<String>> TRANSITIONS = Map.ofEntries("
    )
    .unwrap();
    for (i, (from, tos)) in transition_entries.iter().enumerate() {
        let comma = if i + 1 < transition_entries.len() {
            ","
        } else {
            ""
        };
        writeln!(
            out,
            "        Map.entry({from}, Set.of({})){comma}",
            tos.join(", ")
        )
        .unwrap();
    }
    writeln!(out, "    );").unwrap();
    writeln!(out).unwrap();

    // isValidState
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Check if a state name is valid.").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static boolean isValidState(String state) {{"
    )
    .unwrap();
    writeln!(out, "        return ALL_STATES.contains(state);").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // isTerminal
    writeln!(out, "    /**").unwrap();
    writeln!(out, "     * Check if a state is terminal (no outgoing transitions).").unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static boolean isTerminal(String state) {{"
    )
    .unwrap();
    writeln!(out, "        return TERMINAL_STATES.contains(state);").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // canTransition
    writeln!(out, "    /**").unwrap();
    writeln!(
        out,
        "     * Check if a transition from one state to another is allowed."
    )
    .unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static boolean canTransition(String from, String to) {{"
    )
    .unwrap();
    writeln!(
        out,
        "        Set<String> targets = TRANSITIONS.get(from);"
    )
    .unwrap();
    writeln!(
        out,
        "        return targets != null && targets.contains(to);"
    )
    .unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // getValidTransitions
    writeln!(out, "    /**").unwrap();
    writeln!(
        out,
        "     * Get all valid target states from the given state."
    )
    .unwrap();
    writeln!(out, "     */").unwrap();
    writeln!(
        out,
        "    public static Set<String> getValidTransitions(String from) {{"
    )
    .unwrap();
    writeln!(
        out,
        "        return TRANSITIONS.getOrDefault(from, Set.of());"
    )
    .unwrap();
    writeln!(out, "    }}").unwrap();

    // Close class
    writeln!(out, "}}").unwrap();

    Some((filename, out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexflow_parser::ast::schema::*;

    fn make_sm_schema() -> SchemaDefinition {
        SchemaDefinition {
            imports: Vec::new(),
            name: "order_workflow".to_string(),
            patterns: vec![MutationPattern::StateMachine],
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: Vec::new(),
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: Vec::new(),
            immutable: None,
            state_machine: Some(StateMachineBlock {
                for_entity: Some("order_id".to_string()),
                states: vec![
                    StateDecl {
                        name: "pending".to_string(),
                        qualifier: Some(StateQualifier::Initial),
                    },
                    StateDecl {
                        name: "processing".to_string(),
                        qualifier: None,
                    },
                    StateDecl {
                        name: "shipped".to_string(),
                        qualifier: None,
                    },
                    StateDecl {
                        name: "delivered".to_string(),
                        qualifier: Some(StateQualifier::Terminal),
                    },
                ],
                initial_state: None,
                transitions: vec![
                    TransitionDecl::Arrow {
                        from: "pending".to_string(),
                        to: "processing".to_string(),
                        trigger: Some("start_processing".to_string()),
                    },
                    TransitionDecl::Arrow {
                        from: "processing".to_string(),
                        to: "shipped".to_string(),
                        trigger: Some("mark_shipped".to_string()),
                    },
                    TransitionDecl::Arrow {
                        from: "shipped".to_string(),
                        to: "delivered".to_string(),
                        trigger: Some("mark_delivered".to_string()),
                    },
                ],
                on_transition: Vec::new(),
            }),
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        }
    }

    #[test]
    fn test_has_state_machine() {
        assert!(has_state_machine(&make_sm_schema()));

        let no_sm = SchemaDefinition {
            state_machine: None,
            ..make_sm_schema()
        };
        assert!(!has_state_machine(&no_sm));
    }

    #[test]
    fn test_sm_filename() {
        let (filename, _) = generate_state_machine(&make_sm_schema()).unwrap();
        assert_eq!(filename, "OrderWorkflowStateMachine.java");
    }

    #[test]
    fn test_sm_state_constants() {
        let (_, content) = generate_state_machine(&make_sm_schema()).unwrap();

        assert!(content.contains("STATE_PENDING = \"pending\""));
        assert!(content.contains("STATE_PROCESSING = \"processing\""));
        assert!(content.contains("STATE_SHIPPED = \"shipped\""));
        assert!(content.contains("STATE_DELIVERED = \"delivered\""));
    }

    #[test]
    fn test_sm_initial_state() {
        let (_, content) = generate_state_machine(&make_sm_schema()).unwrap();

        assert!(content.contains("INITIAL_STATE = STATE_PENDING"));
    }

    #[test]
    fn test_sm_terminal_states() {
        let (_, content) = generate_state_machine(&make_sm_schema()).unwrap();

        assert!(content.contains("TERMINAL_STATES = Set.of(STATE_DELIVERED)"));
    }

    #[test]
    fn test_sm_transitions() {
        let (_, content) = generate_state_machine(&make_sm_schema()).unwrap();

        assert!(content.contains("Map.entry(STATE_PENDING, Set.of(STATE_PROCESSING))"));
        assert!(content.contains("Map.entry(STATE_PROCESSING, Set.of(STATE_SHIPPED))"));
        assert!(content.contains("Map.entry(STATE_SHIPPED, Set.of(STATE_DELIVERED))"));
    }

    #[test]
    fn test_sm_methods() {
        let (_, content) = generate_state_machine(&make_sm_schema()).unwrap();

        assert!(content.contains("public static boolean isValidState(String state)"));
        assert!(content.contains("public static boolean isTerminal(String state)"));
        assert!(content.contains("public static boolean canTransition(String from, String to)"));
        assert!(content.contains("public static Set<String> getValidTransitions(String from)"));
    }

    #[test]
    fn test_no_sm_returns_none() {
        let schema = SchemaDefinition {
            imports: Vec::new(),
            name: "simple".to_string(),
            patterns: Vec::new(),
            targets: Vec::new(),
            version: None,
            compatibility: None,
            retention: None,
            identity: Vec::new(),
            streaming: None,
            serialization: None,
            fields: Vec::new(),
            nested_objects: Vec::new(),
            computed: Vec::new(),
            constraints: Vec::new(),
            immutable: None,
            state_machine: None,
            parameters: Vec::new(),
            entries: Vec::new(),
            rules: Vec::new(),
            migration: Vec::new(),
        };

        assert!(generate_state_machine(&schema).is_none());
    }

    #[test]
    fn test_from_transition_syntax() {
        let mut schema = make_sm_schema();
        schema.state_machine.as_mut().unwrap().transitions = vec![
            TransitionDecl::From {
                state: "pending".to_string(),
                targets: vec!["processing".to_string(), "cancelled".to_string()],
            },
        ];
        schema.state_machine.as_mut().unwrap().states.push(StateDecl {
            name: "cancelled".to_string(),
            qualifier: Some(StateQualifier::Terminal),
        });

        let (_, content) = generate_state_machine(&schema).unwrap();

        assert!(content.contains("Map.entry(STATE_PENDING, Set.of(STATE_PROCESSING, STATE_CANCELLED))"));
    }
}
