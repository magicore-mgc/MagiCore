//! `native/mod.rs` — Native registry clients for lib adapter facade.
//! Facade client registry native cho adapter lib (crates.io và PyPI, tránh gọi cargo/pip).

pub mod cargo_client;
pub mod engine;
pub mod pypi_client;

mod contract;

pub use contract::{PackageMetadata, RegistryClient};
