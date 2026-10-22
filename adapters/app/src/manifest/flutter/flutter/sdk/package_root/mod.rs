//! Flutter SDK package-root owner wiring.
//! Wiring owner đường dẫn package Flutter SDK.

mod package_root;
pub(crate) use package_root::{flutter_sdk_package_root, validate_flutter_sdk_package_name};
