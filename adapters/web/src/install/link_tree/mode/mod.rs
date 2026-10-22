//! Mode owner wiring.
//! Wiring owner mode.

#[path = "mode.rs"]
mod implementation;

pub use implementation::{StrictTreeLinkMode, strict_tree_link_mode};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
