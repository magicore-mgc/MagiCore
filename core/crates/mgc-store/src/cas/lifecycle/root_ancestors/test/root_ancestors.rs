//! CAS ancestor redirection tests.
//! Test chuyển hướng ancestor CAS.

#[cfg(unix)]
#[test]
fn symlink_ancestor_obeys_root_owner_allowance() {
    use std::os::unix::fs::{MetadataExt, symlink};

    let parent = tempfile::tempdir().unwrap();
    let real = parent.path().join("real");
    std::fs::create_dir(&real).unwrap();
    let link = parent.path().join("link");
    symlink(&real, &link).unwrap();
    let owner = std::fs::symlink_metadata(&link).unwrap().uid();
    let candidate = link.join("cas");
    let result = super::super::check_cas_root_ancestors(&candidate);
    if owner == 0 {
        assert!(
            result.is_ok(),
            "root-owned system symlinks are explicitly allowed"
        );
    } else {
        assert!(
            result.is_err(),
            "symlink ancestors owned by non-root are rejected"
        );
    }
}
