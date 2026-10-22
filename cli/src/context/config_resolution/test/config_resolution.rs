//! Verify marker precedence and explicit override rejection at the coordinator.
//! Kiểm tra ưu tiên marker và từ chối override tại bộ điều phối phân giải.

use super::resolve_config;
use std::path::Path;

#[test]
fn project_context_rejects_marker_config_conflict_even_with_core_override() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    mgc_config::project::ProjectConfig::new("identity-route", "web")
        .save(project.path())
        .expect("web project identity must be initialized");
    std::fs::write(project.path().join(".mgc.core"), "ai\n")
        .expect("conflicting marker must be written");

    let root = project.path().to_path_buf();
    for core_override in [None, Some("ai")] {
        let result = resolve_config(&root, Some(&root), core_override);
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("conflicting project identities must be rejected"),
        };
        assert!(
            error.to_string().contains("conflicts with mgc.toml"),
            "identity conflict should be explicit, got: {error}"
        );
    }
}

#[test]
fn explicit_core_override_cannot_reassign_an_owned_project() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    mgc_config::project::ProjectConfig::new("identity-override", "web")
        .save(project.path())
        .expect("web project identity must be initialized");

    let root = project.path().to_path_buf();
    let result = resolve_config(&root, Some(&root), Some("ai"));
    let error = match result {
        Err(error) => error,
        Ok(_) => panic!("explicit core override must not reassign an owned project"),
    };
    assert!(
        error.to_string().contains("core override 'ai' conflicts"),
        "identity conflict should be explicit, got: {error}"
    );

    std::fs::remove_file(root.join(".mgc.core"))
        .expect("test must exercise config-only project identity");
    let result = resolve_config(&root, Some(&root), Some("ai"));
    assert!(
        result
            .expect_err("config identity must reject reassignment")
            .to_string()
            .contains("core override 'ai' conflicts"),
        "config-only identity conflict should be explicit"
    );
}

#[test]
fn explicit_core_without_project_root_builds_a_synthetic_config() {
    let cwd = Path::new("/workspace/standalone");
    let (root, config) = resolve_config(cwd, None, Some("ai"))
        .expect("explicit core should allow commands outside a detected project");

    assert_eq!(root.as_path(), cwd);
    assert_eq!(config.name, "project");
    assert_eq!(config.ecosystem, "ai");
}

#[test]
fn missing_project_root_requires_an_explicit_core() {
    let cwd = Path::new("/workspace/standalone");
    let error = resolve_config(cwd, None, None)
        .expect_err("commands without a project root must require an explicit core");

    assert_eq!(
        error.to_string(),
        "No MagiCore project found. Run `mgc init` to create one, or specify `--core <type>`."
    );
}
