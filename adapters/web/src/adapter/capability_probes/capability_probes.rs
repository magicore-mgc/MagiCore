//! `capability_probes.rs` — Capability evidence probes / probe bằng chứng capability.
use super::super::WebAdapter;
use mgc_types::{
    MgResult,
    capabilities::{LifecycleRunner, Materializer, ScaffoldProvider},
};

impl ScaffoldProvider for WebAdapter {
    /// Evidence: the scaffold lane — `mgc create-web` via the CLI scaffold
    /// engine (cli/src/scaffold + commands/core/create).
    /// Dẫn chứng: lane scaffold — `mgc create-web` qua scaffold engine CLI.
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}

impl Materializer for WebAdapter {
    /// Evidence: the node_modules materializer (install/materialize.rs +
    /// layout.rs — strict symlink virtual store / legacy flat layout).
    /// Dẫn chứng: bộ materialize node_modules (install/materialize.rs +
    /// layout.rs — virtual store symlink strict / layout phẳng legacy).
    fn probe_materializer(&self) -> MgResult<()> {
        Ok(())
    }
}

impl LifecycleRunner for WebAdapter {
    /// Evidence: the lifecycle script runner (lifecycle.rs +
    /// script_policy.rs — preinstall/install/postinstall under policy).
    /// Dẫn chứng: bộ chạy lifecycle script (lifecycle.rs +
    /// script_policy.rs — preinstall/install/postinstall theo policy).
    fn probe_lifecycle_runner(&self) -> MgResult<()> {
        Ok(())
    }
}
