//! Materializer capability implementation.
//! Triển khai capability materializer.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: materializes the dependency tree layout (node_modules…).
/// Marker + probe: materialize layout cây dependency (node_modules…).
pub trait Materializer: CoreIdent {
    fn probe_materializer(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "materialize",
            "this core does not materialize a dependency tree layout",
        ))
    }
}
