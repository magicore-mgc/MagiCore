//! Hook configuration exports.
//! Export cấu hình hook.

mod config;

pub use config::HooksConfig;
pub(super) use config::load_hook_config;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
