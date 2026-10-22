// lockfile_signed_write_tests — signed lock preservation.
// lockfile_signed_write_tests — kiểm thử signed lock preservation.
use super::*;

#[test]
fn web_writer_refuses_signed_lock_and_preserves_signature_pair() {
    let dir = tempdir_real().unwrap();
    let path = dir.path().join("mgc.lock");
    let mut signed = Lockfile::new();
    let mut package = mgc_lockfile::Package::new(
        "old-web-dep".to_string(),
        "1.0.0".to_string(),
        "https://registry.example/old-web-dep.tgz".to_string(),
        "sha512-old".to_string(),
    );
    package.ecosystem = mgc_lockfile::EcosystemTag::Web;
    package.owner_core = Some("web".to_string());
    package.registry = Some("https://registry.example".to_string());
    signed.packages.push(package);
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut signed, &path, &key).unwrap();
    let lock_before = std::fs::read(&path).unwrap();
    let signature_path = path.with_extension("lock.sig");
    let signature_before = std::fs::read(&signature_path).unwrap();
    let graph = ResolvedGraph::empty();

    let result = crate::lockfile::write_web_lockfile_with_state(
        dir.path(),
        &graph,
        "https://registry.example",
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
fn web_writer_leaves_an_unchanged_signed_lock_byte_identical() {
    let dir = tempdir_real().unwrap();
    let path = dir.path().join("mgc.lock");
    let mut existing = Lockfile::new();
    let mut package = mgc_lockfile::Package::new(
        "stable-web-dep".to_string(),
        "1.0.0".to_string(),
        "https://registry.example/stable-web-dep.tgz".to_string(),
        "sha512-stable".to_string(),
    );
    package.ecosystem = mgc_lockfile::EcosystemTag::Web;
    package.owner_core = Some("web".to_string());
    package.registry = Some("https://registry.example".to_string());
    existing.packages.push(package);
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut existing, &path, &key).unwrap();
    let lock_before = std::fs::read(&path).unwrap();
    let signature_path = path.with_extension("lock.sig");
    let signature_before = std::fs::read(&signature_path).unwrap();
    let graph = ResolvedGraph {
        packages: vec![ResolvedPackage {
            id: PackageId::new(
                PackageName::new("stable-web-dep").unwrap(),
                Version::parse("1.0.0").unwrap(),
            ),
            integrity: "sha512-stable".to_string(),
            tarball_url: "https://registry.example/stable-web-dep.tgz".to_string(),
            deps: vec![],
            peer_deps: vec![],
            direct: true,
            dev: false,
        }],
    };

    crate::lockfile::write_web_lockfile_with_state(dir.path(), &graph, "https://registry.example")
        .unwrap();

    assert_eq!(std::fs::read(&path).unwrap(), lock_before);
    assert_eq!(std::fs::read(&signature_path).unwrap(), signature_before);
    assert_eq!(
        mgc_lockfile::verify_lockfile(&path).unwrap(),
        mgc_lockfile::VerificationStatus::Valid
    );
}
