//! Tests the native outdated orchestration's empty-manifest path.
//! Kiểm tra nhánh manifest rỗng của orchestration outdated native.

#![allow(clippy::unwrap_used)]

use super::super::super::query_outdated_native;
use crate::context::ProjectContext;

#[tokio::test]
async fn empty_native_manifest_returns_without_a_lock_or_registry_query() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"outdated-empty-native\"\necosystem = \"lib\"\n[lib]\nlanguage = \"rust\"\n",
    )
    .unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"outdated-empty-native\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
    )
    .unwrap();
    let root = project.path().to_path_buf();
    let context = ProjectContext::load_at(&root, Some(&root), None).unwrap();

    let outdated = query_outdated_native(&context).await.unwrap();

    assert!(outdated.is_empty());
    assert!(!root.join("mgc.lock").exists());
}
