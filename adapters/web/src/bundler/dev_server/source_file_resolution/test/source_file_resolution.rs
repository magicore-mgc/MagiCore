#![allow(clippy::unwrap_used)]
//! Source-file resolution security and compatibility tests.
//! Test bảo mật và tương thích resolve file source.

use super::super::super::SourceFileResolutionError;
use super::super::super::{resolve_project_file, resolve_source_file};

#[tokio::test]
async fn project_files_resolve_to_canonical_paths() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("src/main.ts");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "export const ready = true;").unwrap();

    let resolved = resolve_source_file(temp.path(), "src/main.ts")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resolved, file.canonicalize().unwrap());
}

#[tokio::test]
async fn public_files_remain_available_as_fallback() {
    let temp = tempfile::tempdir().unwrap();
    let file = temp.path().join("public/logo.svg");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(&file, "<svg/>").unwrap();

    let resolved = resolve_source_file(temp.path(), "logo.svg")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resolved, file.canonicalize().unwrap());
}

#[tokio::test]
async fn project_only_resolution_does_not_use_public_fallback() {
    let temp = tempfile::tempdir().unwrap();
    let public_index = temp.path().join("public/index.html");
    std::fs::create_dir_all(public_index.parent().unwrap()).unwrap();
    std::fs::write(&public_index, "public fallback candidate").unwrap();

    assert!(
        resolve_project_file(temp.path(), "index.html")
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn missing_files_and_directories_are_not_resolved() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join("public/assets")).unwrap();

    assert!(
        resolve_source_file(temp.path(), "missing.txt")
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        resolve_source_file(temp.path(), "public/assets")
            .await
            .unwrap()
            .is_none()
    );
}

#[tokio::test]
async fn hidden_project_files_are_forbidden() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::write(temp.path().join(".env"), "secret=hidden").unwrap();
    std::fs::create_dir_all(temp.path().join(".git")).unwrap();
    std::fs::write(temp.path().join(".git/config"), "secret=hidden").unwrap();

    for path in [".env", ".git/config", "src/.private.ts"] {
        assert!(
            matches!(
                resolve_source_file(temp.path(), path).await,
                Err(SourceFileResolutionError::Forbidden)
            ),
            "hidden request path {path:?} must be forbidden"
        );
    }
}

#[tokio::test]
async fn well_known_assets_are_only_resolved_from_public() {
    let temp = tempfile::tempdir().unwrap();
    let public_file = temp.path().join("public/.well-known/security.txt");
    std::fs::create_dir_all(public_file.parent().unwrap()).unwrap();
    std::fs::write(&public_file, "contact=security@example.invalid").unwrap();
    let root_file = temp.path().join(".well-known/security.txt");
    std::fs::create_dir_all(root_file.parent().unwrap()).unwrap();
    std::fs::write(&root_file, "private-root-copy").unwrap();

    let resolved = resolve_source_file(temp.path(), ".well-known/security.txt")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(resolved, public_file.canonicalize().unwrap());
    assert!(matches!(
        resolve_source_file(temp.path(), ".well-known/.private").await,
        Err(SourceFileResolutionError::Forbidden)
    ));
}
