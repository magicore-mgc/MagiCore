//! Entry owner wiring.
//! Wiring owner entry.

#[path = "entry.rs"]
mod implementation;

pub use implementation::{link_package_tree, link_package_tree_with_profile};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
