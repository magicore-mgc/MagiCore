//! Shadow-path ownership and cleanup contracts.
//! Hợp đồng sở hữu và dọn dẹp đường dẫn tạm.

// `unique_temp_dir` is owned by the shadow-path module and is only reachable
// through the `run::implementation` re-export chain; the other two names come
// from the public `environment` module because `run::implementation` does not
// forward `EnvironmentError`.
// (`unique_temp_dir` thuộc owner shadow-path và chỉ tới được qua chuỗi re-export
// `run::implementation`; hai tên còn lại lấy từ module `environment` đã public,
// vì `run::implementation` không chuyển tiếp `EnvironmentError`.)
#[cfg(any(windows, test))]
use crate::run::implementation::environment::{EnvironmentError, ShadowPath};
#[cfg(any(windows, test))]
use crate::run::implementation::unique_temp_dir;

#[test]
fn explicit_shadow_path_cleanup_removes_owned_directory() {
    let path =
        unique_temp_dir("mgc-exec-shadow-cleanup-test").expect("allocate unique cleanup test path");
    let mut shadow =
        ShadowPath::create_at(path.clone(), &[]).expect("create guarded temporary directory");
    shadow.cleanup().expect("surface explicit cleanup result");

    assert!(
        !path.exists(),
        "explicit cleanup removes the owned directory"
    );
    shadow
        .cleanup()
        .expect("explicit cleanup is idempotent after success");
}

#[cfg(unix)]
#[test]
fn cleanup_rejects_symlink_substitution_without_touching_target() {
    use std::os::unix::fs::PermissionsExt;

    let root = tempfile::tempdir().expect("create symlink cleanup fixture");
    let guarded_path = root.path().join("guarded");
    let mut shadow = ShadowPath::create_at(guarded_path.clone(), &[])
        .expect("create guarded temporary directory");
    let moved_path = root.path().join("original-guarded");
    std::fs::rename(&guarded_path, &moved_path).expect("move owned directory aside");

    let target = root.path().join("attacker-owned");
    std::fs::create_dir(&target).expect("create protected target directory");
    let protected_file = target.join("keep.txt");
    std::fs::write(&protected_file, "preserve this file").expect("create protected file");
    let original_permissions = std::fs::metadata(&target)
        .expect("read target permissions")
        .permissions()
        .mode();
    std::os::unix::fs::symlink(&target, &guarded_path).expect("replace cleanup path with symlink");

    let error = shadow.cleanup().expect_err("reject a substituted symlink");

    assert!(matches!(
        error,
        EnvironmentError::RestoreDirectoryPermissions(_)
    ));
    assert_eq!(
        std::fs::read_to_string(protected_file).expect("read protected file"),
        "preserve this file"
    );
    assert_eq!(
        std::fs::metadata(&target)
            .expect("read target permissions after cleanup attempt")
            .permissions()
            .mode(),
        original_permissions,
        "cleanup must not chmod the symlink target"
    );

    let mut moved_permissions = std::fs::metadata(&moved_path)
        .expect("read moved guarded-directory permissions")
        .permissions();
    moved_permissions.set_mode(0o700);
    std::fs::set_permissions(&moved_path, moved_permissions).expect("make moved fixture removable");
    std::fs::remove_dir_all(moved_path).expect("remove moved guarded fixture");
}
