//! Verify-operation regression tests. / Regression test thao tác verify.

use super::support::project;
use super::*;
use mgc_config::project::ProjectConfig;
use mgc_types::PatchSpec;

#[tokio::test]
async fn patch_verify_rejects_paths_escaping_project_patch_store() {
    use sha2::{Digest, Sha256};

    let temp = tempfile::tempdir().unwrap();
    let outside = temp.path().join("outside.patch");
    let bytes = b"outside patch contents";
    std::fs::write(&outside, bytes).unwrap();
    std::fs::create_dir_all(temp.path().join(".magicore/patches")).unwrap();
    let integrity = format!("sha256-{}", hex::encode(Sha256::digest(bytes)));
    let mut config = ProjectConfig::new("patch-test", "web");
    config.patches.push(PatchSpec::new(
        "example".to_string(),
        mgc_types::VersionRange::star(),
        "../../outside.patch".to_string(),
        integrity,
    ));
    config.save(temp.path()).unwrap();

    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Verify,
        },
        temp.path(),
    )
    .await;

    assert!(
        result.is_err(),
        "verify must not trust a patch path outside its store"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn patch_verify_rejects_symlinked_patch_file() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let patches = temp.path().join(".magicore/patches");
    std::fs::create_dir_all(&patches).unwrap();
    let outside = temp.path().join("outside.patch");
    std::fs::write(&outside, "outside patch contents").unwrap();
    symlink(&outside, patches.join("example.patch")).unwrap();
    let mut config = ProjectConfig::new("patch-test", "web");
    config.patches.push(PatchSpec::new(
        "example".to_string(),
        mgc_types::VersionRange::star(),
        "example.patch".to_string(),
        "sha256-invalid-is-irrelevant".to_string(),
    ));
    config.save(temp.path()).unwrap();

    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Verify,
        },
        temp.path(),
    )
    .await;

    assert!(
        result.is_err(),
        "verify must not follow a patch-file symlink"
    );
}

#[tokio::test]
async fn patch_verify_rejects_oversized_patch_without_reading_it() {
    let temp = tempfile::tempdir().unwrap();
    let patches = temp.path().join(".magicore/patches");
    std::fs::create_dir_all(&patches).unwrap();
    let patch_path = patches.join("large.patch");
    let file = std::fs::File::create(&patch_path).unwrap();
    file.set_len(MAX_PATCH_VERIFY_BYTES + 1).unwrap();
    let mut config = ProjectConfig::new("patch-test", "web");
    config.patches.push(PatchSpec::new(
        "example".to_string(),
        mgc_types::VersionRange::star(),
        "large.patch".to_string(),
        "sha256-unused".to_string(),
    ));
    config.save(temp.path()).unwrap();

    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Verify,
        },
        temp.path(),
    )
    .await;

    assert!(
        result.is_err(),
        "oversized patch must fail before buffering"
    );
}

#[tokio::test]
async fn patch_verify_accepts_project_owned_file_with_matching_digest() {
    let temp = tempfile::tempdir().unwrap();
    project(temp.path());
    let source = temp.path().join("input.patch");
    std::fs::write(&source, b"bounded patch content").unwrap();

    run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Add {
                package: "example".to_string(),
                file: source.to_string_lossy().into_owned(),
                range: None,
            },
        },
        temp.path(),
    )
    .await
    .unwrap();
    let result = run_at_project_root(
        PatchArgs {
            cmd: PatchCmd::Verify,
        },
        temp.path(),
    )
    .await;

    assert!(result.is_ok(), "a valid project patch must verify");
}
