//! Request Path owner wiring — nối owner request path.
#[path = "request_path.rs"]
mod implementation;
pub(super) use self::implementation::request_path_is_safe;
