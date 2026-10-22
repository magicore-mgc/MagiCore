//! Tests that symlinked profiles fail closed without changing their target.
//! Kiểm tra profile là symlink bị từ chối và target không bị sửa.

use super::super::ensure_profile_entry;

#[cfg(unix)]
#[test]
fn symlinked_profile_is_rejected_without_modifying_target() {
    let home = tempfile::tempdir().unwrap();
    let target = home.path().join("profile-target");
    let profile = home.path().join(".profile");
    let bin = home.path().join("bin");
    std::fs::write(&target, "# user profile\n").unwrap();
    std::fs::create_dir(&bin).unwrap();
    std::os::unix::fs::symlink(&target, &profile).unwrap();

    let error = ensure_profile_entry(&profile, &bin, false)
        .expect_err("symlinked shell profiles must be rejected");

    assert!(error.to_string().contains("must not be a symlink"));
    assert_eq!(std::fs::read_to_string(target).unwrap(), "# user profile\n");
}
