//! Dispatch owner exports the stable runtime-detection functions.
mod runtime_detect;
pub use runtime_detect::{detect_optimizer_runtimes, detect_runtimes};

#[cfg(test)]
#[path = "test/runtime_detect.rs"]
mod tests;
