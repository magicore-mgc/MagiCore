//! Expose the optimizer/bench contract honestly for the hardware core.
//! Trình bày trung thực hợp đồng optimizer/bench cho core hardware.

use super::super::adapter::HardwareAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::OptimizerProvider;

impl OptimizerProvider for HardwareAdapter {
    /// Answer honestly: `hardware` does NOT provide the optimizer capability.
    ///
    /// PA-1 (reviewLoop0902): this probe used to return `Ok(())`, so
    /// `mgc capabilities` advertised `OptimizerProvider` for hardware while
    /// `mgc optimizer` on a hardware project only wrote the two machine-level
    /// files (`.mgc-optimizer/profile.json`, `gpu.env`) via the generic lane —
    /// and aborted whenever no language runtime was detected, which is the
    /// normal case for a `mgc create-hardware` scaffold (no package.json, no
    /// pyproject.toml, no Cargo.toml).
    ///
    /// The capability belongs to the CORE, not to the template listing: the
    /// optimizer/bench templates this core scaffolds are `ScaffoldProvider`
    /// evidence, never optimizer evidence.
    /// (PA-1: probe cũ trả `Ok(())` nên `mgc capabilities` quảng cáo
    /// `OptimizerProvider` trong khi `mgc optimizer` chỉ ghi hai file mức máy
    /// qua lane chung và abort khi không có runtime — chính là trạng thái bình
    /// thường của scaffold `mgc create-hardware`. Template optimizer/bench là dẫn
    /// chứng của `ScaffoldProvider`, không phải của optimizer.)
    fn probe_optimizer(&self) -> MgResult<()> {
        Err(mgc_types::capabilities::unsupported_capability(
            "hardware",
            "optimizer",
            "v1.2.0 hardware is the optimizer/bench template listing only; \
             `mgc optimizer` applies the machine-level profile through the \
             generic lane and nothing hardware-specific",
        ))
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
