//! Shared task-run command facade.
//! Facade lệnh chạy task dùng chung.

mod run;
mod run_with_hooks;
mod runtime_policy;
mod script_policy;
mod script_resolution;
mod task_execution;

pub use run::run;
pub(crate) use run_with_hooks::run_with_hooks;
pub(crate) use script_resolution::resolve_task_script;
pub(crate) use task_execution::run_task_script;

#[cfg(test)]
use script_resolution::{resolve_mgc_toml_script, resolve_package_json_script};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
