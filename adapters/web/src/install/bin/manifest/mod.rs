//! Wiring for installed `package.json` executable metadata.
//! Wiring cho metadata executable trong `package.json` đã cài.

mod manifest;
pub use manifest::{InstalledPackageManifest, PackageBinField, package_bin_entries};
