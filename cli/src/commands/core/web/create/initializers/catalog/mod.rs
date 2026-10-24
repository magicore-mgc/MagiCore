//! Official initializer catalog owner.
//! Owner catalog initializer chính thức.

#[path = "catalog.rs"]
mod implementation;

pub(in crate::commands::core::web::implementation) use implementation::{BACKEND, FRONTEND};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
