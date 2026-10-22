// lockfile_v4_signature_tests — signed mutation refusal and schema detection.
// lockfile_v4_signature_tests — kiểm thử signed mutation refusal and schema detection.
use super::*;

#[test]
fn web_writer_v4_refuses_signed_lock_mutation() {
    use mgc_lockfile::v4::SignatureBlock;
    let dir = tempdir_real().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"app\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    // A signed v4 lock on disk must never be rewritten by the ordinary
    // install writer (same guard as the v3 path).
    // (Lock v4 đã ký trên đĩa không bao giờ bị writer install thường ghi đè.)
    let mut doc = mgc_lockfile::canonical::LockfileV4::new("mgc/test");
    // Real signature math: sign the payload digest so the read path
    // reaches the mutation guard instead of failing verification.
    // (Chữ ký thật: ký digest payload để đường đọc tới được guard mutation
    // thay vì fail ở verify.)
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    let signer = key.signer().unwrap();
    doc.metadata.generated_at = "2026-10-07T00:00:00Z".to_string();
    doc.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&doc.payload());
    let digest_bytes = doc
        .metadata
        .lockfile_hash
        .strip_prefix("blake3-")
        .map(|b64| {
            use base64::Engine;
            base64::engine::general_purpose::STANDARD
                .decode(b64)
                .unwrap()
        })
        .unwrap();
    let signature = signer.sign(&digest_bytes);
    doc.metadata.signature = Some(SignatureBlock {
        algorithm: "ed25519".to_string(),
        key_id: key.key_id.clone(),
        public_key: signer.public_key().to_base64(),
        digest: doc.metadata.lockfile_hash.clone(),
        signed_at: "2026-10-07T00:00:00Z".to_string(),
        signature: format!("ed25519-{}", signature.to_base64()),
    });
    let text = mgc_lockfile::canonical::write_v4_document(&doc).unwrap();
    std::fs::write(dir.path().join("mgc.lock"), text).unwrap();

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
    let err = crate::lockfile::write_web_lockfile_with_state(
        dir.path(),
        &graph,
        "https://registry.example",
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("signed"),
        "expected signed-lock refusal, got: {err}"
    );
}

#[test]
fn lock_on_disk_version_dispatch() {
    use crate::lockfile::lock_on_disk_is_v4;
    // Missing lock is not v4 (legacy paths keep their behavior).
    // (Lock thiếu không phải v4 (đường legacy giữ nguyên).)
    let missing = tempdir_real().unwrap();
    assert!(!lock_on_disk_is_v4(missing.path()));

    // A v4 document on disk routes around legacy readers.
    // (Lock v4 trên đĩa đi vòng reader legacy.)
    let dir = tempdir_real().unwrap();
    let doc = mgc_lockfile::canonical::LockfileV4::new("mgc/test");
    let text = mgc_lockfile::canonical::write_v4_document(&doc).unwrap();
    std::fs::write(dir.path().join("mgc.lock"), text).unwrap();
    assert!(lock_on_disk_is_v4(dir.path()));

    // Legacy content stays on the legacy path.
    // (Nội dung legacy ở nguyên đường legacy.)
    std::fs::write(
        dir.path().join("mgc.lock"),
        "version = \"3\"\n[metadata]\ngenerated_at = \"x\"\ngenerator = \"mgc/test\"\nlockfile_hash = \"\"\n",
    )
    .unwrap();
    assert!(!lock_on_disk_is_v4(dir.path()));
}
