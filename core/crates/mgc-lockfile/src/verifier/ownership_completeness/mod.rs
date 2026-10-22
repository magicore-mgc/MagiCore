//! Wires the independent package provenance-completeness gate.
//! Nối cổng kiểm tra completeness provenance package độc lập.

mod error;
mod ownership_completeness;

pub use self::ownership_completeness::verify_ownership_completeness;
