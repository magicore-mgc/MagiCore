#![allow(clippy::unwrap_used)]

use super::super::execution_plan::plan_workspaces;
use super::super::workspace_filter::filter_workspaces;
use super::super::workspace_order::order_workspaces;
use clap::Parser;

#[test]
fn filter_order_and_identity_plan_compose_before_execution() {
    let root = tempfile::tempdir().unwrap();
    let app = root.path().join("apps/app");
    let library = root.path().join("packages/library");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(&library).unwrap();
    std::fs::write(
        app.join("package.json"),
        r#"{"name":"@core/app","dependencies":{"@core/library":"workspace:*"}}"#,
    )
    .unwrap();
    std::fs::write(library.join("package.json"), r#"{"name":"@core/library"}"#).unwrap();
    mgc_config::project::ProjectConfig::new("core-app", "web")
        .save(&app)
        .unwrap();
    mgc_config::project::ProjectConfig::new("core-library", "web")
        .save(&library)
        .unwrap();
    let command = crate::Cli::try_parse_from(["mgc", "install-web"])
        .unwrap()
        .command
        .unwrap();

    let selected =
        filter_workspaces(root.path(), &[app.clone(), library.clone()], "@core/*").unwrap();
    let ordered = order_workspaces(selected).unwrap();
    let targets = plan_workspaces(ordered, &command, None).unwrap();

    assert_eq!(targets.len(), 2);
    assert_eq!(targets[0].path, library);
    assert_eq!(targets[1].path, app);
    assert!(
        targets
            .iter()
            .all(|target| target.core.as_deref() == Some("web"))
    );
}

#[test]
fn filter_without_matches_stays_empty_without_ordering_or_dispatch() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("apps/app");
    std::fs::create_dir_all(&workspace).unwrap();

    let selected = filter_workspaces(root.path(), &[workspace], "no-such-package").unwrap();

    assert!(selected.is_empty());
}
