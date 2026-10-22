//! Public exports for registry-chain owners.
//! Xuất API công khai từ các owner registry-chain.

mod registry_chain;

pub use registry_chain::{
    DEFAULT_NPM_REGISTRY, ScopedRegistry, WEB_REGISTRY_URL_ENV, registry_chain,
    registry_chain_checked, registry_chain_scoped_checked,
};
