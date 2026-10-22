//! Atomic lock publication and signature-preservation tests.
//! Test publication lock nguyên tử và giữ nguyên chữ ký.

use super::support::*;
#[test]
fn lib_writer_refuses_signed_lock_and_preserves_signature_pair() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    let mut signed = mgc_lockfile::Lockfile::new();
    signed
        .packages
        .push(package("old-lib-dep", "1.0.0", EcosystemTag::Python));
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut signed, &path, &key).unwrap();
    let lock_before = std::fs::read(&path).unwrap();
    let signature_path = path.with_extension("lock.sig");
    let signature_before = std::fs::read(&signature_path).unwrap();

    let result = write_canonical_lock(
        dir.path(),
        EcosystemTag::Python,
        vec![package("new-lib-dep", "2.0.0", EcosystemTag::Python)],
    );

    assert!(result.unwrap_err().to_string().contains("signed mgc.lock"));
    assert_eq!(std::fs::read(&path).unwrap(), lock_before);
    assert_eq!(std::fs::read(&signature_path).unwrap(), signature_before);
    assert_eq!(
        mgc_lockfile::verify_lockfile(&path).unwrap(),
        mgc_lockfile::VerificationStatus::Valid
    );
}

#[test]
fn lib_writer_leaves_an_unchanged_signed_lock_byte_identical() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "lib").unwrap();
    let path = dir.path().join("mgc.lock");
    let mut existing = mgc_lockfile::Lockfile::new();
    let mut existing_package = package("stable-lib-dep", "1.0.0", EcosystemTag::Python);
    existing_package.owner_core = Some("lib".to_string());
    existing.packages.push(existing_package);
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut existing, &path, &key).unwrap();
    let lock_before = std::fs::read(&path).unwrap();
    let signature_path = path.with_extension("lock.sig");
    let signature_before = std::fs::read(&signature_path).unwrap();
    let mut next_package = package("stable-lib-dep", "1.0.0", EcosystemTag::Python);
    next_package.owner_core = Some("lib".to_string());

    write_canonical_lock(dir.path(), EcosystemTag::Python, vec![next_package]).unwrap();

    assert_eq!(std::fs::read(&path).unwrap(), lock_before);
    assert_eq!(std::fs::read(&signature_path).unwrap(), signature_before);
    assert_eq!(
        mgc_lockfile::verify_lockfile(&path).unwrap(),
        mgc_lockfile::VerificationStatus::Valid
    );
}

#[test]
fn lib_lock_writer_records_direct_roots_without_erasing_sibling_ecosystems() {
    let dir = tempfile::tempdir().unwrap();
    mgc_config::project::ProjectConfig::write_core_marker_at(dir.path(), "lib").unwrap();
    let mut lock = Lockfile::new();
    let mut old_python = package("existing-python", "2.0.0", EcosystemTag::Python);
    old_python.owner_core = Some("lib".to_string());
    lock.packages.push(old_python);
    lock.root_dependencies_by_owner.insert(
        "lib".to_string(),
        vec![mgc_lockfile::format_root_pin(
            EcosystemTag::Python,
            "existing-python@2.0.0",
        )],
    );
    std::fs::write(
        dir.path().join("mgc.lock"),
        writer::serialize_lockfile(&lock).unwrap(),
    )
    .unwrap();

    write_canonical_lock_with_roots(
        dir.path(),
        EcosystemTag::Rust,
        vec![package("serde", "1.0.0", EcosystemTag::Rust)],
        vec!["serde@1.0.0".to_string()],
    )
    .unwrap();

    let updated = parser::load_lockfile(&dir.path().join("mgc.lock")).unwrap();
    assert_eq!(
        updated.root_dependencies_by_owner["lib"],
        vec![
            mgc_lockfile::format_root_pin(EcosystemTag::Python, "existing-python@2.0.0"),
            mgc_lockfile::format_root_pin(EcosystemTag::Rust, "serde@1.0.0"),
        ]
    );
}

#[test]
fn atomic_lock_publication_replaces_existing_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mgc.lock");
    std::fs::write(
        &path,
        b"version = \"3\"\n[metadata]\ngenerated_at = \"1970-01-01T00:00:00Z\"\ngenerator = \"fixture\"\nlockfile_hash = \"\"\n",
    )
    .unwrap();

    let replacement = b"version = \"3\"\n[metadata]\ngenerated_at = \"1970-01-01T00:00:01Z\"\ngenerator = \"fixture\"\nlockfile_hash = \"\"\n";
    crate::install::lock_write::atomic_write_canonical_lock(&path, replacement).unwrap();

    assert_eq!(std::fs::read(path).unwrap(), replacement);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1);
}

#[test]
fn native_entry_preserves_digest_across_legacy_and_sri() {
    let mut package = mgc_types::ResolvedPackage {
        id: mgc_types::PackageId::new(
            mgc_types::PackageName::new("fixture").unwrap(),
            mgc_types::Version::parse("1.0.0").unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    for raw in [
        "ab".repeat(32),
        "AB".repeat(32),
        "q6urq6urq6urq6urq6urq6urq6urq6urq6urq6urq6s=".to_string(),
    ] {
        package.integrity = format!("sha256-{raw}");
        assert_eq!(
            crate::install::entry_from_package(&package).sha256,
            "ab".repeat(32)
        );
    }
    for bad in ["sha256-", "sha256-broken", "sha256-AAAA"] {
        package.integrity = bad.to_string();
        assert!(
            !crate::install::entry_from_package(&package)
                .sha256
                .is_empty(),
            "malformed SHA256 must not become missing digest"
        );
    }
}
