//! Declare only the capabilities implemented by the hardware adapter.
//! Chỉ khai báo capability mà adapter hardware thực sự triển khai.

use super::super::adapter::HardwareAdapter;
use mgc_types::capabilities::Capability;

impl HardwareAdapter {
    /// Capability manifest (Global Gate 1) — code-reality notes:
    /// - ProjectDetector: `manifest_is_any_mg` (any mgc.toml) — real.
    /// - OptimizerProvider is NOT claimed (PA-1, reviewLoop0902). The
    ///   optimizer/bench framework is ScaffoldProvider evidence — the
    ///   `mgc create-hardware`/`mgc add-hardware` template listing. `mgc optimizer`
    ///   on a hardware project only writes the machine-level profile through the
    ///   generic lane and aborts when no language runtime exists, which is the
    ///   normal shape of a hardware scaffold. Claiming it advertised a dispatch
    ///   the manifest could not back; see `optimizer_provider` for the probe.
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
        Capability::AuditProvider,
    ];
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
