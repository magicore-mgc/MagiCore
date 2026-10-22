//! Fail-closed npm protocol slot; the Web adapter owns its native npm path.
//! Slot npm fail-closed; adapter Web vẫn sở hữu đường npm native riêng.

use async_trait::async_trait;
use mgc_types::MgResult;

use super::error::NpmSlotError;
use crate::protocols::{RegistryProtocol, ResolvedEntry};

/// Phase 2 native engine slot — npm registry. The web adapter keeps its own
/// native resolve/fetch/CAS path; this slot only pins the future seam and
/// fails closed on every call.
/// Chỗ cắm engine native Phase 2 — registry npm. Adapter web giữ đường
/// resolve/fetch/CAS native riêng; slot này chỉ ghim điểm ghép tương lai và
/// fail-closed trên mọi lời gọi.
#[derive(Debug, Clone, Copy, Default)]
pub struct NpmProtocol;

#[async_trait]
impl RegistryProtocol for NpmProtocol {
    async fn resolve(&self, _name: &str, _range: &str) -> MgResult<ResolvedEntry> {
        // Fail closed: a stub must never fake a successful resolution.
        // Fail-closed: stub không bao giờ giả một lần resolve thành công.
        Err(NpmSlotError::Resolve.into())
    }

    async fn download(&self, _entry: &ResolvedEntry) -> MgResult<Vec<u8>> {
        // Fail closed — a stub must never fake a successful download.
        // Fail-closed — stub không bao giờ giả một lần download thành công.
        Err(NpmSlotError::Fetch.into())
    }
}

#[cfg(test)]
#[path = "test/npm.rs"]
mod tests;
