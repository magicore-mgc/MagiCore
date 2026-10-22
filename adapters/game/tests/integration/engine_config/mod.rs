//! Engine Config owner wiring — wiring cho owner engine config.

#[path = "engine_config.rs"]
mod engine_config;
pub use engine_config::{
    detect_returns_none_for_empty_dir, detect_via_mgc_toml_overrides_file_marker,
};
