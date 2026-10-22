// package_list owner wiring — nối owner package_list.
// The implementation stays in its same-name module file.
use super::*;
#[path = "package_list.rs"]
mod implementation;
pub(crate) use implementation::list;
