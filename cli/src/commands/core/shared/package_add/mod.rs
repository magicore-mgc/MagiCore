// package_add owner wiring — nối owner package_add.
// The implementation stays in its same-name module file.
use super::*;
#[path = "package_add.rs"]
mod implementation;
#[path = "manifest_commit/mod.rs"]
mod manifest_commit;
#[path = "package_resolution/mod.rs"]
mod package_resolution;
#[path = "post_add_install/mod.rs"]
mod post_add_install;
#[path = "types/mod.rs"]
mod types;
#[path = "validation/mod.rs"]
mod validation;
pub(crate) use implementation::add;
