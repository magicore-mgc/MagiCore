//! Scaffold provider capability implementation.
//! Triển khai capability scaffold provider.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: scaffolds new projects for the core's frameworks
/// (adapter scaffold modules and/or the CLI `mgc create-<core>` lane).
/// Marker + probe: scaffold project mới cho framework của core (module
/// scaffold của adapter và/hoặc lane CLI `mgc create-<core>`).
pub trait ScaffoldProvider: CoreIdent {
    fn probe_scaffold(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "scaffold",
            "this core does not implement scaffolding",
        ))
    }
}
