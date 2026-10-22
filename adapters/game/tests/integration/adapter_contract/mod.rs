//! Adapter Contract owner wiring — wiring cho owner adapter contract.

#[path = "adapter_contract.rs"]
mod adapter_contract;
pub use adapter_contract::{adapter_name_and_ecosystem, can_handle_returns_true_for_game_project};
