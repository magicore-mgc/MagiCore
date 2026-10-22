//! Framework Detection owner wiring — wiring cho owner framework detection.

#[path = "framework_detection.rs"]
mod framework_detection;
pub use framework_detection::{
    detect_platformio_via_ini, detect_returns_none_for_empty_dir, detect_zephyr_via_west_yml,
};
