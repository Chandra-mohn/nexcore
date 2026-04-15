//! Proc file resolver -- finds and parses cataloged procedure files.
//!
//! Given a procedure name (from EXEC MYPROC), searches JCLLIB paths
//! and a project directory for matching .jcl files (or extensionless members).

use std::collections::HashMap;
use std::path::PathBuf;

use crate::ast::JclProc;
use crate::parser::parse_jcl;
use crate::ast::JclSource;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Resolver that finds and caches parsed procedure definitions.
#[derive(Debug)]
pub struct ProcResolver {
    /// Ordered search paths (JCLLIB ORDER= paths + project dirs).
    search_paths: Vec<PathBuf>,
    /// Extensions to try, in order.
    extensions: Vec<&'static str>,
    /// Cache of resolved procs (name -> parsed proc).
    cache: HashMap<String, Option<JclProc>>,
    /// Maximum nesting depth to prevent infinite recursion.
    max_depth: usize,
}

impl ProcResolver {
    /// Create a resolver with the given search paths.
    ///
    /// Search paths are tried in order. Each path is a directory
    /// that may contain proc files.
    pub fn new(search_paths: Vec<PathBuf>) -> Self {
        Self {
            search_paths,
            extensions: vec!["jcl", "JCL"],
            cache: HashMap::new(),
            max_depth: 15, // z/OS limit
        }
    }

    /// Add additional search paths (e.g., from JCLLIB ORDER=).
    pub fn add_search_paths(&mut self, paths: &[String]) {
        for p in paths {
            let path = PathBuf::from(p);
            if path.is_dir() && !self.search_paths.contains(&path) {
                self.search_paths.push(path);
            }
        }
    }

    /// Resolve a procedure by name.
    ///
    /// Returns the parsed `JclProc` if found, or `None` if the proc
    /// file cannot be located.
    pub fn resolve(&mut self, proc_name: &str) -> Option<&JclProc> {
        let key = proc_name.to_uppercase();

        if !self.cache.contains_key(&key) {
            let result = self.find_and_parse(&key);
            self.cache.insert(key.clone(), result);
        }

        self.cache.get(&key).and_then(|v| v.as_ref())
    }

    /// Find a proc file on disk and parse it.
    fn find_and_parse(&self, proc_name: &str) -> Option<JclProc> {
        let path = self.find_file(proc_name)?;

        let source_text = std::fs::read_to_string(&path).ok()?;
        let parsed = parse_jcl(&source_text).ok()?;

        match parsed {
            JclSource::Proc(proc) => Some(proc),
            JclSource::Job(job) => {
                // Some sites store procs as full JCL with a JOB card.
                // Extract the steps as a pseudo-proc.
                Some(JclProc {
                    name: Some(proc_name.to_string()),
                    symbols: HashMap::new(),
                    body: job.body,
                })
            }
        }
    }

    /// Search for a proc file across all search paths.
    fn find_file(&self, proc_name: &str) -> Option<PathBuf> {
        let names_to_try = [
            proc_name.to_uppercase(),
            proc_name.to_lowercase(),
            proc_name.to_string(),
        ];

        for dir in &self.search_paths {
            for name in &names_to_try {
                // Try with each extension
                for ext in &self.extensions {
                    let candidate = dir.join(format!("{name}.{ext}"));
                    if candidate.is_file() {
                        return Some(candidate);
                    }
                }

                // Try without extension (common on mainframe-migrated repos)
                let candidate = dir.join(name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }

        None
    }

    /// Returns the maximum nesting depth for recursive resolution.
    pub fn max_depth(&self) -> usize {
        self.max_depth
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn resolve_proc_from_directory() {
        let dir = tempfile::tempdir().unwrap();
        let proc_content = r#"
//MYPROC  PROC RC=0
//STEP1   EXEC PGM=MYPROG,PARM=&RC
//INPUT   DD DSN=MY.INPUT,DISP=SHR
//OUTPUT  DD SYSOUT=*
// PEND
"#;
        fs::write(dir.path().join("MYPROC.jcl"), proc_content).unwrap();

        let mut resolver = ProcResolver::new(vec![dir.path().to_path_buf()]);
        let proc = resolver.resolve("MYPROC");

        assert!(proc.is_some(), "should resolve MYPROC");
        let proc = proc.unwrap();
        assert_eq!(proc.name.as_deref(), Some("MYPROC"));
        assert!(!proc.body.is_empty());
    }

    #[test]
    fn resolve_lowercase_file() {
        let dir = tempfile::tempdir().unwrap();
        let proc_content = r#"
//SUBPROC PROC
//S1      EXEC PGM=UTIL
//SYSOUT  DD SYSOUT=*
// PEND
"#;
        fs::write(dir.path().join("subproc.jcl"), proc_content).unwrap();

        let mut resolver = ProcResolver::new(vec![dir.path().to_path_buf()]);
        let proc = resolver.resolve("SUBPROC");

        assert!(proc.is_some(), "should resolve lowercase file");
    }

    #[test]
    fn resolve_missing_proc() {
        let dir = tempfile::tempdir().unwrap();
        let mut resolver = ProcResolver::new(vec![dir.path().to_path_buf()]);
        assert!(resolver.resolve("NONEXIST").is_none());
    }

    #[test]
    fn caches_results() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("CACHED.jcl"), r#"
//CACHED  PROC
//S1      EXEC PGM=TEST
// PEND
"#).unwrap();

        let mut resolver = ProcResolver::new(vec![dir.path().to_path_buf()]);

        // First resolve
        assert!(resolver.resolve("CACHED").is_some());
        // Second resolve hits cache
        assert!(resolver.resolve("CACHED").is_some());
        assert_eq!(resolver.cache.len(), 1);
    }
}
