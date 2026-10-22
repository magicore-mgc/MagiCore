#[path = "error.rs"]
mod error;
#[path = "multi.rs"]
mod implementation;

pub(crate) use implementation::build_multi_app;
#[cfg(test)]
pub(crate) use implementation::{finish_multi_build, resolve_multi_platforms};
