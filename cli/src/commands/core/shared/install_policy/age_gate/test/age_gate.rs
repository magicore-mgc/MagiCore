use super::super::{age_gate_lock_action, refresh_locked_graph_for_age_gate};

#[test]
fn active_age_gate_rejects_locked_frozen_replay_without_release_evidence() {
    let error = age_gate_lock_action("web", true, true, true, true).unwrap_err();
    assert!(error.to_string().contains("publication timestamps"));
    assert!(age_gate_lock_action("web", true, true, true, false).unwrap());
    assert!(!age_gate_lock_action("web", false, false, true, true).unwrap());
    assert!(!age_gate_lock_action("web", true, true, false, true).unwrap());
    assert!(age_gate_lock_action("iot", true, false, false, false).is_err());
}

#[test]
fn install_lock_refresh_uses_project_policy_and_adapter_capability() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("mgc.toml"), "[security]\nweb = 24\n").unwrap();
    let web = mgc_web_adapter::WebAdapter::new().unwrap();

    assert!(refresh_locked_graph_for_age_gate(&web, project.path(), true, false).unwrap());
    assert!(refresh_locked_graph_for_age_gate(&web, project.path(), true, true).is_err());
}

#[cfg(feature = "app")]
#[test]
fn install_lock_refresh_rejects_unsupported_app_adapter() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("mgc.toml"), "[security]\napp = 24\n").unwrap();
    let app = mgc_app_adapter::AppAdapter::new(mgc_app_adapter::AppLanguage::Flutter);
    assert!(refresh_locked_graph_for_age_gate(&app, project.path(), false, false).is_err());
}
