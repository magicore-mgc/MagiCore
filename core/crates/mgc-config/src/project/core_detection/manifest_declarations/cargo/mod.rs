//! Cargo declaration owner wiring.
//! Wiring owner khai báo Cargo.
mod cargo;
pub(super) use cargo::explicit_core;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
