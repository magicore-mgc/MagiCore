//! Lockfile provider capability module wiring.
//! Wiring module capability lockfile provider.

mod lockfile_provider;
pub use self::lockfile_provider::LockfileProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
