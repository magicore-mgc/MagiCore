//! Tree owner wiring.
//! Wiring owner tree.

#[path = "tree.rs"]
mod implementation;

pub use implementation::{hardlink_tree, hardlink_tree_with_profile};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
