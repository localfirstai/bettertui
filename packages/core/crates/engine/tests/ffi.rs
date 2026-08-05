use bettertui_engine::ffi::*;

#[test]
fn ffi_create_destroy() {
    unsafe {
        let handle = ffi_engine_create(80, 24);
        assert_ne!(handle, 0);
        ffi_engine_destroy(handle);
    }
}

#[test]
fn ffi_null_safety() {
    unsafe {
        assert_eq!(ffi_engine_node_count(0), 0);
        assert_eq!(ffi_engine_frame_count(0), 0);
        assert_eq!(ffi_engine_validate(0), -1);
    }
}

#[test]
fn ffi_node_count() {
    unsafe {
        let handle = ffi_engine_create(80, 24);
        assert_eq!(ffi_engine_node_count(handle), 1);
        ffi_engine_destroy(handle);
    }
}

#[test]
fn ffi_frame_count() {
    unsafe {
        let handle = ffi_engine_create(80, 24);
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
        let handle = ffi_engine_create(80, 24);
        let tree = ffi_engine_print_tree(handle);
        assert!(!tree.is_null());
        ffi_free_string(tree);
        ffi_engine_destroy(handle);
    }
}

#[test]
fn ffi_tree_summary() {
    unsafe {
        let handle = ffi_engine_create(80, 24);
        let summary = ffi_engine_tree_summary(handle);
        assert!(!summary.is_null());
        let s = std::ffi::CStr::from_ptr(summary).to_str().unwrap();
        assert!(s.contains("Nodes: 1"));
        ffi_free_string(summary);
        ffi_engine_destroy(handle);
    }
}

#[test]
fn ffi_validate() {
    unsafe {
        let handle = ffi_engine_create(80, 24);
        assert_eq!(ffi_engine_validate(handle), 0);
        ffi_engine_destroy(handle);
    }
}

#[test]
fn ffi_free_string_null() {
    unsafe {
        ffi_free_string(std::ptr::null_mut());
    }
}
