//! Preserve shared fail-closed defaults for unsupported CI/CD operations.
//! Giữ default fail-closed dùng chung cho thao tác CICD chưa hỗ trợ.

use super::super::adapter::CicdAdapter;

// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::DependencyResolver for CicdAdapter {}
impl mgc_types::capabilities::ArtifactFetcher for CicdAdapter {}
impl mgc_types::capabilities::ContentStoreProvider for CicdAdapter {}
impl mgc_types::capabilities::LockfileProvider for CicdAdapter {}
impl mgc_types::capabilities::LifecycleRunner for CicdAdapter {}
impl mgc_types::capabilities::Materializer for CicdAdapter {}
impl mgc_types::capabilities::SimulatorProvider for CicdAdapter {}
impl mgc_types::capabilities::DeviceProvider for CicdAdapter {}
impl mgc_types::capabilities::DeployProvider for CicdAdapter {}
impl mgc_types::capabilities::ModelRuntimeProvider for CicdAdapter {}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
