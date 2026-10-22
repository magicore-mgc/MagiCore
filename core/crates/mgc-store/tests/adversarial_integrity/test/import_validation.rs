//! Import Validation tests. / Test import validation.

use crate::adversarial_integrity::support::tmp_store_dir;
use mgc_store::cas::{ContentStore, IntegrityHash, validate_blake3_hex};

pub(crate) fn short_hash_string_is_rejected_not_panicking() {
    // "abc" previously sliced hash[..2] fine but Unicode/short input could
    // panic; now every malformed digest must return an Err.
    // Trước đây chuỗi ngắn/Unicode có thể panic qua hash[..2]; giờ mọi digest
    // sai dạng phải trả Err rõ ràng.
    for bad in [
        "",
        "a",
        "ab",
        "abc",
        "zzzz",
        "😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀",
        "ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789",
        "sha256:abcdef0123456789",
        "blake3-0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000extra",
    ] {
        let res = IntegrityHash::from_hash_str(bad, false);
        assert!(res.is_err(), "malformed digest '{bad}' must be rejected");
        assert!(validate_blake3_hex(bad).is_err());
    }
}

pub(crate) fn valid_hash_string_is_accepted() {
    let good = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    assert!(IntegrityHash::from_hash_str(good, false).is_ok());
    assert!(validate_blake3_hex(good).is_ok());
}

pub(crate) fn import_with_declared_hash_mismatch_is_rejected() {
    let root = tmp_store_dir("declared-mismatch");
    let store = ContentStore::new(root.clone()).unwrap();

    let real = IntegrityHash::from_bytes(b"payload-A", false);
    let wrong = IntegrityHash::from_bytes(b"payload-B", false);

    let res = store.import_bytes_with_hash(b"payload-A", wrong.as_hex(), false);
    assert!(res.is_err(), "declared digest must match uploaded content");
    let _ = real;
}

pub(crate) fn poisoned_import_rejected_even_when_declared_blob_exists() {
    let root = tmp_store_dir("poison-existing");
    let store = ContentStore::new(root.clone()).unwrap();

    // Legitimately store content-B first (its blob now exists in the CAS).
    // Lưu hợp lệ content-B trước (blob của nó giờ có trong CAS).
    let b = IntegrityHash::from_bytes(b"payload-B", false);
    store.import_bytes(b"payload-B").unwrap();
    assert!(b.cas_path(&root).exists());

    // Now try to import content-A while DECLARING digest-B. The cache-hit
    // shortcut must NOT verify blob-B and return Ok — the actual bytes (A)
    // never matched the declared digest (B). This is the R3 adversarial
    // gate.
    // Giờ thử import content-A nhưng KHAI digest-B. Đường tắt cache-hit
    // KHÔNG được verify blob-B rồi trả Ok — bytes thực (A) chưa bao giờ
    // khớp digest khai (B). Đây là cổng đối kháng R3.
    let res = store.import_bytes_with_hash(b"payload-A", b.as_hex(), false);
    assert!(
        res.is_err(),
        "poisoned import must be rejected at the adversarial gate even when the declared blob exists"
    );
}

pub(crate) fn import_with_malformed_declared_hash_is_rejected() {
    let root = tmp_store_dir("malformed-declared");
    let store = ContentStore::new(root.clone()).unwrap();

    let res = store.import_bytes_with_hash(b"payload", "deadbeef", false);
    assert!(
        res.is_err(),
        "malformed declared digest must fail-closed, not panic"
    );
}
