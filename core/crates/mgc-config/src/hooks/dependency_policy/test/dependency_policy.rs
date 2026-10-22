//! Tests for dependency lifecycle hook classification and veto.
//! Test nhận diện event dependency và chặn hook.

use std::collections::HashMap;

use super::super::{dependency_event_verb, ensure_dependency_event_hooks_allowed};
use crate::hooks::HooksConfig;

#[test]
fn dependency_verbs_include_direct_pre_and_post_events() {
    for verb in ["install", "add", "remove", "update", "publish"] {
        assert_eq!(dependency_event_verb(verb), Some(verb));
        assert_eq!(dependency_event_verb(&format!("pre-{verb}")), Some(verb));
        assert_eq!(dependency_event_verb(&format!("post-{verb}")), Some(verb));
    }
}

#[test]
fn unrelated_events_are_not_dependency_events() {
    assert_eq!(dependency_event_verb("test"), None);
    assert_eq!(dependency_event_verb("pre-build"), None);
}

#[test]
fn configured_post_hook_blocks_pre_dependency_phase() {
    let config = HooksConfig {
        hooks: HashMap::from([("post-install".to_string(), vec!["tool run".to_string()])]),
    };
    let error = ensure_dependency_event_hooks_allowed(&config, "pre-install").unwrap_err();
    assert!(error.to_string().contains("disabled"));
}

#[test]
fn empty_dependency_hook_does_not_trigger_the_veto() {
    let config = HooksConfig {
        hooks: HashMap::from([("pre-install".to_string(), vec!["  ".to_string()])]),
    };
    assert!(ensure_dependency_event_hooks_allowed(&config, "pre-install").is_ok());
}
