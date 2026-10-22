//! Core identity, project detection and scaffold capability.
//! Định danh core, nhận diện project và capability scaffold.

use crate::adapter::AppAdapter;
use crate::language::manifest_is_app;
use mgc_types::capabilities::{CoreIdent, ProjectDetector, ScaffoldProvider};
use mgc_types::{Ecosystem, MgResult};
use std::path::Path;

impl CoreIdent for AppAdapter {
    fn core_id(&self) -> &'static str {
        "app"
    }

    fn name(&self) -> &str {
        "app"
    }

    fn ecosystem(&self) -> Ecosystem {
        Ecosystem::App
    }
}

impl ProjectDetector for AppAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        manifest_is_app(project_root)
    }
}

impl ScaffoldProvider for AppAdapter {
    /// Evidence: `mgc create-app` scaffold lane (CLI create commands).
    /// Dẫn chứng: lane scaffold `mgc create-app` (lệnh create của CLI).
    fn probe_scaffold(&self) -> MgResult<()> {
        Ok(())
    }
}
