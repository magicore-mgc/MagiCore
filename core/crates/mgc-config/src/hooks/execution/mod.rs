//! Hook execution API exports.
//! Export API chạy hook.

mod execution;

pub use execution::run_hooks;
#[cfg(test)]
pub(super) use execution::run_hooks_with_config;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
