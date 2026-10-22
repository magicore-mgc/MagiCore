//! Cloud Config owner wiring — wiring cho owner cloud config.

#[path = "cloud_config.rs"]
mod cloud_config;
pub use cloud_config::{
    detect_cloudflare_via_mgc_toml, detect_returns_none_for_empty_dir,
    detect_returns_none_for_unknown_type_in_mgc_toml,
    mgc_toml_overrides_file_marker_for_cloud_type,
    mgc_toml_recognizes_cdk_typescript_framework_alias,
    mgc_toml_recognizes_terraform_gcp_framework_alias,
};
