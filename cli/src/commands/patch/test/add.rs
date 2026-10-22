//! Add-operation regression tests. / Regression test thao tác add.

use super::support::project;
use super::*;
use mgc_config::project::ProjectConfig;
use mgc_resolver::patches::get_patches_dir;
use std::path::Path;
use std::time::Duration;

#[tokio::test]
async fn patch_add_refuses_to_race_install_before_writing_anything() {
    let temp = tempfile::tempdir().unwrap();
    project(temp.path());
    let source = temp.path().join("input.patch");
    std::fs::write(&source, "patch bytes").unwrap();
    let before = std::fs::read(temp.path().join("mgc.toml")).unwrap();
    let _held =
        mgc_lockfile::project_lock::ProjectWriteLock::acquire(temp.path(), Duration::from_secs(1))
            .unwrap();

    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Add {
                package: "example".to_string(),
                file: source.to_string_lossy().into_owned(),
                range: None,
            },
        },
        temp.path(),
    )
    .await;

    assert!(result.is_err(), "patch add must not race install");
    assert_eq!(std::fs::read(temp.path().join("mgc.toml")).unwrap(), before);
    assert!(!get_patches_dir(Some(temp.path())).unwrap().exists());
}

#[tokio::test]
async fn patch_add_rejects_oversized_source_before_mutation() {
    let temp = tempfile::tempdir().unwrap();
    project(temp.path());
    let source = temp.path().join("oversized.patch");
    let file = std::fs::File::create(&source).unwrap();
    file.set_len(MAX_PATCH_VERIFY_BYTES + 1).unwrap();
    let config_before = std::fs::read(temp.path().join("mgc.toml")).unwrap();

    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Add {
                package: "example".to_string(),
                file: source.to_string_lossy().into_owned(),
                range: None,
            },
        },
        temp.path(),
    )
    .await;

    assert!(result.is_err(), "oversized patch source must be rejected");
    assert_eq!(
        std::fs::read(temp.path().join("mgc.toml")).unwrap(),
        config_before
    );
    assert!(!temp.path().join(".magicore/patches").exists());
}

#[tokio::test]
async fn patch_add_uses_distinct_paths_for_sanitization_collisions() {
    let temp = tempfile::tempdir().unwrap();
    project(temp.path());
    for (package, file_name, contents) in [
        ("foo/bar", "one.patch", b"first".as_slice()),
        ("foo_bar", "two.patch", b"second".as_slice()),
    ] {
        let source = temp.path().join(file_name);
        std::fs::write(&source, contents).unwrap();
        run_at_project_root(
            PatchArgs {
                cmd: PatchCmd::Add {
                    package: package.to_string(),
                    file: source.to_string_lossy().into_owned(),
                    range: None,
                },
            },
            temp.path(),
        )
        .await
        .unwrap();
    }

    let config = ProjectConfig::load(temp.path()).unwrap().unwrap();
    assert_eq!(config.patches.len(), 2);
    assert_ne!(
        config.patches[0].patch_path, config.patches[1].patch_path,
        "distinct package identities must not alias the same patch file"
    );
}

#[tokio::test]
async fn patch_add_rejects_duplicate_package_range_without_overwriting() {
    let temp = tempfile::tempdir().unwrap();
    project(temp.path());
    let first = temp.path().join("first.patch");
    let second = temp.path().join("second.patch");
    std::fs::write(&first, b"first patch").unwrap();
    std::fs::write(&second, b"replacement patch").unwrap();
    let add = |file: &Path| PatchArgs {
        cmd: PatchCmd::Add {
            package: "example".to_string(),
            file: file.to_string_lossy().into_owned(),
            range: Some("^1.0.0".to_string()),
        },
    };
    run_at_project_root(add(&first), temp.path()).await.unwrap();
    let original_config = std::fs::read(temp.path().join("mgc.toml")).unwrap();
    let original_patch = std::fs::read_dir(temp.path().join(".magicore/patches"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let original_bytes = std::fs::read(original_patch.path()).unwrap();

    let result = run_at_project_root(add(&second), temp.path()).await;

    assert!(
        result.is_err(),
        "duplicate package+range must require explicit removal"
    );
    assert_eq!(
        std::fs::read(temp.path().join("mgc.toml")).unwrap(),
        original_config
    );
    assert_eq!(
        std::fs::read(original_patch.path()).unwrap(),
        original_bytes
    );
}

#[cfg(unix)]
#[tokio::test]
async fn patch_add_refuses_symlinked_patch_directory() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    project(temp.path());
    let outside = temp.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    let magicore = temp.path().join(".magicore");
    std::fs::create_dir(&magicore).unwrap();
    symlink(&outside, magicore.join("patches")).unwrap();
    let source = temp.path().join("input.patch");
    std::fs::write(&source, "patch bytes").unwrap();

    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Add {
                package: "example".to_string(),
                file: source.to_string_lossy().into_owned(),
                range: None,
            },
        },
        temp.path(),
    )
    .await;

    assert!(
        result.is_err(),
        "a symlinked patch directory must be refused"
    );
    assert_eq!(std::fs::read_dir(outside).unwrap().count(), 0);
}
