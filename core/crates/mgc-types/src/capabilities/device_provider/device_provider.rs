//! Device provider capability implementation.
//! Triển khai capability device provider.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: flashes/talks to physical devices (phase-1: unclaimed).
/// Marker + probe: flash / nói chuyện với thiết bị thật (phase-1: chưa claim).
pub trait DeviceProvider: CoreIdent {
    fn probe_device(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "device",
            "this core has no device capability",
        ))
    }
}
