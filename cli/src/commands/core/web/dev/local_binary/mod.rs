// local_binary owner wiring — nối owner local_binary.
// local_binary module export stays focused at this boundary.
use super::*;
#[path = "local_binary.rs"]
mod implementation;
pub(super) use implementation::*;
