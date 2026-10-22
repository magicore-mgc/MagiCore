// package_update owner wiring — nối owner package_update.
// The implementation stays in its same-name module file.
use super::*;
#[path = "package_update.rs"]
mod implementation;
#[path = "manifest_commit/mod.rs"]
mod manifest_commit;
#[path = "package_resolution/mod.rs"]
mod package_resolution;
#[path = "post_update_install/mod.rs"]
mod post_update_install;
#[path = "target_selection/mod.rs"]
mod target_selection;
#[path = "types/mod.rs"]
mod types;
pub(crate) use implementation::{native_update, update};
