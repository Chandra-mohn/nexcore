// NexCore -- Nexflow Compiler: Import Resolution
// Copyright (c) 2024-2026 Chandra Mohn. All Rights Reserved.

//! Resolves import paths to files, parses them, and builds a complete project.
//!
//! Given a root file (typically a `.service` or `.api` file), the resolver:
//! 1. Parses the root file
//! 2. Extracts import paths
//! 3. Resolves each import relative to the importing file's directory
//! 4. Recursively parses and resolves imports (with cycle detection)
//! 5. Returns a `Project` containing all loaded ASTs

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use nexflow_parser::ParseError;
use nexflow_parser::ast::api::ApiDefinition;
use nexflow_parser::ast::common::ImportPath;
use nexflow_parser::ast::nexquery::NexQueryScript;
use nexflow_parser::ast::proc::ProcProgram;
use nexflow_parser::ast::rules::RulesProgram;
use nexflow_parser::ast::schema::{SchemaDefinition, SchemaProgram};
use nexflow_parser::ast::service::ServiceDefinition;
use nexflow_parser::ast::transform::TransformProgram;

use crate::diagnostics::{DiagnosticSource, ValidationResult};

/// A resolved project containing all parsed ASTs from a file tree.
#[derive(Debug, Clone)]
pub struct Project {
    /// All loaded API definitions.
    pub apis: Vec<ApiDefinition>,
    /// All loaded service definitions.
    pub services: Vec<ServiceDefinition>,
    /// All loaded schema definitions (flattened from schema programs).
    pub schemas: Vec<SchemaDefinition>,
    /// All loaded transform programs.
    pub transforms: Vec<TransformProgram>,
    /// All loaded rules programs.
    pub rules: Vec<RulesProgram>,
    /// All loaded proc programs.
    pub procs: Vec<ProcProgram>,
    /// All loaded NexQuery scripts.
    pub queries: Vec<NexQueryScript>,
    /// Files that were referenced but could not be found or parsed.
    pub unresolved: Vec<UnresolvedImport>,
    /// All file paths that were successfully loaded.
    pub loaded_files: Vec<PathBuf>,
}

/// An import that could not be resolved.
#[derive(Debug, Clone)]
pub struct UnresolvedImport {
    /// The raw import path from the source file.
    pub raw_path: String,
    /// The resolved absolute path (if resolution succeeded but file is missing).
    pub resolved_path: Option<PathBuf>,
    /// The file that contains this import.
    pub source_file: PathBuf,
    /// Why it could not be resolved.
    pub reason: String,
}

