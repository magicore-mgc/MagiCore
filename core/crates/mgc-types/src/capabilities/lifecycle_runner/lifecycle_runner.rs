//! Lifecycle runner capability implementation.
//! Triển khai capability lifecycle runner.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: runs the core's lifecycle (scripts / toolchain phases).
/// No method moves in phase 1 — lifecycle execution is orchestrator-level
/// (inside install / the CLI per-core lanes) and stays where it is.
/// Marker + probe: chạy lifecycle của core (script / pha toolchain). Phase 1
/// không di chuyển method nào — chạy lifecycle là cấp orchestrator (trong
/// install / lane CLI per-core) và giữ nguyên vị trí.
pub trait LifecycleRunner: CoreIdent {
    fn probe_lifecycle_runner(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "lifecycle",
            "this core has no mgc-run lifecycle phase",
        ))
    }
}
