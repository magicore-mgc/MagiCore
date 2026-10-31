//! Flutter template wiring — wiring template Flutter.

#[path = "flutter.rs"]
mod flutter;

pub use flutter::{DART_SDK_CONSTRAINT, FLUTTER_SDK_CONSTRAINT, package_name, scaffold_project};
