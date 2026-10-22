//! Model runtime capability implementation.
//! Triển khai capability runtime model.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: runs AI model runtimes (phase-1: unclaimed; the model
/// download/runtime machinery lives in the ai core's own modules).
/// Marker + probe: chạy runtime model AI (phase-1: chưa claim; cơ chế
/// download/runtime model nằm ở module riêng của core ai).
pub trait ModelRuntimeProvider: CoreIdent {
    fn probe_model_runtime(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "model-runtime",
            "this core has no model runtime capability",
        ))
    }
}
