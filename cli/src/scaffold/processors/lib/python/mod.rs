//! Python scaffold owner wiring.
//! Wiring owner scaffold Python.

#[path = "python.rs"]
mod implementation;
pub(super) use implementation::write;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
