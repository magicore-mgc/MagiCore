//! Scaffold provider capability module wiring.
//! Wiring module capability scaffold provider.

mod scaffold_provider;
pub use self::scaffold_provider::ScaffoldProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
