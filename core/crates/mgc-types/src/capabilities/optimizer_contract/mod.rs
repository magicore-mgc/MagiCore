//! Optimizer capability contract module wiring.
//! Wiring module hợp đồng capability optimizer.

mod optimizer_contract;
pub use self::optimizer_contract::{OPTIMIZER_CAPABILITY_CORES, optimizer_capability_supported};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
