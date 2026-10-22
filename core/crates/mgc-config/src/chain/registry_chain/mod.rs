//! Wiring and exports for registry-chain behavior owners.
//! Wiring và export cho các owner xử lý registry-chain.

mod builder;
mod checked;
mod legacy;
mod scoped_registry;

pub use builder::{DEFAULT_NPM_REGISTRY, WEB_REGISTRY_URL_ENV};
pub use checked::{registry_chain_checked, registry_chain_scoped_checked};
pub use legacy::registry_chain;
pub use scoped_registry::ScopedRegistry;
