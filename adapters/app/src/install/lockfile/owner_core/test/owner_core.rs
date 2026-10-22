//! Lock owner core identity tests / Test định danh core sở hữu lock.
#![allow(clippy::unwrap_used)]

use super::super::app_lock_owner_core;
use mgc_config::project::ProjectConfig;

#[test]
fn explicit_core_marker_takes_precedence_for_lock_ownership() {
    let project = tempfile::tempdir().unwrap();
    ProjectConfig::write_core_marker_at(project.path(), "ai").unwrap();

    assert_eq!(app_lock_owner_core(project.path()).unwrap(), "ai");
}

#[test]
fn project_config_cloud_alias_resolves_to_canonical_lock_owner() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = \"sample\"\necosystem = \"cloud\"\n",
    )
    .unwrap();

    assert_eq!(app_lock_owner_core(project.path()).unwrap(), "clo");
}

#[test]
fn missing_project_identity_keeps_legacy_app_lock_owner() {
    let project = tempfile::tempdir().unwrap();

    assert_eq!(app_lock_owner_core(project.path()).unwrap(), "app");
}
