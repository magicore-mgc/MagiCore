//! Implements the current project detector; marker classification remains in the cloud-type owner. — Implement detector hiện tại; phân loại marker do owner cloud-type sở hữu.

use std::path::Path;

use crate::cloud_type::manifest_is_cloud;
use mgc_types::capabilities::ProjectDetector;

use super::super::CloudAdapter;

impl ProjectDetector for CloudAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        manifest_is_cloud(project_root)
    }
}
