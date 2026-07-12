//! Filesystem service for directory trees and file watching.
//!
//! Provides directory traversal, file metadata, and basic filesystem operations
//! for building file explorers and tree views.

use std::path::{Path, PathBuf};

/// Entry type in the filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EntryType {
    /// A regular file.
    #[default]
    File,
    /// A directory.
    Directory,
    /// A symbolic link.
    Symlink,
}

/// An entry in the filesystem tree.
#[derive(Debug, Clone)]
pub struct FileEntry {
    /// The file/directory name.
    pub name: String,
    /// Full path.
    pub path: PathBuf,
    /// Entry type.
    pub entry_type: EntryType,
    /// File size in bytes (0 for directories).
    pub size: u64,
    /// Whether the entry is hidden (starts with '.').
    pub hidden: bool,
    /// Whether the directory is expanded (for tree views).
    pub expanded: bool,
}

impl FileEntry {
    /// Creates a new file entry.
    pub fn new(name: impl Into<String>, path: PathBuf, entry_type: EntryType) -> Self {
        let name_str: String = name.into();
        Self {
            hidden: name_str.starts_with('.'),
            name: name_str,
            path,
            entry_type,
            size: 0,
            expanded: false,
        }
    }

    /// Returns true if this is a directory.
    pub fn is_dir(&self) -> bool {
        self.entry_type == EntryType::Directory
    }

    /// Returns true if this is a file.
    pub fn is_file(&self) -> bool {
        self.entry_type == EntryType::File
    }
}

/// Reads directory entries from the filesystem.
pub fn read_dir(path: &Path) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();
        let entry_type = if metadata.is_dir() {
            EntryType::Directory
        } else if metadata.is_symlink() {
            EntryType::Symlink
        } else {
            EntryType::File
        };
        let mut file_entry = FileEntry::new(name, entry.path(), entry_type);
        file_entry.size = metadata.len();
        entries.push(file_entry);
    }
    entries.sort_by(|a, b| {
        // Directories first, then alphabetically
        match (a.is_dir(), b.is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    Ok(entries)
}

/// Returns the home directory.
pub fn home_dir() -> Option<PathBuf> {
    dirs_home()
}

#[cfg(unix)]
fn dirs_home() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

#[cfg(windows)]
fn dirs_home() -> Option<PathBuf> {
    std::env::var("USERPROFILE").ok().map(PathBuf::from)
}

/// Returns the current working directory.
pub fn current_dir() -> std::io::Result<PathBuf> {
    std::env::current_dir()
}

/// Returns the file extension (without the dot).
pub fn extension(path: &Path) -> Option<String> {
    path.extension().and_then(|e| e.to_str()).map(String::from)
}

/// Returns the file name without extension.
pub fn stem(path: &Path) -> Option<String> {
    path.file_stem().and_then(|e| e.to_str()).map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_entry_creation() {
        let entry = FileEntry::new("test.txt", PathBuf::from("/tmp/test.txt"), EntryType::File);
        assert_eq!(entry.name, "test.txt");
        assert!(entry.is_file());
        assert!(!entry.is_dir());
        assert!(!entry.hidden);
    }

    #[test]
    fn hidden_file() {
        let entry = FileEntry::new(".hidden", PathBuf::from("/tmp/.hidden"), EntryType::File);
        assert!(entry.hidden);
    }

    #[test]
    fn directory_entry() {
        let entry = FileEntry::new("dir", PathBuf::from("/tmp/dir"), EntryType::Directory);
        assert!(entry.is_dir());
        assert!(!entry.is_file());
    }

    #[test]
    fn read_current_dir() {
        let entries = read_dir(&std::env::current_dir().unwrap()).unwrap();
        assert!(!entries.is_empty());
    }

    #[test]
    fn home_dir_exists() {
        let home = home_dir();
        assert!(home.is_some());
    }

    #[test]
    fn extension_extraction() {
        assert_eq!(extension(Path::new("file.txt")), Some("txt".to_string()));
        assert_eq!(extension(Path::new("noext")), None);
        assert_eq!(
            extension(Path::new("archive.tar.gz")),
            Some("gz".to_string())
        );
    }

    #[test]
    fn stem_extraction() {
        assert_eq!(stem(Path::new("file.txt")), Some("file".to_string()));
        assert_eq!(
            stem(Path::new("archive.tar.gz")),
            Some("archive.tar".to_string())
        );
    }

    #[test]
    fn current_dir_works() {
        let dir = current_dir().unwrap();
        assert!(dir.exists());
    }
}
