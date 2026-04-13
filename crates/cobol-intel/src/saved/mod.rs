use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::error::{IntelError, IntelResult};

/// Manages saved NexQuery queries as .nxq files in a directory.
#[derive(Debug)]
pub struct QueryStore {
    dir: PathBuf,
    queries: HashMap<String, String>,
}

impl QueryStore {
    /// Open or create a query store directory.
    pub fn open(dir: &Path) -> IntelResult<Self> {
        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }

        let mut store = Self {
            dir: dir.to_owned(),
            queries: HashMap::new(),
        };
        store.load_all()?;
        Ok(store)
    }

    /// Save a named query.
    pub fn save(&mut self, name: &str, query: &str) -> IntelResult<()> {
        let path = self.path_for(name);
        std::fs::write(&path, query)?;
        self.queries.insert(name.to_owned(), query.to_owned());
        Ok(())
    }

    /// Load a named query. Returns the query text.
    pub fn get(&self, name: &str) -> IntelResult<&str> {
        self.queries
            .get(name)
            .map(String::as_str)
            .ok_or_else(|| IntelError::QueryError {
                reason: format!("saved query '{name}' not found"),
            })
    }

    /// Delete a named query.
    pub fn delete(&mut self, name: &str) -> IntelResult<()> {
        let path = self.path_for(name);
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        self.queries.remove(name);
        Ok(())
    }

    /// List all saved query names.
    pub fn list(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.queries.keys().map(String::as_str).collect();
        names.sort();
        names
    }

    /// Number of saved queries.
    pub fn count(&self) -> usize {
        self.queries.len()
    }

    fn path_for(&self, name: &str) -> PathBuf {
        self.dir.join(format!("{name}.nxq"))
    }

    fn load_all(&mut self) -> IntelResult<()> {
        let entries = match std::fs::read_dir(&self.dir) {
            Ok(e) => e,
            Err(_) => return Ok(()), // directory doesn't exist yet
        };

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("nxq") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    let content = std::fs::read_to_string(&path)?;
                    self.queries.insert(stem.to_owned(), content);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = QueryStore::open(dir.path()).unwrap();

        store
            .save("clearing", "programs calling CLRG0100\nrank by risk;")
            .unwrap();

        assert_eq!(store.count(), 1);
        assert_eq!(
            store.get("clearing").unwrap(),
            "programs calling CLRG0100\nrank by risk;"
        );
    }

    #[test]
    fn list_queries() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = QueryStore::open(dir.path()).unwrap();

        store.save("alpha", "trace ALPHA;").unwrap();
        store.save("beta", "trace BETA;").unwrap();

        let names = store.list();
        assert_eq!(names, vec!["alpha", "beta"]);
    }

    #[test]
    fn delete_query() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = QueryStore::open(dir.path()).unwrap();

        store.save("temp", "trace TEMP;").unwrap();
        assert_eq!(store.count(), 1);

        store.delete("temp").unwrap();
        assert_eq!(store.count(), 0);
        assert!(store.get("temp").is_err());
    }

    #[test]
    fn persistence_across_open() {
        let dir = tempfile::tempdir().unwrap();

        {
            let mut store = QueryStore::open(dir.path()).unwrap();
            store.save("persistent", "rank programs by complexity;").unwrap();
        }

        // Reopen
        let store = QueryStore::open(dir.path()).unwrap();
        assert_eq!(store.count(), 1);
        assert_eq!(
            store.get("persistent").unwrap(),
            "rank programs by complexity;"
        );
    }

    #[test]
    fn get_nonexistent() {
        let dir = tempfile::tempdir().unwrap();
        let store = QueryStore::open(dir.path()).unwrap();
        let err = store.get("nope").unwrap_err();
        assert!(err.to_string().contains("nope"));
    }

    #[test]
    fn overwrite_existing() {
        let dir = tempfile::tempdir().unwrap();
        let mut store = QueryStore::open(dir.path()).unwrap();

        store.save("query1", "version 1;").unwrap();
        store.save("query1", "version 2;").unwrap();

        assert_eq!(store.get("query1").unwrap(), "version 2;");
        assert_eq!(store.count(), 1);
    }
}
