//! FFI bridge for exposing the engine to foreign language bindings.
//!
//! Provides a stable C-compatible interface for integrating BetterTUI
//! with other languages and runtimes (Node.js via NAPI, Rust, etc.).

mod filesystem;

pub use filesystem::{FileEntry, home_dir, read_dir};

use crate::engine::Engine;

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
        Self {
            inner: Engine::new(),
        }
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
        unsafe { drop(Box::from_raw(handle)) };
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
        unsafe { (*handle).inner.begin_frame() };
    }
}

/// Commits the current frame.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_commit_frame(handle: *mut FfiEngine) {
    if !handle.is_null() {
        unsafe { (*handle).inner.commit_frame() };
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
        unsafe { drop(std::ffi::CString::from_raw(s)) };
    }
}

/// Returns the tree summary. Caller must free with ffi_string_free.
///
/// # Safety
/// `handle` must be a valid pointer returned by `ffi_engine_create`.
pub unsafe extern "C" fn ffi_engine_tree_summary(
    handle: *const FfiEngine,
) -> *mut std::ffi::c_char {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ffi_result_values() {
        assert_eq!(FfiResult::Success as i32, 0);
        assert_eq!(FfiResult::InvalidArgument as i32, 1);
        assert_eq!(FfiResult::Failed as i32, 2);
        assert_eq!(FfiResult::OutOfMemory as i32, 3);
    }

    #[test]
    fn ffi_create_destroy() {
        unsafe {
            let handle = ffi_engine_create();
            assert!(!handle.is_null());
            ffi_engine_destroy(handle);
        }
    }

    #[test]
    fn ffi_null_safety() {
        unsafe {
            assert_eq!(ffi_engine_node_count(std::ptr::null()), 0);
            assert_eq!(ffi_engine_frame_count(std::ptr::null()), 0);
            assert!(ffi_engine_print_tree(std::ptr::null()).is_null());
            assert!(ffi_engine_tree_summary(std::ptr::null()).is_null());
            assert_eq!(
                ffi_engine_validate(std::ptr::null()),
                FfiResult::InvalidArgument
            );
        }
    }

    #[test]
    fn ffi_node_count() {
        unsafe {
            let handle = ffi_engine_create();
            assert_eq!(ffi_engine_node_count(handle), 1); // root node
            ffi_engine_destroy(handle);
        }
    }

    #[test]
    fn ffi_frame_count() {
        unsafe {
            let handle = ffi_engine_create();
            assert_eq!(ffi_engine_frame_count(handle), 0);
            ffi_engine_begin_frame(handle);
            assert_eq!(ffi_engine_frame_count(handle), 1);
            ffi_engine_commit_frame(handle);
            ffi_engine_destroy(handle);
        }
    }

    #[test]
    fn ffi_print_tree() {
        unsafe {
            let handle = ffi_engine_create();
            let tree = ffi_engine_print_tree(handle);
            assert!(!tree.is_null());
            ffi_string_free(tree);
            ffi_engine_destroy(handle);
        }
    }

    #[test]
    fn ffi_tree_summary() {
        unsafe {
            let handle = ffi_engine_create();
            let summary = ffi_engine_tree_summary(handle);
            assert!(!summary.is_null());
            let s = std::ffi::CStr::from_ptr(summary).to_str().unwrap();
            assert!(s.contains("Nodes: 1"));
            ffi_string_free(summary);
            ffi_engine_destroy(handle);
        }
    }

    #[test]
    fn ffi_validate() {
        unsafe {
            let handle = ffi_engine_create();
            assert_eq!(ffi_engine_validate(handle), FfiResult::Success);
            ffi_engine_destroy(handle);
        }
    }

    #[test]
    fn ffi_engine_wrapper() {
        unsafe {
            let handle = ffi_engine_create();
            let engine = (*handle).engine();
            assert_eq!(engine.node_count(), 1);
            ffi_engine_destroy(handle);
        }
    }

    #[test]
    fn ffi_string_free_null() {
        unsafe {
            ffi_string_free(std::ptr::null_mut()); // should not panic
        }
    }
}
