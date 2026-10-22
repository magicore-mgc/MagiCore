//! Threshold and advice must not point to an invalid/global prune command.
//! Ngưỡng và hướng xử lý không được trỏ tới lệnh prune sai/global.

use super::super::{LOW_DISK_WARNING_THRESHOLD_BYTES, advice};

#[test]
fn unknown_and_at_threshold_space_do_not_warn() {
    let store = std::path::Path::new("/configured/mgc/store/v3");
    let volume = std::path::Path::new("/configured/mgc");
    assert!(advice(0, store, volume).is_none());
    assert!(advice(LOW_DISK_WARNING_THRESHOLD_BYTES, store, volume).is_none());
}

#[test]
fn low_space_advice_names_configured_store_and_does_not_claim_prune_frees_it() {
    let store = std::path::Path::new("/configured/mgc/store/v3");
    let volume = std::path::Path::new("/configured/mgc");
    let advice = advice(LOW_DISK_WARNING_THRESHOLD_BYTES - 1, store, volume).unwrap();

    assert!(advice.message.contains("/configured/mgc/store/v3"));
    assert!(advice.message.contains("/configured/mgc'"));
    assert!(advice.suggested_action.contains("/configured/mgc/store/v3"));
    assert!(!advice.suggested_action.contains("mgc store prune"));
}
