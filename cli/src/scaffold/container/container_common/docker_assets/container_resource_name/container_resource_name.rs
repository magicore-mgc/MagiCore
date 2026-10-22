//! Builds the existing local Docker image name from core and project identity.
//! Tạo tên Docker image local hiện có từ core và tên project.

use crate::scaffold::processor::{Scaffolder, slugify};
use crate::wizard::engine::ScaffoldConfig;

pub(in crate::scaffold::container) fn container_resource_name(
    core: &str,
    config: &ScaffoldConfig,
) -> String {
    format!(
        "magicore-{core}-{}:local",
        slugify(&Scaffolder::config_display_name(config))
    )
}
