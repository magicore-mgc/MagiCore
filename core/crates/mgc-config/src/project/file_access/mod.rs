//! Shared project-file access facade and functional-owner wiring.
//! Facade truy cập file project dùng chung và wiring owner chức năng.
mod atomic_write_project_config;
mod ecosystem_from_config;
mod error;
mod project_config_metadata;
mod read_to_string_ecosystem;
mod regular_project_text;

pub(super) use atomic_write_project_config::atomic_write_project_config;
pub(super) use ecosystem_from_config::ecosystem_from_config;
pub(super) use project_config_metadata::ensure_regular_project_config;
pub(super) use read_to_string_ecosystem::read_to_string_ecosystem;
pub use regular_project_text::read_regular_project_text;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
