//! Schema pattern detection from file organization and access mode.
//!
//! Determines the SchemaDSL mutation pattern based on COBOL file
//! characteristics rather than field-name heuristics.

use cobol_transpiler::ast::{AccessMode, FileDescription, FileOrganization};

use crate::dsl::dsl_ast::MutationPattern;

/// Detect the schema mutation pattern from a file description.
pub fn detect_pattern(fd: &FileDescription) -> MutationPattern {
    match fd.organization {
        // KSDS: keyed, updated in place, random access -> master_data
        FileOrganization::Indexed => MutationPattern::MasterData,

        // RRDS: slot-based lookup table -> reference_data
        FileOrganization::Relative => MutationPattern::ReferenceData,

        // Sequential/LineSequential: depends on access direction
        FileOrganization::Sequential | FileOrganization::LineSequential => {
            match fd.access_mode {
                // Sequential-only access: likely batch output or log
                AccessMode::Sequential => {
                    // Check naming for financial patterns
                    let name_lower = fd.file_name.to_lowercase();
                    if is_ledger_name(&name_lower) {
                        MutationPattern::EventLog
                    } else {
                        // Default sequential: could be input master or output report.
                        // Conservative default: master_data (most common for input).
                        MutationPattern::MasterData
                    }
                }
                // Random/Dynamic on sequential = unusual, treat as master
                AccessMode::Random | AccessMode::Dynamic => MutationPattern::MasterData,
            }
        }
    }
}

/// Detect pattern for WORKING-STORAGE or LINKAGE entries (no file association).
pub fn detect_pattern_for_ws() -> MutationPattern {
    // Working storage groups are typically master data structures
    MutationPattern::MasterData
}

/// Detect pattern for LINKAGE SECTION / COMMAREA.
pub fn detect_pattern_for_linkage() -> MutationPattern {
    // COMMAREA is a request/response interface -- closest is command pattern.
    // SchemaDSL doesn't have command/response yet, use master_data as fallback.
    MutationPattern::MasterData
}

/// Detect pattern for EXEC SQL tables.
pub fn detect_pattern_for_sql() -> MutationPattern {
    // DB2 tables are typically master_data. The actual pattern should come
    // from the DDL (temporal tables, audit tables, etc.) but we don't have
    // DDL access yet. Conservative default.
    MutationPattern::MasterData
}

/// Check if a file name suggests a ledger/log/journal (-> event_log).
fn is_ledger_name(name: &str) -> bool {
    name.contains("ledger")
        || name.contains("journal")
        || name.contains("log")
        || name.contains("audit")
        || name.contains("trans")
        || name.contains("history")
}

#[cfg(test)]
mod tests {
    use super::*;
    use cobol_transpiler::ast::FileDescriptorType;

    fn make_fd(org: FileOrganization, access: AccessMode, name: &str) -> FileDescription {
        FileDescription {
            descriptor_type: FileDescriptorType::Fd,
            file_name: name.to_string(),
            assign_to: None,
            organization: org,
            access_mode: access,
            record_key: None,
            alternate_keys: vec![],
            relative_key: None,
            file_status: None,
            records: vec![],
            record_names: vec![],
            recording_mode: None,
            record_size: None,
        }
    }

    #[test]
    fn indexed_is_master_data() {
        let fd = make_fd(FileOrganization::Indexed, AccessMode::Dynamic, "ACCT-FILE");
        assert!(matches!(detect_pattern(&fd), MutationPattern::MasterData));
    }

    #[test]
    fn relative_is_reference_data() {
        let fd = make_fd(FileOrganization::Relative, AccessMode::Random, "LOOKUP-TBL");
        assert!(matches!(detect_pattern(&fd), MutationPattern::ReferenceData));
    }

    #[test]
    fn sequential_ledger_is_event_log() {
        let fd = make_fd(FileOrganization::Sequential, AccessMode::Sequential, "TRANS-LEDGER");
        assert!(matches!(detect_pattern(&fd), MutationPattern::EventLog));
    }

    #[test]
    fn sequential_default_is_master() {
        let fd = make_fd(FileOrganization::Sequential, AccessMode::Sequential, "INPUT-FILE");
        assert!(matches!(detect_pattern(&fd), MutationPattern::MasterData));
    }
}
