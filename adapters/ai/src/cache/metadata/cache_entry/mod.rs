//! Cache-entry exports — xuất type metadata public và from-path constructor.

mod cache_entry;

pub use cache_entry::CacheEntry;

#[cfg(test)]
#[path = "test/cache_entry.rs"]
mod tests;
