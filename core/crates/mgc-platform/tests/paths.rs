#![allow(clippy::unwrap_used)]
//! Integration tests for mgc-platform paths — test riêng tại test/ (RULE §5)
use mgc_platform::paths::{GlobalPaths, ProjectPaths};
use std::process::Command;

#[test]
fn project_paths_computes_correctly() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = ProjectPaths::from_root(tmp.path());
    assert!(paths.patches_dir().ends_with(".magicore/patches"));
    assert!(
        paths
            .lock_signatures
            .ends_with(".magicore/lock-signatures.json")
    );
}

#[test]
fn ensure_dirs_creates_all() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = ProjectPaths::from_root(tmp.path());
    paths.ensure_dirs().unwrap();
    assert!(tmp.path().join(".magicore/patches").exists());
    // lock_signatures is a file, not dir
    assert!(tmp.path().join(".magicore").exists());
}

#[test]
fn global_paths_creates_patches_dir() {
    let paths = GlobalPaths::new().unwrap();
    assert_eq!(
        paths.patches_dir(),
        mgc_platform::paths::mgc_data_dir().join("patches")
    );
}

#[test]
fn global_paths_honors_store_root_override() {
    let temp = tempfile::tempdir().unwrap();
    let store_root = temp.path().join("isolated-store");
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "global_paths_store_root_override_child",
            "--nocapture",
        ])
        .env("MAGICORE_STORE_ROOT", &store_root)
        .env("MGC_TEST_EXPECTED_STORE_ROOT", &store_root)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "store-root override child failed:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn global_paths_store_root_override_child() {
    let Some(expected) = std::env::var_os("MGC_TEST_EXPECTED_STORE_ROOT") else {
        return;
    };
    assert_eq!(
        GlobalPaths::new().unwrap().store,
        std::path::PathBuf::from(expected)
    );
}

#[test]
fn find_project_root_detects_mgc_toml() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("mgc.toml"), "").unwrap();
    let found = mgc_platform::paths::find_project_root(tmp.path()).unwrap();
    assert_eq!(found, tmp.path());
}

#[test]
fn project_data_paths_are_stable_and_separate_same_named_projects() {
    let first_parent = tempfile::tempdir().unwrap();
    let second_parent = tempfile::tempdir().unwrap();
    let first = first_parent.path().join("app");
    let second = second_parent.path().join("app");
    std::fs::create_dir_all(&first).unwrap();
    std::fs::create_dir_all(&second).unwrap();

    let first_path = mgc_platform::paths::project_data_dir(&first);
    assert_eq!(first_path, mgc_platform::paths::project_data_dir(&first));
    assert_ne!(first_path, mgc_platform::paths::project_data_dir(&second));
    assert!(first_path.starts_with(mgc_platform::paths::mgc_data_dir()));
}

#[test]
fn mgc_data_dir_honors_absolute_override_in_child_process() {
    let temp = tempfile::tempdir().unwrap();
    let expected = temp.path().join("central-mgc-data");
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "mgc_data_dir_override_child", "--nocapture"])
        .env("MGC_DATA_DIR", &expected)
        .env("MGC_TEST_EXPECTED_DATA_DIR", &expected)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "data-dir override child failed:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn mgc_data_dir_override_child() {
    let Some(expected) = std::env::var_os("MGC_TEST_EXPECTED_DATA_DIR") else {
        return;
    };
    assert_eq!(
        mgc_platform::paths::mgc_data_dir(),
        std::path::PathBuf::from(expected)
    );
}

#[cfg(not(windows))]
#[test]
fn managed_shell_path_block_is_idempotent_and_tracks_data_root_changes() {
    let home = tempfile::tempdir().unwrap();
    let spaced = home.path().join("data root/bin");
    std::fs::create_dir_all(&spaced).unwrap();
    let first = mgc_platform::paths::ensure_bin_on_path(home.path(), "/bin/zsh", &spaced).unwrap();
    let profile = home.path().join(".zprofile");
    let initial = std::fs::read_to_string(&profile).unwrap();
    assert!(initial.contains(&first.to_string_lossy().to_string()));
    assert!(initial.contains("case \":$PATH:\""));

    mgc_platform::paths::ensure_bin_on_path(home.path(), "/bin/zsh", &spaced).unwrap();
    assert_eq!(std::fs::read_to_string(&profile).unwrap(), initial);

    let second = home.path().join("alternate/bin");
    std::fs::create_dir_all(&second).unwrap();
    mgc_platform::paths::ensure_bin_on_path(home.path(), "/bin/zsh", &second).unwrap();
    let updated = std::fs::read_to_string(profile).unwrap();
    assert!(updated.contains(&second.canonicalize().unwrap().to_string_lossy().to_string()));
    assert!(!updated.contains(&first.to_string_lossy().to_string()));
    assert_eq!(updated.matches("magicore-managed-tool-bin >>>").count(), 1);
}

#[cfg(not(windows))]
#[test]
fn managed_fish_path_uses_conf_d_and_rejects_symlinked_profiles() {
    let home = tempfile::tempdir().unwrap();
    let bin = home.path().join("data/bin");
    std::fs::create_dir_all(&bin).unwrap();
    mgc_platform::paths::ensure_bin_on_path(home.path(), "/usr/bin/fish", &bin).unwrap();
    let profile = home.path().join(".config/fish/conf.d/magicore.fish");
    let contents = std::fs::read_to_string(profile).unwrap();
    assert!(contents.contains("set -gx PATH"));

    let zsh_profile = home.path().join(".zprofile");
    let target = home.path().join("profile-target");
    std::fs::write(&target, "# user profile\n").unwrap();
    std::os::unix::fs::symlink(&target, &zsh_profile).unwrap();
    assert!(mgc_platform::paths::ensure_bin_on_path(home.path(), "/bin/zsh", &bin).is_err());
    assert_eq!(std::fs::read_to_string(target).unwrap(), "# user profile\n");
}

#[cfg(unix)]
#[test]
fn managed_bin_rejects_a_symlink_before_updating_the_path() {
    let home = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let link = home.path().join("bin");
    std::os::unix::fs::symlink(outside.path(), &link).unwrap();

    assert!(mgc_platform::paths::ensure_bin_on_path(home.path(), "/bin/zsh", &link).is_err());
    assert!(!home.path().join(".zprofile").exists());
}
