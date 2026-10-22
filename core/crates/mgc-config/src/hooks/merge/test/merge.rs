//! Tests for ordered hook command merging.
//! Test gộp command hook theo thứ tự.

use std::collections::HashMap;

use super::super::merge::merge_config;
use crate::hooks::HooksConfig;

#[test]
fn duplicate_events_keep_config_source_order() {
    let project = HooksConfig {
        hooks: HashMap::from([("install".to_string(), vec!["project".to_string()])]),
    };
    let user = HooksConfig {
        hooks: HashMap::from([("install".to_string(), vec!["user".to_string()])]),
    };
    let mut merged = HooksConfig::default();
    merge_config(&mut merged, project);
    merge_config(&mut merged, user);
    assert_eq!(
        merged.hooks["install"],
        vec!["project".to_string(), "user".to_string()]
    );
}
