//! Expose the existing MagiCore-owned optimizer and bench generator.
//! Cung cấp generator optimizer và bench do MagiCore quản lý.

use super::super::adapter::HardwareAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::OptimizerProvider;

impl OptimizerProvider for HardwareAdapter {
    /// Evidence: optimizer/bench packages are materialized by the mgc
    /// generator (`mgc add-hardware <pkg>` — the old add/remove guidance)
    /// and listed below; `mgc optimize`/`bench` run the mgc harness.
    /// Dẫn chứng: package optimizer/bench được materialize bởi generator
    /// mgc (`mgc add-hardware <pkg>`) và liệt kê bên dưới;
    /// `mgc optimize`/`bench` chạy harness của mgc.
    fn probe_optimizer(&self) -> MgResult<()> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
