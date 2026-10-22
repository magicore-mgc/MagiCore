//! TypeScript scaffold owner wiring.
//! Wiring owner scaffold TypeScript.

#[path = "typescript.rs"]
mod implementation;
pub(super) use implementation::write;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
