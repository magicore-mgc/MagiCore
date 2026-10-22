//! Keep unsupported hardware capabilities on shared fail-closed defaults.
//! Giữ capability hardware chưa hỗ trợ ở default fail-closed dùng chung.

use super::super::adapter::HardwareAdapter;

// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::ScaffoldProvider for HardwareAdapter {}
impl mgc_types::capabilities::DependencyResolver for HardwareAdapter {}
impl mgc_types::capabilities::ArtifactFetcher for HardwareAdapter {}
impl mgc_types::capabilities::ContentStoreProvider for HardwareAdapter {}
impl mgc_types::capabilities::LockfileProvider for HardwareAdapter {}
impl mgc_types::capabilities::LifecycleRunner for HardwareAdapter {}
impl mgc_types::capabilities::Materializer for HardwareAdapter {}
impl mgc_types::capabilities::SimulatorProvider for HardwareAdapter {}
impl mgc_types::capabilities::DeviceProvider for HardwareAdapter {}
impl mgc_types::capabilities::DeployProvider for HardwareAdapter {}
impl mgc_types::capabilities::ModelRuntimeProvider for HardwareAdapter {}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