/// Load a project starting from a root file.
///
/// The root file is parsed, its imports are resolved relative to its directory,
/// and all transitively imported files are loaded. Files with extensions we
/// don't have parsers for (.rules, .xform, .proc) are recorded as unresolved
/// with an informational reason.
///
/// # Errors
///
/// Returns `ParseError::Ast` if the root path cannot be canonicalized (e.g., does not exist).
pub fn load_project(root_path: &Path) -> Result<(Project, ValidationResult), ParseError> {
    let root_path = root_path
        .canonicalize()
        .map_err(|e| ParseError::ast("Resolver", format!("Cannot resolve root path '{}': {e}", root_path.display())))?;

    let mut project = Project {
        apis: Vec::new(),
        services: Vec::new(),
        schemas: Vec::new(),
        transforms: Vec::new(),
        rules: Vec::new(),
        procs: Vec::new(),
        queries: Vec::new(),
        unresolved: Vec::new(),
        loaded_files: Vec::new(),
    };
    let mut diagnostics = ValidationResult::new();
    let mut visited: HashSet<PathBuf> = HashSet::new();
    let mut queue: Vec<(PathBuf, PathBuf)> = vec![(root_path.clone(), root_path.clone())];

    while let Some((file_path, source_file)) = queue.pop() {
        if visited.contains(&file_path) {
            continue;
        }
        visited.insert(file_path.clone());

        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match ext {
            "api" => {
                match load_and_parse_api(&file_path) {
                    Ok((api, imports)) => {
                        let dir = file_path.parent().unwrap_or(Path::new("."));
                        for imp in &imports {
                            let resolved = resolve_import_path(dir, &imp.raw);
                            queue.push((resolved, file_path.clone()));
                        }
                        project.apis.push(api);
                        project.loaded_files.push(file_path);
                    }
                    Err(e) => {
                        diagnostics.error(
                            DiagnosticSource::Api,
                            format!("Failed to parse '{}': {e}", file_path.display()),
                            None,
                        );
                        project.unresolved.push(UnresolvedImport {
                            raw_path: file_path.display().to_string(),
                            resolved_path: Some(file_path.clone()),
                            source_file,
                            reason: e.to_string(),
                        });
                    }
                }
            }
            "service" => {
                match load_and_parse_service(&file_path) {
                    Ok((service, imports)) => {
                        let dir = file_path.parent().unwrap_or(Path::new("."));
                        for imp in &imports {
                            let resolved = resolve_import_path(dir, &imp.raw);
                            queue.push((resolved, file_path.clone()));
                        }
                        project.services.push(service);
                        project.loaded_files.push(file_path);
                    }
                    Err(e) => {
                        diagnostics.error(
                            DiagnosticSource::Service,
                            format!("Failed to parse '{}': {e}", file_path.display()),
                            None,
                        );
                        project.unresolved.push(UnresolvedImport {
                            raw_path: file_path.display().to_string(),
                            resolved_path: Some(file_path.clone()),
                            source_file,
                            reason: e.to_string(),
                        });
                    }
                }
            }
            "schema" => {
                match load_and_parse_schema(&file_path) {
                    Ok((program, imports)) => {
                        let dir = file_path.parent().unwrap_or(Path::new("."));
                        for imp in &imports {
                            let resolved = resolve_import_path(dir, &imp.raw);
                            queue.push((resolved, file_path.clone()));
                        }
                        project.schemas.extend(program.schemas);
                        project.loaded_files.push(file_path);
                    }
                    Err(e) => {
                        diagnostics.error(
                            DiagnosticSource::Schema,
                            format!("Failed to parse '{}': {e}", file_path.display()),
                            None,
                        );
                        project.unresolved.push(UnresolvedImport {
                            raw_path: file_path.display().to_string(),
                            resolved_path: Some(file_path.clone()),
                            source_file,
                            reason: e.to_string(),
                        });
                    }
                }
            }
            "xform" | "transform" => {
                match load_and_parse_transform(&file_path) {
                    Ok((program, imports)) => {
                        let dir = file_path.parent().unwrap_or(Path::new("."));
                        for imp in &imports {
                            let resolved = resolve_import_path(dir, &imp.raw);
                            queue.push((resolved, file_path.clone()));
                        }
                        project.transforms.push(program);
                        project.loaded_files.push(file_path);
                    }
                    Err(e) => {
                        project.unresolved.push(UnresolvedImport {
                            raw_path: file_path.display().to_string(),
                            resolved_path: Some(file_path.clone()),
                            source_file,
                            reason: e.to_string(),
                        });
                    }
                }
            }
            "rules" => {
                match load_and_parse_rules(&file_path) {
                    Ok((program, imports)) => {
                        let dir = file_path.parent().unwrap_or(Path::new("."));
                        for imp in &imports {
                            let resolved = resolve_import_path(dir, &imp.raw);
                            queue.push((resolved, file_path.clone()));
                        }
                        project.rules.push(program);
                        project.loaded_files.push(file_path);
                    }
                    Err(e) => {
                        project.unresolved.push(UnresolvedImport {
                            raw_path: file_path.display().to_string(),
                            resolved_path: Some(file_path.clone()),
                            source_file,
                            reason: e.to_string(),
                        });
                    }
                }
            }
            "proc" => {
                match load_and_parse_proc(&file_path) {
                    Ok((program, imports)) => {
                        let dir = file_path.parent().unwrap_or(Path::new("."));
                        for imp in &imports {
                            let resolved = resolve_import_path(dir, &imp.raw);
                            queue.push((resolved, file_path.clone()));
                        }
                        project.procs.push(program);
                        project.loaded_files.push(file_path);
                    }
                    Err(e) => {
                        project.unresolved.push(UnresolvedImport {
                            raw_path: file_path.display().to_string(),
                            resolved_path: Some(file_path.clone()),
                            source_file,
                            reason: e.to_string(),
                        });
                    }
                }
            }
            "nxq" => {
                match load_and_parse_nexquery(&file_path) {
                    Ok(script) => {
                        project.queries.push(script);
                        project.loaded_files.push(file_path);
                    }
                    Err(e) => {
                        project.unresolved.push(UnresolvedImport {
                            raw_path: file_path.display().to_string(),
                            resolved_path: Some(file_path.clone()),
                            source_file,
                            reason: e.to_string(),
                        });
                    }
                }
            }
            "flow" => {
                project.unresolved.push(UnresolvedImport {
                    raw_path: file_path.display().to_string(),
                    resolved_path: Some(file_path.clone()),
                    source_file,
                    reason: format!("Parser for .{ext} files not yet implemented"),
                });
            }
            _ => {
                project.unresolved.push(UnresolvedImport {
                    raw_path: file_path.display().to_string(),
                    resolved_path: Some(file_path.clone()),
                    source_file,
                    reason: format!("Unknown file extension: .{ext}"),
                });
            }
        }
    }

    Ok((project, diagnostics))
}

