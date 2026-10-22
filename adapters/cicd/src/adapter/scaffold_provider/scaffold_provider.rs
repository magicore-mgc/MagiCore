//! Expose the existing MagiCore CI/CD scaffold lane.
//! Cung cấp lane scaffold CICD hiện có của MagiCore.

use super::super::adapter::CicdAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::ScaffoldProvider;

impl ScaffoldProvider for CicdAdapter {
    /// Evidence: `mgc create-cicd <provider>` scaffold lane.
    /// Dẫn chứng: lane scaffold `mgc create-cicd <provider>`.
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
