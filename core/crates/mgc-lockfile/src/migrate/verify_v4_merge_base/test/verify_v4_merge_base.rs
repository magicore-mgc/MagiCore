//! Checks that a v4 document with an empty digest is not trusted.
//! Kiểm tra tài liệu v4 thiếu digest không được tin cậy.

use super::super::verify_v4_merge_base;
use crate::canonical::LockfileV4;

#[test]
fn rejects_a_merge_base_with_unverified_digest_math() {
    let document = LockfileV4::new("mgc/test");
    assert!(verify_v4_merge_base(&document).is_err());
}
