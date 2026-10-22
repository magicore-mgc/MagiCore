//! Keeps unsupported AI capabilities explicit and fail-closed.
//! Giữ capability AI chưa hỗ trợ tường minh và fail-closed.

use super::super::AiAdapter;

// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::Materializer for AiAdapter {}
impl mgc_types::capabilities::SimulatorProvider for AiAdapter {}
impl mgc_types::capabilities::DeviceProvider for AiAdapter {}
impl mgc_types::capabilities::DeployProvider for AiAdapter {}
impl mgc_types::capabilities::ModelRuntimeProvider for AiAdapter {}
