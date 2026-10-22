//! Wire root pubspec parsing and dependency-name projection.
//! Nối parse pubspec gốc và projection tên dependency.

mod dependency_names;
mod root_manifest;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

use self::dependency_names::flutter_pubspec_dependency_names;
pub(super) use self::root_manifest::read_root_manifest;
