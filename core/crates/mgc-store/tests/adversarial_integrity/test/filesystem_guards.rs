//! Filesystem Guards tests. / Test filesystem guards.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::IntegrityHash;
use std::fs;

pub(crate) fn typed_hash_fields_are_not_publicly_accessible() {
    // Compile-time proof lives in tests/compile_fail/ (trybuild-style):
    // `IntegrityHash { hash: "x".into(), executable: false }` must NOT
    // compile outside the crate. At runtime we prove the *reflection-free*
    // invariant: every public constructor rejects malformed digests and the
    // accessors are the only way to read the value back.
    // Bằng chứng compile nằm ở tests/compile_fail/ (kiểu trybuild):
    // struct literal ngoài crate KHÔNG compile được. Runtime chứng minh
    // invariant: mọi constructor public từ chối digest sai, accessor là
    // đường đọc duy nhất.
    let h = IntegrityHash::from_bytes(b"data", true);
    assert_eq!(h.as_hex().len(), 64);
    assert!(h.is_executable());

    let parsed =
        IntegrityHash::try_from("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
            .unwrap();
    assert!(!parsed.is_executable());
    assert_eq!(parsed.as_hex().len(), 64);

    // TryFrom must reject everything from_hash_str rejects (same contract).
    // TryFrom phải từ chối mọi thứ from_hash_str từ chối (cùng hợp đồng).
    for bad in [
        "",
        "x",
        "ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789",
    ] {
        assert!(IntegrityHash::try_from(bad).is_err(), "must reject '{bad}'");
    }
}

#[cfg(unix)]
pub(crate) fn cas_root_unreadable_metadata_fails_closed() {
    // P0-3 (adversarial review vòng-9): the OLD code fell through
    // `else if !root.exists()` — `Path::exists()` SWALLOWS the metadata
    // error, so a permission-denied root read as "missing" and the
    // store proceeded to create-and-trust. Contract: `symlink_metadata`
    // failing with anything but NotFound on the ROOT is a hard error
    // naming the cause.
    // (P0-3: code cũ rơi qua `else if !root.exists()` — `Path::exists()`
    // NUỐT lỗi metadata nên root bị từ chối quyền bị đọc thành "mất" và
    // store tiếp tục tạo-rồi-tin. Hợp đồng: `symlink_metadata` lỗi gì
    // KHÁC NotFound trên ROOT là lỗi cứng nêu rõ nguyên nhân.)
    use std::os::unix::fs::PermissionsExt;

    let root = tmp_store_dir("root-metadata-deny");
    let denied = root.join("denied");
    fs::create_dir_all(&denied).unwrap();
    fs::set_permissions(&denied, fs::Permissions::from_mode(0o000)).unwrap();

    let store_root = denied.join("store");
    let err = mgc_store::cas::validate_cas_root(&store_root).unwrap_err();
    let msg = match &err {
        mgc_store::cas::StoreError::Io { msg, .. } => msg.clone(),
        other => panic!("expected Io error, got {other:?}"),
    };
    assert!(
        msg.to_lowercase().contains("metadata"),
        "a permission-denied root must fail with a metadata error, got: {msg}"
    );

    // Restore for tempdir cleanup.
    // (Khôi phục permission để tempdir dọn được.)
    fs::set_permissions(&denied, fs::Permissions::from_mode(0o755)).unwrap();
}
