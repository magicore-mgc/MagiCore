//! Native registry clients for app platforms.

pub mod cocoapods_client;
pub mod maven_client;
pub mod pub_client;
pub mod rn_layers;

mod contract;

pub use contract::{AppPackageMetadata, AppRegistryClient};
