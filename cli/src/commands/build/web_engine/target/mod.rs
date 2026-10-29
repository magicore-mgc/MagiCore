#[path = "target.rs"]
mod implementation;

pub(crate) use implementation::{WebBuildTarget, resolve_web_build_target};
