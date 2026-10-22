//! Local audit-fix no-op and fail-closed owner tests.
//! Test tại chỗ cho no-op và fail-closed của audit-fix.

#![allow(clippy::unwrap_used)]

use super::super::run_audit_fix;
use mgc_types::{PackageId, PackageName, Version};

#[test]
fn empty_vulnerability_set_does_not_read_or_resolve() {
    let project = tempfile::tempdir().unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    let updated = runtime
        .block_on(run_audit_fix(project.path(), &[], |_| async {
            Err(mgc_types::MgError::Other(
                "resolver must not run for an empty finding set".to_owned(),
            ))
        }))
        .unwrap();

    assert_eq!(updated, 0);
    assert!(!project.path().join("package.json").exists());
    assert!(!project.path().join("mgc.lock").exists());
}

#[test]
fn missing_manifest_match_fails_before_resolver_or_writes() {
    let project = tempfile::tempdir().unwrap();
    let original_manifest =
        r#"{"name":"demo","version":"1.0.0","dependencies":{"react":"^18.2.0"}}"#;
    std::fs::write(project.path().join("package.json"), original_manifest).unwrap();
    let vulnerable = [PackageId::new(
        PackageName::new("not-installed").unwrap(),
        Version::parse("1.0.0").unwrap(),
    )];
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    let error = runtime
        .block_on(run_audit_fix(project.path(), &vulnerable, |_| async {
            Err(mgc_types::MgError::Other(
                "resolver must not run without a matching manifest dependency".to_owned(),
            ))
        }))
        .unwrap_err();

    assert!(error.to_string().contains("found no matching dependency"));
    assert_eq!(
        std::fs::read_to_string(project.path().join("package.json")).unwrap(),
        original_manifest
    );
    assert!(!project.path().join("mgc.lock").exists());
}
