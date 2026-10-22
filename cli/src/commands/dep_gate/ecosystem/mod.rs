//! Canonical ecosystem identifier wiring — wiring ecosystem id chuẩn.
mod ecosystem;
pub use ecosystem::eco;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
