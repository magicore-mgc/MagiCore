//! Provider Detection owner wiring — wiring cho owner provider detection.

#[path = "provider_detection.rs"]
mod provider_detection;
#[cfg(unix)]
pub use provider_detection::provider_detection_does_not_follow_external_mgc_config_symlink;
pub use provider_detection::{
    detect_argocd_via_application_yaml, detect_aws_via_main_tf,
    detect_aws_via_mgc_toml_overrides_file_marker, detect_circleci_via_config_yml,
    detect_cloudflare_via_wrangler_toml, detect_gcp_via_mgc_toml,
    detect_github_actions_via_workflows_dir, detect_gitlab_via_gitlab_ci_yml,
    detect_returns_none_for_empty_dir, detect_returns_none_for_unknown_mgc_toml_provider,
};
