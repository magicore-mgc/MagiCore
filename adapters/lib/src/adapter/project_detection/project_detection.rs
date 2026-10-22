use crate::adapter::types::LibAdapter;
use crate::language::manifest_is_lib;
use mgc_types::capabilities::ProjectDetector;
use std::path::Path;

impl ProjectDetector for LibAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        manifest_is_lib(project_root)
    }
}
