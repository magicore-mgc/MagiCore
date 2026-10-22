//! Layer readiness owner wiring.
//! Nối owner readiness của layer.

#[path = "layer_readiness.rs"]
mod implementation;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
