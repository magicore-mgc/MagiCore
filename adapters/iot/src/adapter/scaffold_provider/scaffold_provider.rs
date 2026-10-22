//! Reports the current scaffold capability; generation remains in the IoT scaffold module. — Báo capability scaffold hiện có; sinh source vẫn ở module scaffold IoT.

use super::super::IotAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::ScaffoldProvider;
impl ScaffoldProvider for IotAdapter {
    /// Evidence: src/scaffold generators + the `mgc create-iot` CLI lane.
    /// Dẫn chứng: bộ sinh src/scaffold + lane CLI `mgc create-iot`.
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}
