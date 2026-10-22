//! Native Rust OSV scanner exports.
//! Export scanner OSV native cho Rust.

mod audit;
mod cargo_lock;

pub use audit::audit_rust;
pub use cargo_lock::read_cargo_lock_pins;

#[cfg(test)]
#[path = "test/rust.rs"]
mod tests;
