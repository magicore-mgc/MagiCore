//! Offline App project templates — template project App viết offline.
//!
//! `flutter` is the only App language whose dependency engine MagiCore owns, so
//! it is the only one whose project can be scaffolded without a third-party SDK
//! binary. Other languages keep failing closed at the initializer.
//! (`flutter` là language App duy nhất có dependency engine do MagiCore sở hữu,
//! nên cũng là language duy nhất scaffold được mà không cần binary SDK bên
//! thứ ba. Language khác vẫn fail-closed tại initializer.)

mod flutter;

pub use flutter::{DART_SDK_CONSTRAINT, FLUTTER_SDK_CONSTRAINT, package_name, scaffold_project};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
