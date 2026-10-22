//! Native Rust audit entry point.
//! Entry point audit Rust native.

mod audit;
pub use audit::audit_rust;

#[cfg(test)]
#[path = "test/audit.rs"]
mod tests;
