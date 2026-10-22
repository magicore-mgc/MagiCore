// Existing SRI test cases: migration integrity.
// Test SRI hiện có: migration integrity.

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
        let (document, warnings) = mgc_lockfile::migrate::migrate_v3_to_v4(lock).expect("test operation must succeed");
        assert!(
            !warnings.is_empty(),
            "malformed digest must produce a migration warning"
        );
        assert!(
            document.packages[0]
                .artifact
                .as_ref()
                .expect("test operation must succeed")
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
        let (document, warnings) = mgc_lockfile::migrate::migrate_v3_to_v4(lock).expect("test operation must succeed");
        assert!(
            warnings.is_empty(),
            "lossless representation conversion must not be reported as hash loss: {warnings:?}"
        );
        let sri = document.packages[0]
            .artifact
            .as_ref()
            .expect("test operation must succeed")
            .integrity_sri
            .as_ref()
            .expect("test operation must succeed");
        check_install_sri(sri).expect("test operation must succeed");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(sri.strip_prefix("sha256-").expect("test operation must succeed"))
            .expect("test operation must succeed");
        assert_eq!(
            bytes,
            vec![0xab; 32],
            "conversion must retain the exact digest bytes"
        );
    }
}
