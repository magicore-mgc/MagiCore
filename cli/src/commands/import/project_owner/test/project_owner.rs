//! Project core-owner mapping cases — ca ánh xạ owner core.
#![allow(clippy::unwrap_used)]

use super::super::{normalize_config_owner, validate_core_owner};

#[test]
fn accepts_each_known_project_core_owner() {
    for core in mgc_config::project::ProjectConfig::KNOWN_CORES {
        assert_eq!(validate_core_owner((*core).to_string()).unwrap(), *core);
    }
}

#[test]
fn normalizes_config_cloud_alias_to_clo() {
    assert_eq!(normalize_config_owner("cloud"), "clo");
    assert_eq!(normalize_config_owner(" Cloud "), "clo");
}

#[test]
fn normalizes_other_config_core_names_without_rewriting_them() {
    assert_eq!(normalize_config_owner(" WEB "), "web");
    assert_eq!(normalize_config_owner("hardware"), "hardware");
}

#[test]
fn rejects_unknown_owner() {
    let error = validate_core_owner("other".to_string()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "refusing to import lock for unknown core owner 'other'"
    );
}
