//! Delegate project recognition to existing CI/CD manifest/provider checks.
//! Ủy quyền nhận diện project cho kiểm tra manifest/provider CICD hiện có.

use super::super::adapter::CicdAdapter;
use crate::provider::manifest_is_cicd;
use mgc_types::capabilities::ProjectDetector;
use std::path::Path;

impl ProjectDetector for CicdAdapter {
    fn can_handle(&self, project_root: &Path) -> bool {
        manifest_is_cicd(project_root)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
