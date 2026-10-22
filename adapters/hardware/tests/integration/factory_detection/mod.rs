//! Factory Detection owner wiring.
//! Wiring cho owner factory detection.

#[path = "factory_detection.rs"]
mod factory_detection;
pub use factory_detection::{
    adapter_for_returns_none_for_dir_without_mgc_toml,
    adapter_for_returns_none_for_non_mgc_ecosystem,
    adapter_for_returns_some_for_game_ecosystem_cross_core,
    adapter_for_returns_some_for_hardware_ecosystem,
};
