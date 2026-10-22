//! Lib language detection module wiring.
//! Wiring module nhận diện ngôn ngữ Lib.

mod detection;
mod language_name;
mod markers;
mod project_config;

pub use detection::detect_language;
