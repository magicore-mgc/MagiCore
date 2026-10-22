//! IoT framework/board facade with stable exports and focused owners.
//! Facade framework/board IoT giữ export ổn định và nối các owner.

pub(super) mod board_queries;
pub(super) mod board_registry;
pub(super) mod board_types;
mod framework;
pub(super) mod framework_detection;
pub(super) mod framework_type;
pub(super) mod legacy_board_map;
pub(super) mod manifest_metadata;

pub use board_queries::{
    board_target, board_target_for_framework, boards_for_framework, known_boards,
};
pub use board_types::BoardInfo;
pub(in crate::framework) use framework::canonical_framework;
pub use framework_detection::detect_framework;
pub use framework_type::IotFramework;
pub use legacy_board_map::KNOWN_BOARDS;
pub(crate) use manifest_metadata::{manifest_is_iot, target_from_manifest};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