/// Resolve an import path relative to a directory.
fn resolve_import_path(base_dir: &Path, raw_import: &str) -> PathBuf {
    let cleaned = raw_import
        .trim_start_matches("./")
        .trim_start_matches("../");

    if raw_import.starts_with("../") {
        base_dir.parent().unwrap_or(base_dir).join(cleaned)
    } else {
        base_dir.join(cleaned)
    }
}

fn load_and_parse_api(path: &Path) -> Result<(ApiDefinition, Vec<ImportPath>), ParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ParseError::ast("ApiDSL", format!("failed to read {}: {e}", path.display())))?;
    let api = nexflow_parser::parse_api(&source)?;
    let imports = api.imports.clone();
    Ok((api, imports))
}

fn load_and_parse_service(path: &Path) -> Result<(ServiceDefinition, Vec<ImportPath>), ParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ParseError::ast("ServiceDSL", format!("failed to read {}: {e}", path.display())))?;
    let service = nexflow_parser::parse_service(&source)?;
    let imports = service.imports.clone();
    Ok((service, imports))
}

fn load_and_parse_nexquery(path: &Path) -> Result<NexQueryScript, ParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ParseError::ast("NexQueryDSL", format!("failed to read {}: {e}", path.display())))?;
    nexflow_parser::parse_nexquery(&source)
}

fn load_and_parse_transform(path: &Path) -> Result<(TransformProgram, Vec<ImportPath>), ParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ParseError::ast("TransformDSL", format!("failed to read {}: {e}", path.display())))?;
    let program = nexflow_parser::parse_transform(&source)?;
    let imports = program.imports.clone();
    Ok((program, imports))
}

fn load_and_parse_rules(path: &Path) -> Result<(RulesProgram, Vec<ImportPath>), ParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ParseError::ast("RulesDSL", format!("failed to read {}: {e}", path.display())))?;
    let program = nexflow_parser::parse_rules(&source)?;
    let imports = program.imports.clone();
    Ok((program, imports))
}

fn load_and_parse_proc(path: &Path) -> Result<(ProcProgram, Vec<ImportPath>), ParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ParseError::ast("ProcDSL", format!("failed to read {}: {e}", path.display())))?;
    let program = nexflow_parser::parse_proc(&source)?;
    let imports = program.imports.clone();
    Ok((program, imports))
}

fn load_and_parse_schema(path: &Path) -> Result<(SchemaProgram, Vec<ImportPath>), ParseError> {
    let source = std::fs::read_to_string(path)
        .map_err(|e| ParseError::ast("SchemaDSL", format!("failed to read {}: {e}", path.display())))?;
    let program = nexflow_parser::parse_schema(&source)?;
    let imports = program.imports.clone();
    Ok((program, imports))
}

