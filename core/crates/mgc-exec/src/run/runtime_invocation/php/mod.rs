//! PHP argv parser exports.
//! Export parser argv PHP.

#[path = "php.rs"]
mod implementation;
pub(super) use implementation::php_script_argument;
