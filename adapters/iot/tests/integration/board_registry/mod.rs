//! Board Registry owner wiring — wiring cho owner board registry.

#[path = "board_registry.rs"]
mod board_registry;
pub use board_registry::{
    board_registry_filters_choices_by_framework_and_rejects_cross_framework_targets,
    embedded_board_registry_compatibility_api_matches_snapshot,
    platformio_esp32dev_registry_maps_to_its_esp32_target,
};
