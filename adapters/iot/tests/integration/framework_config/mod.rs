//! Framework Config owner wiring — wiring cho owner framework config.

#[path = "framework_config.rs"]
mod framework_config;
pub use framework_config::detect_esp32_rust_via_mgc_toml;
