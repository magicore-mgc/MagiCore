//! Symlink Safety owner wiring — wiring cho owner symlink safety.

#[path = "symlink_safety.rs"]
mod symlink_safety;
#[cfg(unix)]
pub use symlink_safety::{
    board_selection_does_not_follow_external_mgc_config_symlink,
    framework_detection_does_not_follow_external_mgc_config_symlink,
};
