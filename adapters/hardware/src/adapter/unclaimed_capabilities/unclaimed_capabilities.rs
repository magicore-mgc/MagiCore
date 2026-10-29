//! Keep unsupported hardware capabilities on shared fail-closed defaults.
//! Giữ capability hardware chưa hỗ trợ ở default fail-closed dùng chung.

use super::super::adapter::HardwareAdapter;

// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
//
// `ScaffoldProvider` is deliberately NOT in this list: the CLI lane
// `mgc create-hardware <optimizer|bench>` scaffolds this core, and the trait
// documents that lane as valid evidence. Keeping it here made the manifest say
// "Hardware cannot scaffold" while it demonstrably could — and `mgc capabilities`
// is what a caller reads before deciding to write their own scaffolder.
// (`ScaffoldProvider` cố ý KHÔNG ở đây: lane CLI `mgc create-hardware` scaffold
// core này và doc trait chấp nhận lane đó là dẫn chứng. Giữ nó ở đây khiến
// manifest nói "Hardware không scaffold được" trong khi thực tế làm được.)
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
