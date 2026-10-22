//! Web build bundler owner wiring and compatibility exports.
//! Wiring owner build bundler Web và export tương thích.

mod assets;
mod bundle;
mod dependency_cache;
pub mod dev_server;
mod error;
pub mod hmr;
mod types;
mod workspace;

pub use assets::process_assets;
pub use bundle::Bundler;
pub use dependency_cache::{CachedDep, DepsCache, dep_name_is_valid};
pub use types::{BundleResult, BundlerConfig};
