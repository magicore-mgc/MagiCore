// Existing SRI test cases: sri shape.
// Test SRI hiện có: sri shape.

#[test]
fn accepts_full_length_canonical_digests() {
    for (algorithm, size) in [("sha256", 32), ("sha512", 64)] {
        let digest = base64::engine::general_purpose::STANDARD.encode(vec![0u8; size]);
        check_install_sri(&format!("{algorithm}-{digest}")).expect("test operation must succeed");
    }
}
#[test]
fn rejects_wrong_digest_lengths_and_hex_as_sri() {
    for size in [1, 31, 33, 48, 63, 65] {
        let digest = base64::engine::general_purpose::STANDARD.encode(vec![0u8; size]);
        for algorithm in ["sha256", "sha512"] {
            assert!(
                check_install_sri(&format!("{algorithm}-{digest}")).is_err(),
                "wrong digest length accepted: {algorithm}/{size}"
            );
        }
    }
    assert!(
        check_install_sri(&format!("sha256-{}", "a".repeat(64))).is_err(),
        "hex payload is not a 32-byte base64 digest"
    );
}
#[test]
fn rejects_invalid_base64_and_noncanonical_padding_bits() {
    for sri in [
        "sha256-AAAA=",
        "sha256-====",
        "sha512-test",
        "sha256-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB=",
    ] {
        assert!(
            check_install_sri(sri).is_err(),
            "invalid base64 accepted: {sri}"
        );
    }
}

