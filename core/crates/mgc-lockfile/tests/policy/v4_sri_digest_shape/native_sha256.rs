// Existing SRI test cases: native sha256.
// Test SRI hiện có: native sha256.

#[test]
fn native_verifier_receives_same_sha256_from_hex_and_sri() {
    let raw = "ab".repeat(32);
    let sri = format!(
        "sha256-{}",
        base64::engine::general_purpose::STANDARD.encode(vec![0xab; 32])
    );
    for value in [
        format!("sha256-{raw}"),
        format!("sha256-{}", raw.to_uppercase()),
        sri,
    ] {
        assert_eq!(mgc_lockfile::policy::native_sha256_hex(&value), raw);
    }
}
#[test]
fn native_invalid_digest_is_not_erased_into_fallback() {
    for raw in [
        "AAAA",
        "not-a-digest",
        "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB=",
    ] {
        assert_eq!(
            mgc_lockfile::policy::native_sha256_hex(&format!("sha256-{raw}")),
            raw
        );
    }
    assert_eq!(
        mgc_lockfile::policy::native_sha256_hex("sha256-"),
        "sha256-"
    );
    assert_eq!(mgc_lockfile::policy::native_sha256_hex("sha512-marker"), "");
}
