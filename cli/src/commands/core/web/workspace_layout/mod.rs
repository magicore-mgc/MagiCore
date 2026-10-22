//! Shared Web workspace/project-layout detection for dev and install.
//! Nhận diện layout project/workspace Web dùng chung cho dev và install.

mod dev_target;
mod install_targets;
mod manifest_presence;
mod project_mode;
mod types;

pub(in crate::commands::core::web) use dev_target::detect_dev_target;
pub(in crate::commands::core::web) use install_targets::install_targets;
pub(in crate::commands::core::web) use project_mode::{
    detect_project_mode, workspace_backend_dir, workspace_frontend_dir,
};
pub(in crate::commands::core::web) use types::WebProjectMode;
