//! Unclaimed Cloud capabilities — capability Cloud chưa claim.

use super::super::CloudAdapter;

// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::OptimizerProvider for CloudAdapter {}
impl mgc_types::capabilities::Materializer for CloudAdapter {}
impl mgc_types::capabilities::SimulatorProvider for CloudAdapter {}
impl mgc_types::capabilities::DeviceProvider for CloudAdapter {}
impl mgc_types::capabilities::ModelRuntimeProvider for CloudAdapter {}
