//! Native Web processor facade — delegates to framework-family owners.
//! Facade processor Web native — chuyển tiếp tới owner theo họ framework.

use std::path::Path;

use anyhow::Result;

use super::project::write_project as materialize_project;
use super::selection::supports_native_project as selection_supports;

pub struct WebProcessor;

impl WebProcessor {
    pub(crate) fn supports_native_project(sub_type: &str, framework: &str) -> bool {
        selection_supports(sub_type, framework)
    }

    pub(crate) fn write_project(
        target: &Path,
        name: &str,
        sub_type: &str,
        framework: &str,
    ) -> Result<()> {
        materialize_project(target, name, sub_type, framework)
    }
}
