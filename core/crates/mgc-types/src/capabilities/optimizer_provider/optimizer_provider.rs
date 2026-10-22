//! Optimizer provider capability implementation.
//! Triển khai capability optimizer provider.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: optimizes artifacts/benchmarks for the core.
/// Marker + probe: tối ưu artifact / benchmark cho core.
pub trait OptimizerProvider: CoreIdent {
    fn probe_optimizer(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "optimizer",
            "this core has no optimizer/bench capability",
        ))
    }
}
