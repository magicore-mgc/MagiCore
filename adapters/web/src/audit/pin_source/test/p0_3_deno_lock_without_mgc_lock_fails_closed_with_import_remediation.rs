//! Preserve the existing Web audit workflow contract test.
//! Giữ test hợp đồng workflow audit Web hiện có.

use super::*;

#[test]
fn p0_3_deno_lock_without_mgc_lock_fails_closed_with_import_remediation() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("deno.lock"),
        r#"{"version": "5", "npm": {"lodash@4.17.20": {"integrity": "sha512-x"}}}"#,
    )
    .unwrap();

    let err = run_audit_blocking(dir.path()).unwrap_err();
    assert!(
        err.contains("mgc.lock is missing") && err.contains("mgc import deno"),
        "deno.lock remediation must name `mgc import deno`, got: {err}"
    );
}
