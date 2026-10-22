//! Web install-layout policy facade.
//! Facade policy layout install Web.

#[path = "install_layout.rs"]
mod implementation;

pub use implementation::should_use_legacy_flat_layout;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
