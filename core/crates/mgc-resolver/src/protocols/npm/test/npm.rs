//! Npm slot error classification test.
//! Test phân loại lỗi của slot npm.

use super::super::error::NpmSlotError;

#[test]
fn unsupported_slot_guidance_is_stable() {
    assert_eq!(
        NpmSlotError::Resolve.to_string(),
        "npm native resolver is a Phase 2 slot; the web adapter's native path is unaffected"
    );
    assert_eq!(
        NpmSlotError::Fetch.to_string(),
        "npm native fetch is a Phase 2 slot; the web adapter's native path is unaffected"
    );
}
