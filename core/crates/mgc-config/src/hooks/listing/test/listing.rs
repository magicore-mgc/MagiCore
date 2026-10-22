//! Tests for returning the merged hook event map.
//! Test trả về map event hook đã gộp.

use std::collections::HashMap;

use super::super::list_configured_hooks;
use crate::hooks::HooksConfig;

#[test]
fn list_owner_returns_the_loaded_event_map() {
    let hooks = HashMap::from([("pre-install".to_string(), vec!["cargo check".to_string()])]);
    let listed = list_configured_hooks(HooksConfig {
        hooks: hooks.clone(),
    });
    assert_eq!(listed, hooks);
}
