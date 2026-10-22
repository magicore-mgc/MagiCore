// package_remove owner wiring — nối owner package_remove.
// The implementation stays in its same-name module file.
use super::*;
#[path = "dependency_removal/mod.rs"]
mod dependency_removal;
#[path = "package_remove.rs"]
mod implementation;
#[path = "manifest_commit/mod.rs"]
mod manifest_commit;
#[path = "post_remove_install/mod.rs"]
mod post_remove_install;
#[path = "validation/mod.rs"]
mod validation;
pub(crate) use implementation::remove;
