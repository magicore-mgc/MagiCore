//! Monorepo stack owner wiring.
//! Nối owner stack monorepo.

#[path = "monorepo_layer_stack.rs"]
mod implementation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
