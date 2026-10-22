//! CLI patch command facade. / Facade lệnh patch CLI.

mod add;
mod config;
mod list;
mod patch;
mod remove;
mod types;
mod verify;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub use patch::run;
pub use types::{PatchArgs, PatchCmd};

#[cfg(test)]
use config::MAX_PATCH_VERIFY_BYTES;
#[cfg(test)]
use patch::run_at_project_root;
