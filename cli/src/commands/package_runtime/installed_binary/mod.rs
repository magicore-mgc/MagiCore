//! Installed Binary owner wiring.
//! Nối owner installed_binary.

mod error;
mod installed_binary;
pub use error::{
    registry_package_bin_target_mismatch, registry_package_binary_outside_project,
    registry_package_no_binary,
};
pub(super) use installed_binary::{VerifiedBinary, verify_installed_binary};
