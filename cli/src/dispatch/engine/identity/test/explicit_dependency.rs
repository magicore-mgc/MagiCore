use super::*;

#[test]
fn explicit_core_dependency_command_must_match_workspace_identity_and_global_selector() {
    let workspace = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::new("owned-web", "web")
        .save(workspace.path())
        .unwrap();

    assert!(validate_explicit_dependency_core(workspace.path(), "web", None).is_ok());
    assert!(validate_explicit_dependency_core(workspace.path(), "web", Some("web")).is_ok());

    let owner_error = validate_explicit_dependency_core(workspace.path(), "app", None).unwrap_err();
    assert!(owner_error.to_string().contains("belongs to core 'web'"));

    let selector_error =
        validate_explicit_dependency_core(workspace.path(), "web", Some("app")).unwrap_err();
    assert!(
        selector_error
            .to_string()
            .contains("conflicts with the explicit 'web'")
    );
}

#[test]
fn explicit_core_dependency_command_fails_when_workspace_identity_is_missing() {
    let workspace = tempfile::tempdir().unwrap();

    let error = validate_explicit_dependency_core(workspace.path(), "web", None).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("cannot prove project core identity")
    );
}