/// Load individual files by path (no import resolution).
/// Useful for CLI when user specifies multiple files explicitly.
///
/// # Errors
///
/// This function currently always returns `Ok`. Parse failures for individual
/// files are recorded as diagnostics rather than propagated as errors.
pub fn load_files(paths: &[PathBuf]) -> Result<(Project, ValidationResult), ParseError> {
    let mut project = Project {
        apis: Vec::new(),
        services: Vec::new(),
        schemas: Vec::new(),
        transforms: Vec::new(),
        rules: Vec::new(),
        procs: Vec::new(),
        queries: Vec::new(),
        unresolved: Vec::new(),
        loaded_files: Vec::new(),
    };
    let mut diagnostics = ValidationResult::new();

    for path in paths {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        match ext {
            "api" => match load_and_parse_api(path) {
                Ok((api, _)) => {
                    project.apis.push(api);
                    project.loaded_files.push(path.clone());
                }
                Err(e) => {
                    diagnostics.error(
                        DiagnosticSource::Api,
                        format!("Failed to parse '{}': {e}", path.display()),
                        None,
                    );
                }
            },
            "service" => match load_and_parse_service(path) {
                Ok((service, _)) => {
                    project.services.push(service);
                    project.loaded_files.push(path.clone());
                }
                Err(e) => {
                    diagnostics.error(
                        DiagnosticSource::Service,
                        format!("Failed to parse '{}': {e}", path.display()),
                        None,
                    );
                }
            },
            "schema" => match load_and_parse_schema(path) {
                Ok((program, _)) => {
                    project.schemas.extend(program.schemas);
                    project.loaded_files.push(path.clone());
                }
                Err(e) => {
                    diagnostics.error(
                        DiagnosticSource::Schema,
                        format!("Failed to parse '{}': {e}", path.display()),
                        None,
                    );
                }
            },
            "xform" | "transform" => match load_and_parse_transform(path) {
                Ok((program, _)) => {
                    project.transforms.push(program);
                    project.loaded_files.push(path.clone());
                }
                Err(e) => {
                    diagnostics.warning(
                        DiagnosticSource::CrossGrammar,
                        format!("Failed to parse transform '{}': {e}", path.display()),
                        None,
                    );
                }
            },
            "rules" => match load_and_parse_rules(path) {
                Ok((program, _)) => {
                    project.rules.push(program);
                    project.loaded_files.push(path.clone());
                }
                Err(e) => {
                    diagnostics.warning(
                        DiagnosticSource::CrossGrammar,
                        format!("Failed to parse rules '{}': {e}", path.display()),
                        None,
                    );
                }
            },
            "proc" => match load_and_parse_proc(path) {
                Ok((program, _)) => {
                    project.procs.push(program);
                    project.loaded_files.push(path.clone());
                }
                Err(e) => {
                    diagnostics.warning(
                        DiagnosticSource::CrossGrammar,
                        format!("Failed to parse proc '{}': {e}", path.display()),
                        None,
                    );
                }
            },
            "nxq" => match load_and_parse_nexquery(path) {
                Ok(script) => {
                    project.queries.push(script);
                    project.loaded_files.push(path.clone());
                }
                Err(e) => {
                    diagnostics.warning(
                        DiagnosticSource::CrossGrammar,
                        format!("Failed to parse query '{}': {e}", path.display()),
                        None,
                    );
                }
            },
            _ => {
                diagnostics.warning(
                    DiagnosticSource::CrossGrammar,
                    format!("Skipping unsupported file type: '{}'", path.display()),
                    None,
                );
            }
        }
    }

    Ok((project, diagnostics))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_import_path() {
        let base = Path::new("/project/src/api");

        assert_eq!(
            resolve_import_path(base, "./schemas/account.schema"),
            PathBuf::from("/project/src/api/schemas/account.schema")
        );
        assert_eq!(
            resolve_import_path(base, "../schemas/account.schema"),
            PathBuf::from("/project/src/schemas/account.schema")
        );
    }

    #[test]
    fn test_load_schema_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/nexflow/schema/account.schema");

        if path.exists() {
            let (program, _imports) = load_and_parse_schema(&path).expect("parse failed");
            assert!(!program.schemas.is_empty());
            assert_eq!(program.schemas[0].name, "account_summary");
        }
    }

    #[test]
    fn test_load_api_file() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/nexflow/api/account-api.api");

        if path.exists() {
            let (api, imports) = load_and_parse_api(&path).expect("parse failed");
            assert_eq!(api.name, "AccountAPI");
            assert_eq!(imports.len(), 4);
        }
    }

    #[test]
    fn test_load_files_explicit() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/nexflow");

        let paths: Vec<PathBuf> = vec![
            base.join("api/account-api.api"),
            base.join("schema/account.schema"),
            base.join("schema/errors.schema"),
            base.join("service/account-service.service"),
        ];

        // Only test if files exist (CI might not have examples)
        if paths.iter().all(|p| p.exists()) {
            let (project, diagnostics) = load_files(&paths).expect("load failed");
            assert_eq!(project.apis.len(), 1);
            assert_eq!(project.services.len(), 1);
            assert!(project.schemas.len() >= 10); // account.schema has 10+
            assert!(!diagnostics.has_errors());
        }
    }
}
