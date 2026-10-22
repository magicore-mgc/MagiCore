//! Go scaffold owner wiring.
//! Wiring owner scaffold Go.

#[path = "go.rs"]
mod implementation;
pub(super) use implementation::write;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
