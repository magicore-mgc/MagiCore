//! Capability error behavior tests — test hành vi lỗi capability.
//! Locks the exact unknown-core display contract.
//! Khóa contract hiển thị chính xác của unknown-core.

use super::super::CapabilitiesError;

#[test]
fn unknown_core_keeps_legacy_display_text() {
    let error = CapabilitiesError::UnknownCore {
        core: "custom-core".to_owned(),
    };
    assert_eq!(error.to_string(), "Unknown core 'custom-core'");
}
