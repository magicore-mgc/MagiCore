//! Installed Metadata owner wiring.
//! Nối owner installed_metadata.

mod error;
mod installed_metadata;
pub use error::registry_package_invalid_bins;
pub(crate) use installed_metadata::{requested_package_bins, requested_package_version};
