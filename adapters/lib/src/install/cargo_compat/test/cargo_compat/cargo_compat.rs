#[test]
fn cargo_lock_writer_preserves_signed_lock_and_skips_identical_bytes() {
    let project = tempfile::tempdir().unwrap();
    let cargo_lock = project.path().join("Cargo.lock");
    let original = b"signed projection\n";
    std::fs::write(&cargo_lock, original).unwrap();
    let signature = mgc_lockfile::parser::signature_path_for(&cargo_lock);
    std::fs::write(signature, b"signature").unwrap();

    crate::install::atomic_write_cargo_lock(project.path(), original).unwrap();
    let error =
        crate::install::atomic_write_cargo_lock(project.path(), b"replacement\n").unwrap_err();

    assert!(error.to_string().contains("signature sidecar"));
    assert_eq!(std::fs::read(cargo_lock).unwrap(), original);
}
