use crate::adapter::types::LibAdapter;
use mgc_types::MgResult;

// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::OptimizerProvider for LibAdapter {}
impl mgc_types::capabilities::SimulatorProvider for LibAdapter {}
impl mgc_types::capabilities::DeviceProvider for LibAdapter {}
impl mgc_types::capabilities::DeployProvider for LibAdapter {}
/// Materializer claim — TS-only: the embedded web engine materializes
/// the node_modules layout for real; toolchain-owned languages keep
/// their own materialization (cargo target / venv) — unclaimed there.
/// (Claim materializer — chỉ TS: web engine nhúng materialize layout
/// node_modules thật; ngôn ngữ toolchain-owned giữ materialize của
/// toolchain mình — không claim.)
impl mgc_types::capabilities::Materializer for LibAdapter {
    fn probe_materializer(&self) -> MgResult<()> {
        match &self.web {
            Some(web) => web.probe_materializer(),
            None => Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                "materialize",
                "toolchain-owned lib languages materialize through their own toolchain; mgc owns resolve→lock→fetch→store",
            )),
        }
    }
}

/// LifecycleRunner claim — TS-only: scripts lifecycle (run/build/test)
/// executes through the embedded web engine for TS; other languages run
/// the language toolchain via the mgc-exec allowlist instead — unclaimed.
/// (Claim lifecycle — chỉ TS: scripts lifecycle chạy qua web engine nhúng;
/// ngôn ngữ khác chạy toolchain qua allowlist mgc-exec — không claim.)
impl mgc_types::capabilities::LifecycleRunner for LibAdapter {
    fn probe_lifecycle_runner(&self) -> MgResult<()> {
        match &self.web {
            Some(web) => web.probe_lifecycle_runner(),
            None => Err(mgc_types::capabilities::unsupported_capability(
                "lib",
                "lifecycle",
                "toolchain-owned lib languages run build/test through the language toolchain (mgc-exec allowlist), not a web-style scripts lifecycle",
            )),
        }
    }
}

impl mgc_types::capabilities::ModelRuntimeProvider for LibAdapter {}
