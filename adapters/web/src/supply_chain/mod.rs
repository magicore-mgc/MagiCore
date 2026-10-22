//! Public supply-chain guard facade and private owner wiring.
//! Facade công khai của guard chuỗi cung ứng và wiring owner nội bộ.

mod configuration;
mod downgrade;
mod error;
mod publish_age;
mod supply_chain;
mod types;
mod warning;

pub use supply_chain::enforce_resolution_supply_chain_guards;
pub(crate) use supply_chain::{enforce_cached_graph_downgrade_guard, resolution_age_gate_active};
