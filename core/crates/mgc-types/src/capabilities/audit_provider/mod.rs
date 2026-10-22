//! Audit provider capability module wiring.
//! Wiring module capability audit provider.

mod audit_provider;
pub use self::audit_provider::AuditProvider;
#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
