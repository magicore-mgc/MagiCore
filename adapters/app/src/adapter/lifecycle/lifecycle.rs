//! Explicit unsupported lifecycle capability.
//! Capability lifecycle unsupported tường minh.

use crate::adapter::AppAdapter;
use mgc_types::MgResult;
use mgc_types::capabilities::LifecycleRunner;

impl LifecycleRunner for AppAdapter {
    fn probe_lifecycle_runner(&self) -> MgResult<()> {
        Err(mgc_types::capabilities::unsupported_capability(
            "app",
            "lifecycle_runner",
            "build/test/run are compiler and runtime operations, not AppAdapter package-lifecycle capabilities",
        ))
    }
}
