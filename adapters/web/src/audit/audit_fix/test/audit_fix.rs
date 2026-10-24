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

#[test]
fn pinned_range_is_never_widened_to_star() {
    // Regression: the previous implementation replaced the author's range with
    // `*`, which silenced the advisory by deleting every version constraint.
    // Hồi quy: bản cũ thay range của tác giả bằng `*` — "xoá" advisory bằng
    // cách xoá mọi ràng buộc phiên bản.
    let project = tempfile::tempdir().unwrap();
    let original_manifest =
        r#"{"name":"demo","version":"1.0.0","dependencies":{"lodash":"4.17.20"}}"#;
    std::fs::write(project.path().join("package.json"), original_manifest).unwrap();
    let vulnerable = [PackageId::new(
        PackageName::new("lodash").unwrap(),
        Version::parse("4.17.20").unwrap(),
    )];

    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    // The range is left untouched and re-resolved; the refusal happens because
    // the resolved graph still contains the vulnerable release.
    // Range giữ nguyên và được resolve lại; từ chối vì graph vẫn chứa bản dính.
    let pinned = PackageId::new(
        PackageName::new("lodash").unwrap(),
        Version::parse("4.17.20").unwrap(),
    );
    let error = runtime
        .block_on(run_audit_fix(
            project.path(),
            &vulnerable,
            move |_| async move {
                Ok(mgc_types::adapter::ResolvedGraph {
                    packages: vec![mgc_types::adapter::ResolvedPackage {
                        id: pinned,
                        integrity: String::new(),
                        tarball_url: String::new(),
                        deps: Vec::new(),
                        peer_deps: Vec::new(),
                        direct: true,
                        dev: false,
                    }],
                })
            },
        ))
        .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("will not rewrite your constraint"),
        "{error}"
    );
    let after = std::fs::read_to_string(project.path().join("package.json")).unwrap();
    assert!(
        after.contains("4.17.20"),
        "the author's pin must survive: {after}"
    );
    assert!(
        !after.contains("\"*\"") && !after.contains(": \"*\""),
        "audit-fix must never introduce a wildcard range: {after}"
    );
    assert!(
        !project.path().join("mgc.lock").exists(),
        "a refused fix must not write a lockfile"
    );
}
