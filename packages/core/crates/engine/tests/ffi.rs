use bettertui_engine::ffi::*;

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
        assert_eq!(ffi_engine_validate(std::ptr::null()), FfiResult::InvalidArgument);
    }
}

#[test]
fn ffi_node_count() {
    unsafe {
        let handle = ffi_engine_create();
        assert_eq!(ffi_engine_node_count(handle), 1);
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
        ffi_string_free(std::ptr::null_mut());
    }
}

#[test]
fn file_entry_creation() {
    use std::path::PathBuf;
    let entry = FileEntry::new("test.txt", PathBuf::from("/tmp/test.txt"), EntryType::File);
    assert_eq!(entry.name, "test.txt");
    assert!(entry.is_file());
    assert!(!entry.is_dir());
    assert!(!entry.hidden);
}

#[test]
fn hidden_file() {
    use std::path::PathBuf;
    let entry = FileEntry::new(".hidden", PathBuf::from("/tmp/.hidden"), EntryType::File);
    assert!(entry.hidden);
}

#[test]
fn directory_entry() {
    use std::path::PathBuf;
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
