//! Java scaffold owner wiring.
//! Wiring owner scaffold Java.

#[path = "java.rs"]
mod implementation;
pub(super) use implementation::write;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
