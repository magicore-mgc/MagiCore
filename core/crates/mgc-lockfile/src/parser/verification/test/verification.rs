//! Checks local hash error category and diagnostic mapping.
//! Kiểm tra nhóm lỗi hash cục bộ và ánh xạ thông báo.

use super::super::verify_hash;
use crate::LockfileError;

#[test]
fn hash_mismatch_maps_to_tampered_lockfile() {
    let local =
        verify_hash("blake3-expected", "blake3-actual").expect_err("test operation must fail");
    assert!(matches!(
        LockfileError::from(local),
        LockfileError::TamperedLockfile(message)
            if message == "hash mismatch: expected blake3-expected, got blake3-actual"
    ));
}
