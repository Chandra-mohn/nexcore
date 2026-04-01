// NexCore -- Nexflow Codegen: Java Runtime Library Generator (L0)
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Generates the shared Java runtime library for all Nexflow-generated code.
//!
//! **L0 Generator** -- the foundation that L2, L3, L4 all depend on.
//!
//! Produces:
//! - `NexflowRuntime.java` -- static utility class (time, math, string, collection, PII, etc.)
//! - `LookupService.java` -- generic lookup interface
//! - `LookupServiceFactory.java` -- factory with pluggable registration and no-op default
//! - `BusinessCalendar.java` -- pluggable calendar interface
//! - `DefaultBusinessCalendar.java` -- weekday-only implementation
//!
//! Key improvements over the Python-generated version:
//! - All methods null-safe (null in -> null out, no NPE)
//! - BigDecimal precision configurable via DEFAULT_SCALE constant
//! - BusinessCalendar is a pluggable interface (not hardcoded weekend logic)
//! - LookupServiceFactory supports runtime registration of custom backends
//! - Thread-safe implementations (ConcurrentHashMap, volatile)
//! - Zero external dependencies (pure java.* imports)
//! - Proper Javadoc on every public method

pub mod calendar;
pub mod lookup;
pub mod nexflow_runtime;

use std::collections::HashMap;

use crate::GeneratedProject;

/// Configuration for runtime library generation.
#[derive(Debug, Clone)]
pub struct RuntimeGenConfig {
    /// Java package name (default: `com.nexflow.runtime`).
    pub package: String,
    /// Output directory prefix (default: `src/main/java/com/nexflow/runtime`).
    pub java_dir: String,
}

impl Default for RuntimeGenConfig {
    fn default() -> Self {
        Self {
            package: "com.nexflow.runtime".to_string(),
            java_dir: "src/main/java/com/nexflow/runtime".to_string(),
        }
    }
}

/// Generate the complete Java runtime library.
///
/// Returns a `GeneratedProject` with all runtime files.
pub fn generate_java_runtime(config: &RuntimeGenConfig) -> Result<GeneratedProject, String> {
    let mut files = HashMap::new();

    // NexflowRuntime.java
    let (filename, content) = nexflow_runtime::generate_nexflow_runtime();
    files.insert(format!("{}/{filename}", config.java_dir), content);

    // LookupService.java
    let (filename, content) = lookup::generate_lookup_service();
    files.insert(format!("{}/{filename}", config.java_dir), content);

    // LookupServiceFactory.java
    let (filename, content) = lookup::generate_lookup_service_factory();
    files.insert(format!("{}/{filename}", config.java_dir), content);

    // BusinessCalendar.java
    let (filename, content) = calendar::generate_business_calendar();
    files.insert(format!("{}/{filename}", config.java_dir), content);

    // DefaultBusinessCalendar.java
    let (filename, content) = calendar::generate_default_business_calendar();
    files.insert(format!("{}/{filename}", config.java_dir), content);

    Ok(GeneratedProject { files })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generates_all_runtime_files() {
        let config = RuntimeGenConfig::default();
        let project = generate_java_runtime(&config).unwrap();
        let dir = &config.java_dir;

        assert_eq!(project.files.len(), 5);

        assert!(project.files.contains_key(&format!("{dir}/NexflowRuntime.java")));
        assert!(project.files.contains_key(&format!("{dir}/LookupService.java")));
        assert!(project.files.contains_key(&format!("{dir}/LookupServiceFactory.java")));
        assert!(project.files.contains_key(&format!("{dir}/BusinessCalendar.java")));
        assert!(project.files.contains_key(&format!("{dir}/DefaultBusinessCalendar.java")));
    }

    #[test]
    fn test_runtime_is_self_consistent() {
        let config = RuntimeGenConfig::default();
        let project = generate_java_runtime(&config).unwrap();
        let dir = &config.java_dir;

        // NexflowRuntime references BusinessCalendar
        let runtime = &project.files[&format!("{dir}/NexflowRuntime.java")];
        assert!(runtime.contains("BusinessCalendar"));
        assert!(runtime.contains("DefaultBusinessCalendar"));
        assert!(runtime.contains("LookupServiceFactory"));

        // Factory references LookupService
        let factory = &project.files[&format!("{dir}/LookupServiceFactory.java")];
        assert!(factory.contains("LookupService"));

        // Default calendar implements BusinessCalendar
        let default_cal = &project.files[&format!("{dir}/DefaultBusinessCalendar.java")];
        assert!(default_cal.contains("implements BusinessCalendar"));
    }

    #[test]
    fn test_all_files_have_package() {
        let config = RuntimeGenConfig::default();
        let project = generate_java_runtime(&config).unwrap();

        for (_, content) in &project.files {
            assert!(content.contains("package com.nexflow.runtime;"));
        }
    }

    #[test]
    fn test_all_files_have_do_not_edit() {
        let config = RuntimeGenConfig::default();
        let project = generate_java_runtime(&config).unwrap();

        for (_, content) in &project.files {
            assert!(content.contains("DO NOT EDIT"));
        }
    }
}
