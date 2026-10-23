//! Tests for the release-manifest signing command.
//! Test cho lệnh ký manifest phát hành.
use super::*;

// Per-test hermetic tempdir (TempDir nonce, auto-cleaned): the old
// PID-shared dir raced when tests ran in parallel threads of one
// process (one test's remove_dir_all wiped another's manifest mid-run
// → flaky CI red). Never share temp state between tests.
// (Tempdir riêng mỗi test — dir chung theo PID race khi chạy song song.)
fn tmp() -> tempfile::TempDir {
    tempfile::TempDir::new().expect("test tempdir")
}

#[test]
fn sign_is_deterministic_and_self_verified() {
    let dir = tmp();
    let manifest = dir.path().join("manifest.json");
    std::fs::write(&manifest, b"{\"version\":\"9.9.9\"}").unwrap();
    let key = "0".repeat(64);
    run(Some(manifest.display().to_string()), Some(key.to_string())).unwrap();
    let sig1 = std::fs::read_to_string(dir.path().join("manifest.json.sig")).unwrap();
    run(Some(manifest.display().to_string()), Some(key.to_string())).unwrap();
    let sig2 = std::fs::read_to_string(dir.path().join("manifest.json.sig")).unwrap();
    assert_eq!(sig1, sig2, "Ed25519 signing is deterministic");
    assert_eq!(sig1.len(), 128 + 1, "64-byte hex + newline");
}

#[test]
fn bad_key_shapes_fail_closed() {
    let dir = tmp();
    let manifest = dir.path().join("manifest.json");
    std::fs::write(&manifest, b"{}").unwrap();
    assert!(run(Some(manifest.display().to_string()), Some("zz".to_string())).is_err());
    assert!(
        run(
            Some(manifest.display().to_string()),
            Some("abcd".to_string())
        )
        .is_err()
    );
    assert!(
        run(
            Some("/no/such/manifest.json".to_string()),
            Some("00".repeat(32))
        )
        .is_err()
    );
}

#[test]
fn missing_key_without_gate_warns_ok() {
    // No env touched: explicit None + require=false exercises the
    // transitional path hermetically.
    // (Không chạm env: None tường minh + require=false.)
    let dir = tmp();
    let manifest = dir.path().join("manifest.json");
    std::fs::write(&manifest, b"{}").unwrap();
    let out = run_with_key(&manifest.display().to_string(), None, false);
    assert!(out.is_ok(), "transitional unsigned path warns but exits 0");
    assert!(
        !dir.path().join("manifest.json.sig").exists(),
        "no .sig file on the unsigned path"
    );
    assert!(
        run_with_key(&manifest.display().to_string(), None, true).is_err(),
        "require-signed without a key must fail"
    );
}
