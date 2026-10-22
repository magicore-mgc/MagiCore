//! Implements AI core identity and basic capability probes.
//! Triển khai identity core AI và probe capability cơ bản.

use crate::framework::detect_framework;
use mgc_types::capabilities::{
    CoreIdent, LifecycleRunner, OptimizerProvider, ProjectDetector, ScaffoldProvider,
};
use mgc_types::{Ecosystem, MgResult};
use std::path::Path;

use super::super::AiAdapter;

impl CoreIdent for AiAdapter {
    fn core_id(&self) -> &'static str {
        "ai"
    }

    fn name(&self) -> &str {
        "ai"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::Ai
    }
}

impl ProjectDetector for AiAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        detect_framework(project_root).is_some()
    }
}

impl ScaffoldProvider for AiAdapter {
    /// Evidence: `mgc create-ai` scaffold lane (CLI create commands).
    /// Dẫn chứng: lane scaffold `mgc create-ai` (lệnh create của CLI).
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}

impl LifecycleRunner for AiAdapter {
    /// The AI core has CLI-level lifecycle phases. This marker does not
    /// claim dependency ownership; resolver/fetch/lock/store capabilities
    /// are dynamic and appear only for the native PyPI lane.
    /// (Core AI có lifecycle ở CLI; marker này không claim quyền quản lý
    /// dependency. Capability resolver/fetch/lock/store chỉ có ở lane PyPI.)
    fn probe_lifecycle_runner(&self) -> MgResult<()> {
        Ok(())
    }
}

impl OptimizerProvider for AiAdapter {
    /// Evidence: `mgc optimize` dispatches the ai core
    /// (cli/src/commands/optimizer — Ecosystem::Ai branch).
    /// Dẫn chứng: `mgc optimize` điều phối core ai (optimizer CLI).
    fn probe_optimizer(&self) -> MgResult<()> {
        Ok(())
    }
}
