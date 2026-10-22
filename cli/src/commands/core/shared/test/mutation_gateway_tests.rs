//! Test signed/tampered lock admission and mutation snapshots.
//! Kiểm tra tiếp nhận lock signed/tampered và snapshot mutation.

use super::*;

#[tokio::test]
#[cfg(feature = "lib")]
async fn mutation_gateway_rejects_signed_lock_before_creating_a_journal() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let lock_path = root.join("mgc.lock");
    let mut signed = mgc_lockfile::Lockfile::new();
    let mut package = mgc_lockfile::Package::new(
        "six".into(),
        "1.17.0".into(),
        "https://files.pythonhosted.org/six.whl".into(),
        "sha256-test".into(),
    );
    package.ecosystem = mgc_lockfile::EcosystemTag::Python;
    package.owner_core = Some("lib".into());
    package.registry = Some("https://pypi.org/simple".into());
    signed.packages.push(package);
    // Signing must start from a real unsigned lock, not the recovery sentinel.
    // Test signing dùng lock thật, không dùng sentinel hỏng của recovery fixture.
    std::fs::write(&lock_path, toml::to_string(&signed).unwrap()).unwrap();
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut signed, &lock_path, &key).unwrap();
    let lock_before = std::fs::read(&lock_path).unwrap();
    let sig_path = lock_path.with_extension("lock.sig");
    let sig_before = std::fs::read(&sig_path).unwrap();
    let adapter = mgc_lib_adapter::adapter_for(root, None, None)
        .unwrap()
        .expect("pyproject must detect a python lib adapter");

    let error = begin_dependency_mutation(&adapter, root, MutationOperation::Add)
        .await
        .unwrap_err();

    assert!(error.to_string().contains("signed mgc.lock"));
    assert_eq!(std::fs::read(lock_path).unwrap(), lock_before);
    assert_eq!(std::fs::read(sig_path).unwrap(), sig_before);
    assert!(
        !root
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists()
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn mutation_gateway_rejects_signed_v4_lock_before_any_staging() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let lock_path = root.join("mgc.lock");
    let mut signed = mgc_lockfile::canonical::LockfileV4::new("mgc/test");
    signed.metadata.lockfile_hash = mgc_lockfile::canonical::payload_digest(&signed.payload());
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    signed.metadata.signature = Some(
        mgc_lockfile::policy::sign_payload_digest(&signed.metadata.lockfile_hash, &key).unwrap(),
    );
    let lock_before = mgc_lockfile::canonical::write_v4_document(&signed).unwrap();
    std::fs::write(&lock_path, &lock_before).unwrap();
    let manifest_before = std::fs::read(root.join("pyproject.toml")).unwrap();
    let adapter = mgc_lib_adapter::adapter_for(root, None, None)
        .unwrap()
        .expect("pyproject must detect a python lib adapter");

    let error = begin_dependency_mutation(&adapter, root, MutationOperation::Add)
        .await
        .unwrap_err();

    assert!(
        error.to_string().contains("signed v4 mgc.lock"),
        "{error:#}"
    );
    assert_eq!(std::fs::read_to_string(&lock_path).unwrap(), lock_before);
    assert_eq!(
        std::fs::read(root.join("pyproject.toml")).unwrap(),
        manifest_before
    );
    assert!(
        !root
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists(),
        "a signed v4 mutation must fail before journal staging"
    );

    let read_only = begin_dependency_mutation(&adapter, root, MutationOperation::Install)
        .await
        .expect("the install gateway must still admit signed lock reuse");
    drop(read_only);

    let downgraded = lock_before.replacen("version = \"4\"", "version = \"3\"", 1);
    std::fs::write(&lock_path, &downgraded).unwrap();
    let error = begin_dependency_mutation(&adapter, root, MutationOperation::Add)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("signed"), "{error:#}");
    assert_eq!(std::fs::read_to_string(&lock_path).unwrap(), downgraded);
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn mutation_gateway_rejects_tampered_v4_before_any_staging() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let lock_path = root.join("mgc.lock");
    let mut document = mgc_lockfile::canonical::LockfileV4::new("mgc/test");
    let bytes = mgc_lockfile::writer::finalize_v4_document(&mut document).unwrap();
    let tampered = bytes.replace("mgc/test", "mgc/evil");
    std::fs::write(&lock_path, &tampered).unwrap();
    let manifest_before = std::fs::read(root.join("pyproject.toml")).unwrap();
    let adapter = mgc_lib_adapter::adapter_for(root, None, None)
        .unwrap()
        .expect("pyproject must detect a python lib adapter");

    let error = begin_dependency_mutation(&adapter, root, MutationOperation::Add)
        .await
        .unwrap_err();

    assert!(error.to_string().contains("digest"), "{error:#}");
    assert_eq!(std::fs::read_to_string(&lock_path).unwrap(), tampered);
    assert_eq!(
        std::fs::read(root.join("pyproject.toml")).unwrap(),
        manifest_before
    );
    assert!(
        !root
            .join(".magicore/journal/dependency-mutation/journal.json")
            .exists(),
        "a tampered v4 mutation must fail before journal staging"
    );
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn install_gateway_allows_an_existing_signed_lock_for_read_only_reuse() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    python_fixture(root);
    let lock_path = root.join("mgc.lock");
    let mut lock = mgc_lockfile::Lockfile::new();
    let mut package = mgc_lockfile::Package::new(
        "six".into(),
        "1.17.0".into(),
        "https://files.pythonhosted.org/six.whl".into(),
        "sha256-test".into(),
    );
    package.ecosystem = mgc_lockfile::EcosystemTag::Python;
    package.owner_core = Some("lib".into());
    package.registry = Some("https://pypi.org/simple".into());
    lock.packages.push(package);
    // Keep the corrupt-input refusal intact while exercising valid lock reuse.
    // Giữ nguyên từ chối input hỏng, kiểm chứng reuse bằng lock hợp lệ.
    std::fs::write(&lock_path, toml::to_string(&lock).unwrap()).unwrap();
    let key = mgc_crypto::keyring::KeyPair::generate().unwrap();
    mgc_lockfile::sign_and_write_lockfile(&mut lock, &lock_path, &key).unwrap();
    let adapter = mgc_lib_adapter::adapter_for(root, None, None)
        .unwrap()
        .expect("pyproject must detect a python lib adapter");

    let guard = begin_dependency_mutation(&adapter, root, MutationOperation::Install)
        .await
        .expect("plain install must be able to read/reuse a valid signed lock");

    assert_eq!(
        mgc_lockfile::verify_lockfile(&lock_path).unwrap(),
        mgc_lockfile::VerificationStatus::Valid
    );
    drop(guard);
}

#[tokio::test]
#[cfg(feature = "lib")]
async fn mutation_snapshot_treats_only_missing_lock_as_absent() {
    let dir = tempfile::tempdir().unwrap();
    python_fixture(dir.path());
    std::fs::remove_file(dir.path().join("mgc.lock")).unwrap();
    let (adapter, write_lock) = test_adapter_and_lock(dir.path());
    let manifest = adapter.parse_manifest(dir.path()).await.unwrap();

    let snapshot =
        MutationSnapshot::capture(&manifest, dir.path(), &write_lock, MutationOperation::Add)
            .unwrap();

    assert!(!snapshot.has_lock_snapshot());
}
