//! Public supply-chain guard facade and private owner wiring.
//! Facade công khai của guard chuỗi cung ứng và wiring owner nội bộ.

mod configuration;
mod downgrade;
pub(crate) mod error;
#[path = "exotic_source/mod.rs"]
mod exotic_source;
mod publish_age;
mod source_claims;
mod supply_chain;
mod types;
mod warning;

pub(crate) use exotic_source::ensure_registry_only;
pub(crate) use source_claims::{RegistryOrigin, project_lockfile_sources};
pub use supply_chain::enforce_resolution_supply_chain_guards;
pub use supply_chain::ensure_project_source_claims;
pub(crate) use supply_chain::{enforce_cached_graph_downgrade_guard, resolution_age_gate_active};
