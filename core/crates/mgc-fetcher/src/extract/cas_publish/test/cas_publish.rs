//! Covers one complete import/export publication without DB claims.
//! Kiểm tra một lượt import/export hoàn chỉnh không có DB claim.

#[test]
fn publishes_staged_file_to_destination() {
    let temp = tempfile::tempdir().unwrap();
    let base = temp.path().canonicalize().unwrap();
    let root = base.join("package");
    std::fs::create_dir(&root).unwrap();
    let store = mgc_store::ContentStore::new(base.join("store/cas")).unwrap();
    let archive = super::super::super::test_support::gzip_tar_file("package/file", b"payload");
    let staged = super::super::super::cas_spool::spool_archive(archive.as_slice(), &root, false)
        .unwrap()
        .unwrap();
    super::super::super::cas_tree::prepare_destination(&root, &staged).unwrap();
    super::super::publish(
        staged,
        &store,
        None,
        super::super::super::cas::ExtractionDurability::Durable,
    )
    .unwrap();
    assert_eq!(
        std::fs::read(root.join("package/file")).unwrap(),
        b"payload"
    );
}
