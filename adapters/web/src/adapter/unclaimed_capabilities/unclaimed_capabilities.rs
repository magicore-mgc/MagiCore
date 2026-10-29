//! `unclaimed_capabilities.rs` — Unclaimed capability markers / marker capability chưa claim.
use crate::WebAdapter;
// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::SimulatorProvider for WebAdapter {}
impl mgc_types::capabilities::DeviceProvider for WebAdapter {}
impl mgc_types::capabilities::DeployProvider for WebAdapter {}
impl mgc_types::capabilities::ModelRuntimeProvider for WebAdapter {}
