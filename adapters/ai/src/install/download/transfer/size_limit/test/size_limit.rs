//! Tests quota acceptance, rejection and integer overflow.
//! Kiểm tra chấp nhận quota, từ chối và tràn số nguyên.

use crate::install::download::next_model_download_total;

#[test]
fn remote_model_download_quota_is_checked_before_accepting_chunk() {
    assert_eq!(next_model_download_total(4, 6, 10).unwrap(), 10);
    assert!(
        next_model_download_total(4, 7, 10)
            .unwrap_err()
            .to_string()
            .contains("size limit")
    );
    assert!(next_model_download_total(u64::MAX, 1, u64::MAX).is_err());
}
