#[path = "allowed_tool_membership/mod.rs"]
mod allowed_tool_membership;
#[path = "device_tool_scope/mod.rs"]
mod device_tool_scope;
#[path = "forbidden_pm_tool/mod.rs"]
mod forbidden_pm_tool;
#[path = "language_runtime_scope/mod.rs"]
mod language_runtime_scope;
#[path = "non_empty_tool_name/mod.rs"]
mod non_empty_tool_name;
#[path = "project_execution_scope/mod.rs"]
mod project_execution_scope;
#[path = "tool_check.rs"]
mod tool_check;

pub(crate) use device_tool_scope::device_control_only_tool;
pub(crate) use tool_check::check_project_binary_scope;
pub use tool_check::{check_tool_with_scope, check_tool_with_scope_compat};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
