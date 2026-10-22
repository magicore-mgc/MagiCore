//! Delegate hardware project recognition to the shared marker check.
//! Ủy quyền nhận diện project hardware cho kiểm tra marker dùng chung.

use super::super::adapter::HardwareAdapter;
use crate::detection::manifest_is_any_mg;
use mgc_types::capabilities::ProjectDetector;
use std::path::Path;

impl ProjectDetector for HardwareAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        manifest_is_any_mg(project_root)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
