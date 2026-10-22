//! Deploy provider capability module wiring.
//! Wiring module capability deploy provider.

mod deploy_provider;
pub use self::deploy_provider::DeployProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
