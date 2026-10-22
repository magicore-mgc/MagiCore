//! Work-root error contract regression.
//! Test hồi quy contract lỗi work root.

use super::*;
use std::path::Path;

#[test]
fn unsafe_cache_path_is_typed_and_keeps_its_path_context() {
    let error = registry_package_cache_path_unsafe(Path::new("/tmp/mgc-cache"));
    assert_eq!(
        error.to_string(),
        "package runner: cache directory '/tmp/mgc-cache' must be a real directory without symlinks"
    );
    assert!(matches!(
        error.downcast_ref::<WorkRootError>(),
        Some(WorkRootError::UnsafeCachePath { path }) if path == Path::new("/tmp/mgc-cache")
    ));
}
