//! Adapter Contract owner wiring.
//! Wiring cho owner adapter contract.

#[path = "adapter_contract.rs"]
mod adapter_contract;
pub use adapter_contract::{
    adapter_name_and_ecosystem, can_handle_returns_false_for_plain_dir,
    can_handle_returns_true_for_hardware_ecosystem,
};
