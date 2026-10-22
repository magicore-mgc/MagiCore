use super::super::GameAdapter;

// Unclaimed capabilities — empty impls inherit the fail-closed
// Unsupported probes/defaults from mgc_types::capabilities.
// Capability chưa claim — impl rỗng kế thừa probe/default fail-closed
// từ mgc_types::capabilities.
impl mgc_types::capabilities::ArtifactFetcher for GameAdapter {}
impl mgc_types::capabilities::LifecycleRunner for GameAdapter {}
impl mgc_types::capabilities::OptimizerProvider for GameAdapter {}
impl mgc_types::capabilities::Materializer for GameAdapter {}
impl mgc_types::capabilities::SimulatorProvider for GameAdapter {}
impl mgc_types::capabilities::DeviceProvider for GameAdapter {}
impl mgc_types::capabilities::DeployProvider for GameAdapter {}
impl mgc_types::capabilities::ModelRuntimeProvider for GameAdapter {}
