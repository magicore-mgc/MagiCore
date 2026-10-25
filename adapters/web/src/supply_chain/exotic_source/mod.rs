//! Exotic dependency source policy owner.
//! Owner chính sách nguồn dependency exotic.

#[path = "exotic_source.rs"]
mod implementation;

pub(crate) use implementation::ensure_registry_only;
