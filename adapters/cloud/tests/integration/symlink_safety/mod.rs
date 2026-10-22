//! Symlink Safety owner wiring — wiring cho owner symlink safety.

#[path = "symlink_safety.rs"]
mod symlink_safety;
#[cfg(unix)]
pub use symlink_safety::{
    type_detection_does_not_follow_external_mgc_config_symlink,
    type_detection_does_not_follow_external_package_json_symlink,
};
