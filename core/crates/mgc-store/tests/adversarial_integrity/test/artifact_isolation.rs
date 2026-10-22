//! Artifact Isolation tests. / Test artifact isolation.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::ContentStore;
use std::fs;

pub(crate) fn cow_or_copy_export_is_independent_from_store() {
    // Whether the export came from clonefile (APFS COW), FICLONE (reflink)
    // or plain copy — mutating the export must NEVER modify the CAS blob.
    // This is the poisoning test re-run against the COW strategy.
    // Dù export đến từ clonefile (APFS COW), FICLONE (reflink) hay copy
    // thường — sửa file export KHÔNG BAO GIỜ làm thay đổi blob CAS.
    // Đây là test đầu độc chạy lại trên chiến lược COW.
    let root = tmp_store_dir("cow-independence");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store.import_bytes(b"cow-payload-immutable").unwrap();

    let export_dir = root.join("project");
    fs::create_dir_all(&export_dir).unwrap();
    let dest = export_dir.join("cow.txt");
    store.export_to(&hash, &dest).unwrap();

    let before = fs::read(hash.cas_path(&root)).unwrap();
    // Mutate the export the way a build tool would.
    // Sửa file export như một build tool vẫn làm.
    fs::write(&dest, b"MUTATED-COW-EXPORT").unwrap();
    let after = fs::read(hash.cas_path(&root)).unwrap();

    assert_eq!(
        before, after,
        "COW export leaked writes into the shared CAS blob"
    );
    assert_eq!(before, b"cow-payload-immutable".to_vec());

    #[cfg(unix)]
    {
        // nlink=1 proves independence for hardlinks; COW clones may share
        // extents with nlink=1, so the WRITE-through test above is the real
        // proof (extent copy-on-write keeps them independent).
        // nlink=1 chứng minh độc lập với hardlink; clone COW có thể chia sẻ
        // extent với nlink=1, nên test ghi-qua phía trên mới là bằng chứng
        // thật (extent copy-on-write giữ chúng độc lập).
        use std::os::unix::fs::MetadataExt;
        let meta = fs::metadata(&dest).unwrap();
        assert_eq!(meta.nlink(), 1, "export must never be a hardlink");
    }
}

pub(crate) fn repeated_exports_stay_correct_with_cow() {
    // Multiple exports of the same blob (dedup scenario) must each be a
    // correct independent copy — COW clone per destination, re-verified.
    // Export nhiều lần cùng blob (kịch bản dedup) — mỗi bản phải là copy
    // độc lập đúng — clone COW cho từng đích, có re-verify.
    let root = tmp_store_dir("cow-repeat");
    let store = ContentStore::new(root.clone()).unwrap();
    let hash = store.import_bytes(b"repeat-export").unwrap();

    for i in 0..3 {
        let dir = root.join(format!("p{i}"));
        let dest = dir.join("file.txt");
        store.export_to(&hash, &dest).unwrap();
        assert_eq!(fs::read(&dest).unwrap(), b"repeat-export");
        // Re-export to the SAME path (idempotent verify path).
        // Export lại CÙNG path (đường verify idempotent).
        store.export_to(&hash, &dest).unwrap();
        assert_eq!(fs::read(&dest).unwrap(), b"repeat-export");
    }
}
