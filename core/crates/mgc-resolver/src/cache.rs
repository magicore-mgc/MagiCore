//! Public resolver cache module surface.
//! Mặt export công khai của cache resolver.

mod bounded_insert;
mod cache_entry;
mod fresh_read;
pub mod registry_cache;

pub use registry_cache::RegistryCache;
