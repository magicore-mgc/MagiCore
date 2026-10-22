//! Regression tests for empty Lib install graphs.
//! Test hồi quy graph install Lib rỗng.

use crate::LibLanguage;
use crate::install::run_install;
use mgc_types::ResolvedGraph;
use mgc_types::adapter::InstallOptions;

#[tokio::test]
async fn native_lib_empty_offline_install_does_not_write_lockfiles() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"empty-offline\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    run_install(
        crate::language::LibLanguage::Rust,
        None,
        &ResolvedGraph::empty(),
        project.path(),
        InstallOptions {
            offline: true,
            ..InstallOptions::default()
        },
        None,
        Vec::new(),
    )
    .await
    .expect("empty offline install has nothing to fetch or materialize");
    assert!(
        !project.path().join("mgc.lock").exists(),
        "offline install must not publish a lock"
    );
    assert!(!project.path().join("Cargo.lock").exists());
}

#[tokio::test]
async fn native_lib_empty_offline_graph_refuses_declared_dependencies() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"missing-graph\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    let error = run_install(
        crate::language::LibLanguage::Rust,
        None,
        &ResolvedGraph::empty(),
        project.path(),
        InstallOptions {
            offline: true,
            ..InstallOptions::default()
        },
        None,
        Vec::new(),
    )
    .await
    .expect_err("an empty graph must not hide declared dependencies");

    assert!(
        error
            .to_string()
            .contains("MGC resolved an empty Rust graph")
    );
    assert!(!project.path().join("mgc.lock").exists());
    assert!(!project.path().join("Cargo.lock").exists());
}

#[tokio::test]
async fn native_lib_empty_online_rust_install_publishes_both_lockfiles() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"empty-online\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();

    run_install(
        LibLanguage::Rust,
        None,
        &ResolvedGraph::empty(),
        project.path(),
        InstallOptions::default(),
        None,
        Vec::new(),
    )
    .await
    .expect("empty online install publishes selected locks without registry access");

    assert!(project.path().join("mgc.lock").is_file());
    assert!(project.path().join("Cargo.lock").is_file());
}
