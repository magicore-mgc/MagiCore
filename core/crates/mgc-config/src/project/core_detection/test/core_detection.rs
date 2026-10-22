//! Core-detection precedence tests.
//! Test thứ tự ưu tiên nhận diện core.
use super::super::super::ProjectConfig;

#[test]
fn explicit_manifest_declaration_precedes_inferred_signature() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("package.json"),
        r#"{"magicore":{"core":"ai"},"dependencies":{"react":"19"}}"#,
    )
    .expect("package manifest should be written");

    let detected = ProjectConfig::detect_core(project.path())
        .expect("explicit declaration should win over inference");

    assert_eq!(detected.as_deref(), Some("ai"));
}

#[test]
fn configured_project_core_precedes_manifest_declaration() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name='sample'\necosystem='game'\n",
    )
    .expect("project config should be written");
    std::fs::write(
        project.path().join("package.json"),
        r#"{"magicore":{"core":"ai"}}"#,
    )
    .expect("package manifest should be written");

    let detected = ProjectConfig::detect_core(project.path())
        .expect("configured identity should win over declaration");

    assert_eq!(detected.as_deref(), Some("game"));
}

#[test]
fn conflicting_signatures_remain_fail_closed() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("package.json"),
        r#"{"dependencies":{"react":"19"}}"#,
    )
    .expect("package manifest should be written");
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname='sample'\nversion='0.1.0'\n[dependencies]\nbevy='0.15'\n",
    )
    .expect("Cargo manifest should be written");

    let error = ProjectConfig::detect_core(project.path())
        .expect_err("conflicting signatures should be rejected");

    assert!(error.to_string().contains("Ambiguous project core"));
    assert!(error.to_string().contains("game, web"));
}
