//! Typed error categories for the intentionally unsupported npm protocol slot.
//! Lỗi có kiểu cho slot protocol npm được đánh dấu unsupported.

use mgc_types::MgError;

#[derive(Debug, Clone, Copy, thiserror::Error)]
pub(super) enum NpmSlotError {
    #[error("npm native resolver is a Phase 2 slot; the web adapter's native path is unaffected")]
    Resolve,
    #[error("npm native fetch is a Phase 2 slot; the web adapter's native path is unaffected")]
    Fetch,
}

impl From<NpmSlotError> for MgError {
    fn from(error: NpmSlotError) -> Self {
        let (capability, guidance) = match error {
            NpmSlotError::Resolve => ("resolve", NpmSlotError::Resolve.to_string()),
            NpmSlotError::Fetch => ("fetch", NpmSlotError::Fetch.to_string()),
        };
        Self::Unsupported {
            core: "web",
            capability,
            guidance,
        }
    }
}
