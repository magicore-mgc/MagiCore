#[path = "runtime_detection.rs"]
mod implementation;

pub(crate) use implementation::{detect_app_runtime, detect_lib_runtime};
