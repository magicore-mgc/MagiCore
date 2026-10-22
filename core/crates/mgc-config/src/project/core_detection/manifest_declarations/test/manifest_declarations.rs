//! Cross-manifest declaration policy tests.
//! Test policy gom khai báo nhiều manifest.
use super::super::super::error::{CoreDetectionError, RecoveryAction};
use super::super::detect_manifest_core;

#[test]
fn absent_explicit_declarations_return_none() {
    let project = tempfile::tempdir().expect("temporary project should be created");

    assert_eq!(detect_manifest_core(project.path()).unwrap(), None);
}

#[test]
fn conflicting_manifests_fail_closed_in_sorted_order() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("package.json"),
        r#"{"magicore":{"core":"web"}}"#,
    )
    .expect("package manifest should be written");
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[tool.magicore]\ncore='ai'\n",
    )
    .expect("Python manifest should be written");

    let error = detect_manifest_core(project.path()).expect_err("conflict should be rejected");

    assert!(matches!(
        &error,
        CoreDetectionError::ConflictingManifestDeclarations { .. }
    ));
    assert_eq!(
        error.to_string(),
        "conflicting MagiCore core declarations in project manifests: ai, web. Set one authoritative '.mgc.core' marker or mgc.toml ecosystem."
    );
    assert_eq!(
        error.recovery_action(),
        RecoveryAction::RepairProjectIdentity
    );
}

#[test]
fn unknown_manifest_core_is_rejected() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("package.json"),
        r#"{"magicore":{"core":"monolith"}}"#,
    )
    .expect("package manifest should be written");

    let error = detect_manifest_core(project.path()).expect_err("unknown core should be rejected");

    assert!(matches!(
        &error,
        CoreDetectionError::UnknownDeclaredCore { .. }
    ));
    assert!(
        error
            .to_string()
            .contains("manifest declares unknown MagiCore core 'monolith'")
    );
    assert_eq!(
        error.recovery_action(),
        RecoveryAction::RepairProjectIdentity
    );
}
