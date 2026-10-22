//! Catalog specifier resolution facade.
//! Facade phân giải specifier catalog.

mod resolve;

pub use resolve::resolve_catalog_specifier;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
