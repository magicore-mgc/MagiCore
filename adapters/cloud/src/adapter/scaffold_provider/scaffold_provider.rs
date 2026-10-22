//! Reports the existing scaffold capability evidence; file generation stays in `crate::scaffold`. — Báo evidence capability scaffold hiện tại; sinh file do `crate::scaffold` quản lý.

use mgc_types::MgResult;
use mgc_types::capabilities::ScaffoldProvider;

use super::super::CloudAdapter;

impl ScaffoldProvider for CloudAdapter {
    /// Evidence: src/scaffold (terraform/cdk/pulumi generators) + the
    /// `mgc create-clo` CLI lane.
    /// Dẫn chứng: src/scaffold (bộ sinh terraform/cdk/pulumi) + lane CLI
    /// `mgc create-clo`.
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}
