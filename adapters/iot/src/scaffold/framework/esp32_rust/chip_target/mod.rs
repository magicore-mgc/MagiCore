//! Chip-to-Rust-target mapping owner.
//! Owner mapping chip sang target Rust.

mod chip_target;
pub(in crate::scaffold) use chip_target::{chip_target, render_cargo_config};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
