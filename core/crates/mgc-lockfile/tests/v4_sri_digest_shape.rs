//! V4 install readiness requires a decodable full-length digest.
//! Gate không được coi chuỗi alphabet/hex/short digest là SRI dùng được.
use base64::Engine;
use mgc_lockfile::policy::check_install_sri;
#[test]
fn accepts_full_length_canonical_digests() {
    for (algorithm, size) in [("sha256", 32), ("sha512", 64)] {
        let digest = base64::engine::general_purpose::STANDARD.encode(vec![0u8; size]);
        check_install_sri(&format!("{algorithm}-{digest}")).unwrap();
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

#[test]
fn migration_does_not_promote_malformed_digest_to_install_readiness() {
    use mgc_lockfile::{Lockfile, Package};
    for digest in [
        "sha256-AAAA",
        "sha512-test",
        "sha256-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB=",
    ] {
        let mut package = Package::new(
            "fixture".into(),
            "1.0.0".into(),
            "https://registry.example/fixture.tgz".into(),
            digest.into(),
        );
        package.registry = Some("https://registry.example".into());
        let mut lock = Lockfile::new();
        lock.packages.push(package);
        let (document, warnings) = mgc_lockfile::migrate::migrate_v3_to_v4(lock).unwrap();
        assert!(
            !warnings.is_empty(),
            "malformed digest must produce a migration warning"
        );
        assert!(
            document.packages[0]
                .artifact
                .as_ref()
                .unwrap()
                .integrity_sri
                .is_none(),
            "malformed digest must not become install-ready"
        );
    }
}

#[test]
fn migration_normalizes_native_sha256_hex_without_changing_digest() {
    use mgc_lockfile::{Lockfile, Package};
    for raw in ["ab".repeat(32), "AB".repeat(32)] {
        let mut package = Package::new(
            "fixture".into(),
            "1.0.0".into(),
            "https://registry.example/fixture.tgz".into(),
            format!("sha256-{raw}"),
        );
        package.registry = Some("https://registry.example".into());
        let mut lock = Lockfile::new();
        lock.packages.push(package);
        let (document, warnings) = mgc_lockfile::migrate::migrate_v3_to_v4(lock).unwrap();
        assert!(
            warnings.is_empty(),
            "lossless representation conversion must not be reported as hash loss: {warnings:?}"
        );
        let sri = document.packages[0]
            .artifact
            .as_ref()
            .unwrap()
            .integrity_sri
            .as_ref()
            .unwrap();
        check_install_sri(sri).unwrap();
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(sri.strip_prefix("sha256-").unwrap())
            .unwrap();
        assert_eq!(
            bytes,
            vec![0xab; 32],
            "conversion must retain the exact digest bytes"
        );
    }
}

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
