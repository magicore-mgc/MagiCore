#![allow(clippy::unwrap_used)]

use super::super::plan_workspaces;
use crate::Commands;
use clap::Parser;
use std::path::PathBuf;

fn parse_command(name: &str) -> Commands {
    crate::Cli::try_parse_from(["mgc", name])
        .unwrap()
        .command
        .unwrap()
}

#[test]
fn resolves_core_for_an_unclaimed_workspace_before_dispatch() {
    let workspace = tempfile::tempdir().unwrap();
    let targets = plan_workspaces(
        vec![workspace.path().to_path_buf()],
        &parse_command("build"),
        Some("web"),
    )
    .unwrap();

    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].core.as_deref(), Some("web"));
}

#[test]
fn later_explicit_core_mismatch_rejects_the_whole_plan() {
    let web = tempfile::tempdir().unwrap();
    let app = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::new("web-project", "web")
        .save(web.path())
        .unwrap();
    mgc_config::project::ProjectConfig::new("app-project", "app")
        .save(app.path())
        .unwrap();
    let result = plan_workspaces(
        vec![web.path().to_path_buf(), app.path().to_path_buf()],
        &parse_command("install-web"),
        None,
    );

    assert!(result.is_err());
}

#[test]
fn empty_workspace_plan_is_empty() {
    let targets = plan_workspaces(Vec::<PathBuf>::new(), &parse_command("build"), None).unwrap();

    assert!(targets.is_empty());
}
