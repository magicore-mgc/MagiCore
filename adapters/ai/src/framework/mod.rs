//! AI framework module wiring and public exports.
//! Wiring module và export công khai của framework AI.

mod detection;
mod framework;
mod types;

pub use framework::{AiFramework, detect_framework};
