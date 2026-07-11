//! Snapshot testing for widget rendering output.
//!
//! Provides golden file comparison by capturing rendered output and comparing
//! it against stored snapshots. Useful for regression testing renderers.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A captured rendering snapshot.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// The snapshot name/identifier.
    pub name: String,
    /// The rendered content (line-by-line).
    pub content: Vec<String>,
    /// Optional metadata.
    pub metadata: HashMap<String, String>,
}

impl Snapshot {
    /// Creates a new snapshot.
    pub fn new(name: impl Into<String>, content: Vec<String>) -> Self {
        Self {
            name: name.into(),
            content,
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the snapshot.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns the content as a single string with newlines.
    pub fn text(&self) -> String {
        self.content.join("\n")
    }

    /// Returns the number of lines.
    pub fn line_count(&self) -> usize {
        self.content.len()
    }
}

/// Manages snapshot storage and comparison.
pub struct SnapshotManager {
    /// Directory to store snapshot files.
    storage_dir: PathBuf,
    /// In-memory cache of loaded snapshots.
    cache: HashMap<String, Snapshot>,
    /// Whether to auto-update snapshots on mismatch.
    auto_update: bool,
}

impl SnapshotManager {
    /// Creates a new SnapshotManager.
    pub fn new(storage_dir: impl Into<PathBuf>) -> Self {
        Self {
            storage_dir: storage_dir.into(),
            cache: HashMap::new(),
            auto_update: false,
        }
    }

    /// Enables auto-update mode (overwrites snapshots on mismatch).
    pub fn with_auto_update(mut self, auto: bool) -> Self {
        self.auto_update = auto;
        self
    }

    /// Saves a snapshot to disk and cache.
    pub fn save(&mut self, snapshot: &Snapshot) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.storage_dir)?;
        let path = self.snapshot_path(&snapshot.name);
        let content = snapshot.text();
        std::fs::write(&path, content)?;
        self.cache.insert(snapshot.name.clone(), snapshot.clone());
        Ok(())
    }

    /// Loads a snapshot from disk.
    pub fn load(&mut self, name: &str) -> std::io::Result<Snapshot> {
        if let Some(cached) = self.cache.get(name) {
            return Ok(cached.clone());
        }
        let path = self.snapshot_path(name);
        let content = std::fs::read_to_string(&path)?;
        let lines: Vec<String> = content.lines().map(String::from).collect();
        let snapshot = Snapshot::new(name, lines);
        self.cache.insert(name.to_string(), snapshot.clone());
        Ok(snapshot)
    }

    /// Compares a snapshot against the stored version.
    pub fn compare(&mut self, name: &str, actual: &Snapshot) -> SnapshotDiff {
        match self.load(name) {
            Ok(expected) => {
                let mut diff = SnapshotDiff::new(name);
                let max_lines = expected.content.len().max(actual.content.len());
                for i in 0..max_lines {
                    let exp = expected.content.get(i).map(|s| s.as_str());
                    let act = actual.content.get(i).map(|s| s.as_str());
                    if exp != act {
                        diff.add_diff(i, exp, act);
                    }
                }
                if diff.has_diff() && self.auto_update {
                    let _ = self.save(actual);
                    diff.auto_updated = true;
                }
                diff
            }
            Err(_) => {
                // Snapshot doesn't exist yet, save it
                let _ = self.save(actual);
                SnapshotDiff::new(name)
            }
        }
    }

    /// Returns the path where a snapshot would be stored.
    fn snapshot_path(&self, name: &str) -> PathBuf {
        self.storage_dir.join(format!("{name}.snap"))
    }

    /// Returns the storage directory.
    pub fn storage_dir(&self) -> &Path {
        &self.storage_dir
    }

    /// Clears the in-memory cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

/// The result of comparing two snapshots.
#[derive(Debug, Clone)]
pub struct SnapshotDiff {
    /// The snapshot name.
    pub name: String,
    /// Line-by-line diffs.
    pub diffs: Vec<LineDiff>,
    /// Whether auto-update was applied.
    pub auto_updated: bool,
}

#[derive(Debug, Clone)]
pub struct LineDiff {
    pub line: usize,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

impl SnapshotDiff {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            diffs: Vec::new(),
            auto_updated: false,
        }
    }

    fn add_diff(&mut self, line: usize, expected: Option<&str>, actual: Option<&str>) {
        self.diffs.push(LineDiff {
            line,
            expected: expected.map(String::from),
            actual: actual.map(String::from),
        });
    }

    /// Returns true if there are any differences.
    pub fn has_diff(&self) -> bool {
        !self.diffs.is_empty()
    }

    /// Returns the number of differing lines.
    pub fn diff_count(&self) -> usize {
        self.diffs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_creation() {
        let snap = Snapshot::new("test", vec!["line1".into(), "line2".into()]);
        assert_eq!(snap.name, "test");
        assert_eq!(snap.line_count(), 2);
        assert_eq!(snap.text(), "line1\nline2");
    }

    #[test]
    fn snapshot_metadata() {
        let snap = Snapshot::new("test", vec![])
            .with_metadata("author", "test")
            .with_metadata("version", "1.0");
        assert_eq!(snap.metadata.get("author").unwrap(), "test");
        assert_eq!(snap.metadata.get("version").unwrap(), "1.0");
    }

    #[test]
    fn manager_save_and_load() {
        let dir = std::env::temp_dir().join("bettertui_snap_test");
        let mut manager = SnapshotManager::new(&dir);
        let snap = Snapshot::new("test1", vec!["hello".into(), "world".into()]);
        manager.save(&snap).unwrap();
        let loaded = manager.load("test1").unwrap();
        assert_eq!(loaded.content, snap.content);
        // Cleanup
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compare_identical() {
        let dir = std::env::temp_dir().join("bettertui_snap_cmp");
        let mut manager = SnapshotManager::new(&dir);
        let snap = Snapshot::new("cmp1", vec!["line".into()]);
        manager.save(&snap).unwrap();
        let diff = manager.compare("cmp1", &snap);
        assert!(!diff.has_diff());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compare_different() {
        let dir = std::env::temp_dir().join("bettertui_snap_diff");
        let mut manager = SnapshotManager::new(&dir);
        let original = Snapshot::new("cmp2", vec!["old".into()]);
        manager.save(&original).unwrap();
        let updated = Snapshot::new("cmp2", vec!["new".into()]);
        let diff = manager.compare("cmp2", &updated);
        assert!(diff.has_diff());
        assert_eq!(diff.diff_count(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn compare_new_snapshot() {
        let dir = std::env::temp_dir().join("bettertui_snap_new");
        let mut manager = SnapshotManager::new(&dir);
        let snap = Snapshot::new("new1", vec!["content".into()]);
        let diff = manager.compare("new1", &snap);
        assert!(!diff.has_diff());
        // Should have been saved
        assert!(manager.load("new1").is_ok());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn auto_update() {
        let dir = std::env::temp_dir().join("bettertui_snap_auto");
        let mut manager = SnapshotManager::new(&dir).with_auto_update(true);
        let original = Snapshot::new("auto1", vec!["old".into()]);
        manager.save(&original).unwrap();
        let updated = Snapshot::new("auto1", vec!["new".into()]);
        let diff = manager.compare("auto1", &updated);
        assert!(diff.auto_updated);
        let loaded = manager.load("auto1").unwrap();
        assert_eq!(loaded.content, vec!["new"]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
