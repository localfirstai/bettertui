//! FFI bridge for exposing the engine to foreign language bindings.
//!
//! Provides a stable C-compatible interface for integrating BetterTUI
//! with other languages and runtimes (Node.js via NAPI, Rust, etc.).

// === mod.rs ===

use crate::engine::Engine;

pub use filesystem::{EntryType, FileEntry, home_dir, read_dir};

/// Opaque handle to an Engine instance for FFI.
pub struct FfiEngine {
    inner: Engine,
}

impl Default for FfiEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FfiEngine {
    pub fn new() -> Self {
        Self { inner: Engine::new() }
    }

    /// Returns a reference to the inner engine.
    pub fn engine(&self) -> &Engine {
        &self.inner
    }

    /// Returns a mutable reference to the inner engine.
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.inner
    }
}

/// Result code for FFI operations.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiResult {
    /// Operation succeeded.
    Success = 0,
    /// Invalid argument.
    InvalidArgument = 1,
    /// Operation failed.
    Failed = 2,
    /// Out of memory.
    OutOfMemory = 3,
}

/// Creates a new FFI engine handle.
///
/// # Safety
/// The returned handle must be freed with `ffi_engine_destroy`.
pub unsafe extern "C" fn ffi_engine_create() -> *mut FfiEngine {
    Box::into_raw(Box::new(FfiEngine::new()))
}

/// Destroys an FFI engine handle.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
/// This must not be called twice with the same handle.
pub unsafe extern "C" fn ffi_engine_destroy(handle: *mut FfiEngine) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle));
        }
    }
}

/// Returns the number of nodes in the tree.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_node_count(handle: *const FfiEngine) -> u32 {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).inner.node_count() as u32 }
}

/// Returns the frame count.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_frame_count(handle: *const FfiEngine) -> u64 {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).inner.frame_count() }
}

/// Begins a new frame.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_begin_frame(handle: *mut FfiEngine) {
    if !handle.is_null() {
        unsafe {
            (*handle).inner.begin_frame();
        }
    }
}

/// Commits the current frame.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_commit_frame(handle: *mut FfiEngine) {
    if !handle.is_null() {
        unsafe {
            (*handle).inner.commit_frame();
        }
    }
}

/// Prints the tree for debugging. Returns a null pointer on error.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
/// The caller must free the returned string with `ffi_string_free`.
pub unsafe extern "C" fn ffi_engine_print_tree(handle: *const FfiEngine) -> *mut std::ffi::c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }
    let output = unsafe { (*handle).inner.print_tree() };
    match std::ffi::CString::new(output) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a string returned by FFI functions.
///
/// # Safety
/// `s` must be a valid pointer returned by an FFI function.
pub unsafe extern "C" fn ffi_string_free(s: *mut std::ffi::c_char) {
    if !s.is_null() {
        unsafe {
            drop(std::ffi::CString::from_raw(s));
        }
    }
}

/// Returns the tree summary. Caller must free with ffi_string_free.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_tree_summary(handle: *const FfiEngine) -> *mut std::ffi::c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }
    let output = unsafe { (*handle).inner.tree_summary() };
    match std::ffi::CString::new(output) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Validates the tree. Returns FfiResult::Success if valid.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_validate(handle: *const FfiEngine) -> FfiResult {
    if handle.is_null() {
        return FfiResult::InvalidArgument;
    }
    unsafe {
        match (*handle).inner.validate() {
            Ok(()) => FfiResult::Success,
            Err(_) => FfiResult::Failed,
        }
    }
}

// === filesystem.rs ===

mod filesystem {
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
            Self { hidden: name_str.starts_with('.'), name: name_str, path, entry_type, size: 0, expanded: false }
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
        entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
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
    #[allow(dead_code)]
    pub fn current_dir() -> std::io::Result<PathBuf> {
        std::env::current_dir()
    }

    /// Returns the extension of a path.
    #[allow(dead_code)]
    pub fn extension(path: &Path) -> Option<String> {
        path.extension().map(|e| e.to_string_lossy().to_string())
    }

    /// Returns the file name stem (without extension).
    #[allow(dead_code)]
    pub fn stem(path: &Path) -> Option<String> {
        path.file_stem().map(|s| s.to_string_lossy().to_string())
    }
}
