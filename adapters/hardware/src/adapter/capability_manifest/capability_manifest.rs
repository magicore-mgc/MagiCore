//! Declare only the capabilities implemented by the hardware adapter.
//! Chỉ khai báo capability mà adapter hardware thực sự triển khai.

use super::super::adapter::HardwareAdapter;
use mgc_types::capabilities::Capability;

impl HardwareAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `manifest_is_any_mg` (any mgc.toml) — real.
    /// - OptimizerProvider: the optimizer/bench framework — template
    ///   materialization via `mgc add-hardware`/`create-hardware` and the
    ///   optimizer/bench listing below — real (mgc-owned generator).
    /// - AuditProvider: shared polyglot dispatch (sibling manifests get
    ///   real scans) — real.
    ///
    /// Everything registry-shaped is NOT claimed — every previous override
    /// returned an error; `list` only reports materialized templates.
    /// Bảng capability (Global Gate 1) — ghi chú theo code thật.
    pub const CAPABILITIES: &'static [Capability] = &[
        Capability::ProjectDetector,
        // Scaffolding lives in the CLI lane, not in an adapter module; the trait
        // accepts either. See `unclaimed_capabilities` for why this is claimed.
        // (Scaffold nằm ở lane CLI, không phải module adapter; trait chấp nhận cả
        // hai. Xem `unclaimed_capabilities` để biết vì sao claim.)
        Capability::ScaffoldProvider,
        Capability::OptimizerProvider,
        Capability::AuditProvider,
    ];
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
