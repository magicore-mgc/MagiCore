//! Persistent, user-scoped CLI tools installed through MagiCore's native Web lane.
//! CLI bền vững theo user, cài qua Web adapter của MagiCore.

mod atomic_file;
mod install;
mod json_file;
mod list;
mod managed_roots;
mod native_install;
mod project_install;
mod record_lookup;
mod record_validation;
mod record_write;
mod remove;
mod shim;
mod tool;
mod types;
mod update;

pub use self::tool::run;
