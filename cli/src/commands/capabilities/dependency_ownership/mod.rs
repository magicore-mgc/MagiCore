//! Dependency ownership wiring — nối owner dependency ownership.
//! Child owners project the gate policy without redefining it.
//! Owner con chiếu policy gate, không định nghĩa lại policy.

mod dependency_ownership;
mod language_projection;
mod manifest_variants;
mod operation_projection;

pub use dependency_ownership::dependency_ownership;
