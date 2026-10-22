//! Flutter manifest public exports and nearest owner tests.
//! Export manifest Flutter và test đặt cạnh owner.

mod error;
mod flutter;

pub(super) use error::{FlutterError, FlutterResult};
pub(crate) use flutter::{
    expand_flutter_sdk_dependencies, flutter_sdk_package_names, flutter_sdk_package_root,
    flutter_sdk_packages, has_flutter_sdk_dependencies,
};
#[cfg(test)]
pub(crate) use flutter::{
    expand_flutter_sdk_dependencies_with_root, find_flutter_executable_in_path,
};
pub use flutter::{parse_podfile, parse_pubspec, write_podfile, write_pubspec};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
