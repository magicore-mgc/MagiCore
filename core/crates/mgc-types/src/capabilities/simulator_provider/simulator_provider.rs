//! Simulator provider capability implementation.
//! Triển khai capability simulator provider.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: runs simulators for the core (phase-1: unclaimed by all).
/// Marker + probe: chạy simulator cho core (phase-1: chưa core nào claim).
pub trait SimulatorProvider: CoreIdent {
    fn probe_simulator(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "simulate",
            "this core has no simulator capability",
        ))
    }
}
