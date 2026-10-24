//! Allowlist API facade. Policy logic lives in focused responsibility owners.
//! Facade API allowlist; logic policy nằm trong các owner theo trách nhiệm.

#[path = "allowlist.rs"]
mod allowlist;
#[path = "catalog/mod.rs"]
mod catalog;
#[path = "compat_runner/mod.rs"]
mod compat_runner;
#[path = "env_assignment/mod.rs"]
mod env_assignment;
#[path = "forbidden_script/mod.rs"]
mod forbidden_script;
#[path = "invocation/mod.rs"]
mod invocation;
#[path = "project_local_executable/mod.rs"]
mod project_local_executable;
#[path = "react_native_subdir/mod.rs"]
mod react_native_subdir;
#[path = "scope/mod.rs"]
mod scope;
#[path = "scoped_check/mod.rs"]
mod scoped_check;
#[path = "script_chain/mod.rs"]
mod script_chain;
#[path = "script_invocation/mod.rs"]
mod script_invocation;
#[path = "shell_control/mod.rs"]
mod shell_control;
#[path = "simple_script/mod.rs"]
mod simple_script;
#[path = "simple_words/mod.rs"]
mod simple_words;
#[path = "tool_check/mod.rs"]
mod tool_check;

pub use allowlist::{check_tool, is_react_native_subdir};
pub use catalog::{ALLOWED_TOOLS, FORBIDDEN_TOOLS, JS_PACKAGE_MANAGERS, QUALIFIED_COMPAT_RUNNERS};
pub use compat_runner::{compat_runner_allows, parse_compat_runner};
pub use forbidden_script::{find_forbidden_tool_in_script, reject_forbidden_pm_script};
pub use invocation::ScriptInvocation;
pub use scope::{ExecutionScope, ScopeConstraints};
pub use scoped_check::check_tool_scoped;
pub use script_chain::parse_script_chain;
pub use script_invocation::parse_script_invocation;
pub use shell_control::reject_shell_control;
pub use simple_script::parse_simple_script;
pub(crate) use tool_check::{check_project_binary_scope, device_control_only_tool};
pub use tool_check::{check_tool_with_scope, check_tool_with_scope_compat};

pub(crate) use catalog::PROJECT_LANGUAGE_EXECUTABLES;
pub(crate) use forbidden_script::normalize_script_token;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
