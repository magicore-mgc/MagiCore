#![allow(clippy::unwrap_used)]
//! Symlink containment and dispatch-preservation tests.
//! Test containment symlink và giữ nguyên dispatch.

use super::super::super::SourceFileResolutionError;
use super::super::super::resolve_source_file;

#[cfg(unix)]
#[tokio::test]
async fn internal_file_symlinks_are_allowed_when_extension_is_preserved() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("src/actual.txt");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(&target, "in-project").unwrap();
    let alias = temp.path().join("src/alias.txt");
    std::os::unix::fs::symlink(&target, &alias).unwrap();

    let resolved = resolve_source_file(temp.path(), "src/alias.txt")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resolved, target.canonicalize().unwrap());
}

#[cfg(unix)]
#[tokio::test]
async fn external_leaf_symlinks_are_forbidden() {
    let temp = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let secret = outside.path().join("secret.txt");
    std::fs::write(&secret, "outside").unwrap();
    let alias = temp.path().join("public/alias.txt");
    std::fs::create_dir_all(alias.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&secret, &alias).unwrap();

    assert!(matches!(
        resolve_source_file(temp.path(), "alias.txt").await,
        Err(SourceFileResolutionError::Forbidden)
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn external_ancestor_symlinks_are_forbidden() {
    let temp = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let secret = outside.path().join("nested/secret.txt");
    std::fs::create_dir_all(secret.parent().unwrap()).unwrap();
    std::fs::write(&secret, "outside").unwrap();
    std::os::unix::fs::symlink(outside.path(), temp.path().join("linked")).unwrap();

    assert!(matches!(
        resolve_source_file(temp.path(), "linked/nested/secret.txt").await,
        Err(SourceFileResolutionError::Forbidden)
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn symlink_aliases_to_hidden_targets_are_forbidden() {
    let temp = tempfile::tempdir().unwrap();
    let hidden = temp.path().join(".env");
    std::fs::write(&hidden, "secret").unwrap();
    let alias = temp.path().join("public/settings.txt");
    std::fs::create_dir_all(alias.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&hidden, &alias).unwrap();

    assert!(matches!(
        resolve_source_file(temp.path(), "settings.txt").await,
        Err(SourceFileResolutionError::Forbidden)
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn symlink_aliases_cannot_change_the_response_extension() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("src/actual.ts");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(&target, "export const value = true;").unwrap();
    let alias = temp.path().join("src/value.txt");
    std::os::unix::fs::symlink(&target, &alias).unwrap();

    assert!(matches!(
        resolve_source_file(temp.path(), "src/value.txt").await,
        Err(SourceFileResolutionError::Forbidden)
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn well_known_symlink_cannot_redirect_out_of_its_public_directory() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("public/other/security.txt");
    std::fs::create_dir_all(target.parent().unwrap()).unwrap();
    std::fs::write(&target, "not-the-well-known-owner").unwrap();
    let alias = temp.path().join("public/.well-known/security.txt");
    std::fs::create_dir_all(alias.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&target, &alias).unwrap();

    assert!(matches!(
        resolve_source_file(temp.path(), ".well-known/security.txt").await,
        Err(SourceFileResolutionError::Forbidden)
    ));
}
