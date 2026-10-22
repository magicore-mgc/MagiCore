//! Explicit project identity wiring — nối marker và project config.
//! Wiring identity dự án tường minh — marker và cấu hình project.

#[path = "explicit_identity.rs"]
mod explicit_identity;

pub(in crate::dispatch) use explicit_identity::read;
