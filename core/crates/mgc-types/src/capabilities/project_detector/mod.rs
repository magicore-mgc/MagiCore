//! Project detector capability module wiring.
//! Wiring module capability nhận diện dự án.

mod project_detector;
pub use self::project_detector::ProjectDetector;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
