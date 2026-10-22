//! File Markers owner wiring — wiring cho owner file markers.

#[path = "file_markers.rs"]
mod file_markers;
pub use file_markers::{
    detect_cdk_via_package_json_aws_cdk_lib, detect_cdk_via_package_json_cdk_key,
    detect_cloudflare_via_wrangler_toml, detect_pulumi_via_pulumi_yaml,
    detect_terraform_via_any_tf_extension, detect_terraform_via_dot_tf_file,
    package_json_without_cdk_deps_not_detected_as_cdk,
};
