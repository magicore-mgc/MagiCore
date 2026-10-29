//! Keeps empty marker impls so shared capability defaults remain fail-closed and IoT does not claim unimplemented behavior. — Giữ impl marker rỗng để default capability dùng chung fail-closed và IoT không claim hành vi chưa implement.

use super::super::IotAdapter;
// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::ArtifactFetcher for IotAdapter {}
impl mgc_types::capabilities::LifecycleRunner for IotAdapter {}
impl mgc_types::capabilities::Materializer for IotAdapter {}
impl mgc_types::capabilities::SimulatorProvider for IotAdapter {}
impl mgc_types::capabilities::DeviceProvider for IotAdapter {}
impl mgc_types::capabilities::DeployProvider for IotAdapter {}
impl mgc_types::capabilities::ModelRuntimeProvider for IotAdapter {}
