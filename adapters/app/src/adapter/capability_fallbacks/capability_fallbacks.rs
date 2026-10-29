//! Unsupported marker capabilities inherit shared defaults.
//! Capability marker unsupported kế thừa default dùng chung.

use crate::adapter::AppAdapter;
use mgc_types::capabilities::{
    ArtifactFetcher, DeployProvider, DeviceProvider, Materializer, ModelRuntimeProvider,
    SimulatorProvider,
};

// These capabilities remain unclaimed by the App core. Empty implementations
// deliberately inherit the shared `Unsupported` probes from `mgc_types`.
// Các capability này chưa được App core claim. Impl rỗng cố ý kế thừa probe
// `Unsupported` dùng chung từ `mgc_types`.
impl ArtifactFetcher for AppAdapter {}
impl Materializer for AppAdapter {}
impl SimulatorProvider for AppAdapter {}
impl DeviceProvider for AppAdapter {}
impl DeployProvider for AppAdapter {}
impl ModelRuntimeProvider for AppAdapter {}
