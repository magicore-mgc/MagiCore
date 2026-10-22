//! Content store capability module wiring.
//! Wiring module capability content store.

mod content_store_provider;
pub use self::content_store_provider::ContentStoreProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
