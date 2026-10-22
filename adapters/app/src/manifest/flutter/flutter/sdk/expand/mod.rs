//! Flutter SDK expansion entry-point wiring.
//! Wiring entry point mở rộng Flutter SDK.

mod expand;
#[cfg(test)]
pub(crate) use expand::expand_flutter_sdk_dependencies_with_root;
pub(crate) use expand::{
    expand_flutter_sdk_dependencies, flutter_sdk_packages, has_flutter_sdk_dependencies,
};
