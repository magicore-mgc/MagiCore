//! Preserve the existing Web audit workflow contract test.
//! Giữ test hợp đồng workflow audit Web hiện có.

use super::*;

#[test]
fn p0_3_rival_lockfile_without_mgc_lock_fails_closed_with_import_remediation() {
    // CONTRACT: bun.lock/deno.lock tồn tại mà mgc.lock vắng → run_audit
    // FAIL kèm remediation `mgc import` — không fallback audit lockfile
    // đối thủ (đường vận hành chỉ tiêu thụ mgc.lock).
    // (Rival lockfile without mgc.lock → fail with the import remediation;
    // the operational audit lane consumes mgc.lock ONLY.)
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("bun.lock"),
        r#"{"packages": {"lodash@4.17.20": ["lodash@4.17.20", "", {}, "sha512-x"]}}"#,
    )
    .unwrap();

    let err = run_audit_blocking(dir.path()).unwrap_err();
    assert!(
        err.contains("mgc.lock is missing") && err.contains("mgc import"),
        "audit must fail-closed with the import remediation, got: {err}"
    );
}
