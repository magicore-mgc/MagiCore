//! Script, hook, tool, and runtime policy diagnostics.
//! Lỗi script, hook, tool và runtime policy.

mod error;

pub use error::{
    dir_missing, forbidden_pm_script, hook_failed, invalid_spawn_profile, project_tool_failed,
    publish_lifecycle_opt_in_required, publish_lifecycle_policy_invalid,
    runtime_dangerous_flag_rejected, runtime_dangerous_permission_rejected, script_not_found,
    tool_not_installed_project, unsupported_script,
};
