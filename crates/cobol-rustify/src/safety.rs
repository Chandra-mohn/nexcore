//! Safety gate framework for Tier 2+ rules.
//!
//! Safety gates identify COBOL constructs that prevent automatic type
//! promotion. When a gate fires, the field is flagged for review instead
//! of being auto-transformed.

use crate::hints::{FieldHint, FileHints};

/// A safety gate that blocks automatic promotion.
#[derive(Debug, Clone)]
pub struct SafetyGate {
    /// Field name that triggered the gate.
    pub field_name: String,
    /// Why the gate fired.
    pub reason: SafetyReason,
    /// Human-readable recommendation.
    pub recommendation: String,
}

/// Reason a safety gate blocks promotion.
#[derive(Debug, Clone)]
pub enum SafetyReason {
    /// Field participates in REDEFINES (memory overlay).
    Redefines { redefined_by: Vec<String> },
    /// Field is the target of a REDEFINES.
    RedefinesTarget { redefines: String },
    /// Field is passed BY REFERENCE in a CALL statement.
    CallByReference,
    /// Field is a target of MOVE CORRESPONDING.
    MoveCorresponding,
    /// Field is used in file I/O operations.
    FileIoField,
}

impl std::fmt::Display for SafetyReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Redefines { redefined_by } => {
                write!(f, "REDEFINES: redefined by {}", redefined_by.join(", "))
            }
            Self::RedefinesTarget { redefines } => {
                write!(f, "REDEFINES: redefines {redefines}")
            }
            Self::CallByReference => write!(f, "CALL BY REFERENCE"),
            Self::MoveCorresponding => write!(f, "MOVE CORRESPONDING target"),
            Self::FileIoField => write!(f, "File I/O record field"),
        }
    }
}

/// Check all safety gates for a field. Returns empty vec if field is safe to promote.
pub fn check_safety_gates(
    field_name: &str,
    hint: &FieldHint,
    file_hints: &FileHints,
) -> Vec<SafetyGate> {
    let mut gates = Vec::new();

    // Gate: field is redefined by other fields
    if !hint.redefined_by.is_empty() {
        gates.push(SafetyGate {
            field_name: field_name.to_string(),
            reason: SafetyReason::Redefines {
                redefined_by: hint.redefined_by.clone(),
            },
            recommendation: format!(
                "keep as COBOL type until REDEFINES by {} eliminated",
                hint.redefined_by.join(", ")
            ),
        });
    }

    // Gate: field redefines another field
    if let Some(ref target) = hint.redefines {
        gates.push(SafetyGate {
            field_name: field_name.to_string(),
            reason: SafetyReason::RedefinesTarget {
                redefines: target.clone(),
            },
            recommendation: format!("keep as COBOL type -- redefines {target}"),
        });
    }

    // Gate: field is passed BY REFERENCE in CALL
    if hint.call_by_ref {
        gates.push(SafetyGate {
            field_name: field_name.to_string(),
            reason: SafetyReason::CallByReference,
            recommendation: "keep as COBOL type -- external program expects COBOL layout".to_string(),
        });
    }

    // Gate: field is MOVE CORRESPONDING target
    if hint.move_corr_target {
        gates.push(SafetyGate {
            field_name: field_name.to_string(),
            reason: SafetyReason::MoveCorresponding,
            recommendation: "keep as COBOL type -- MOVE CORR relies on field structure".to_string(),
        });
    }

    // Gate: field is used in file I/O
    if file_hints.file_io_fields.contains(&field_name.to_string()) {
        gates.push(SafetyGate {
            field_name: field_name.to_string(),
            reason: SafetyReason::FileIoField,
            recommendation: "keep as COBOL type -- used in file I/O record operations".to_string(),
        });
    }

    gates
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::{FieldHint, FileHints};
    use std::collections::HashMap;

    fn default_hint() -> FieldHint {
        FieldHint {
            pic: "X(10)".to_string(),
            usage: "DISPLAY".to_string(),
            level: 5,
            redefines: None,
            redefined_by: vec![],
            call_by_ref: false,
            move_corr_target: false,
            read_count: 3,
            write_count: 1,
            paragraph_scope: vec![],
        }
    }

    fn default_file_hints() -> FileHints {
        FileHints {
            cobol_source: "TEST.CBL".to_string(),
            fields: HashMap::new(),
            paragraphs: HashMap::new(),
            level_88_groups: HashMap::new(),
            call_targets: vec![],
            file_io_fields: vec![],
        }
    }

    #[test]
    fn safe_field_no_gates() {
        let hint = default_hint();
        let fh = default_file_hints();
        let gates = check_safety_gates("ws_name", &hint, &fh);
        assert!(gates.is_empty());
    }

    #[test]
    fn redefines_blocks() {
        let mut hint = default_hint();
        hint.redefined_by = vec!["ws_parts".to_string()];
        let fh = default_file_hints();
        let gates = check_safety_gates("ws_full", &hint, &fh);
        assert_eq!(gates.len(), 1);
        assert!(matches!(gates[0].reason, SafetyReason::Redefines { .. }));
    }

    #[test]
    fn redefines_target_blocks() {
        let mut hint = default_hint();
        hint.redefines = Some("ws_original".to_string());
        let fh = default_file_hints();
        let gates = check_safety_gates("ws_overlay", &hint, &fh);
        assert_eq!(gates.len(), 1);
        assert!(matches!(gates[0].reason, SafetyReason::RedefinesTarget { .. }));
    }

    #[test]
    fn call_by_ref_blocks() {
        let mut hint = default_hint();
        hint.call_by_ref = true;
        let fh = default_file_hints();
        let gates = check_safety_gates("ws_param", &hint, &fh);
        assert_eq!(gates.len(), 1);
        assert!(matches!(gates[0].reason, SafetyReason::CallByReference));
    }

    #[test]
    fn move_corr_blocks() {
        let mut hint = default_hint();
        hint.move_corr_target = true;
        let fh = default_file_hints();
        let gates = check_safety_gates("ws_target", &hint, &fh);
        assert_eq!(gates.len(), 1);
        assert!(matches!(gates[0].reason, SafetyReason::MoveCorresponding));
    }

    #[test]
    fn file_io_blocks() {
        let hint = default_hint();
        let mut fh = default_file_hints();
        fh.file_io_fields = vec!["ws_record".to_string()];
        let gates = check_safety_gates("ws_record", &hint, &fh);
        assert_eq!(gates.len(), 1);
        assert!(matches!(gates[0].reason, SafetyReason::FileIoField));
    }

    #[test]
    fn multiple_gates() {
        let mut hint = default_hint();
        hint.call_by_ref = true;
        hint.redefined_by = vec!["ws_other".to_string()];
        let fh = default_file_hints();
        let gates = check_safety_gates("ws_multi", &hint, &fh);
        assert_eq!(gates.len(), 2);
    }
}
