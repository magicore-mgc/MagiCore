// template_path owner wiring — nối owner template_path.
// Implementation details remain in the same-name module file.
use super::*;
#[path = "template_path.rs"]
mod implementation;
pub(in crate::scaffold::processor) use implementation::*;
