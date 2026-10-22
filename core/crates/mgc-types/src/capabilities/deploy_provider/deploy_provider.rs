//! Deploy provider capability implementation.
//! Triển khai capability deploy provider.

use super::super::{CoreIdent, unsupported_capability};
use crate::error::MgResult;

/// Marker + probe: deploys the project (cloud/CD target).
/// Marker + probe: deploy project (đích cloud/CD).
pub trait DeployProvider: CoreIdent {
    fn probe_deploy(&self) -> MgResult<()> {
        Err(unsupported_capability(
            self.core_id(),
            "deploy",
            "this core has no deploy capability",
        ))
    }
}
