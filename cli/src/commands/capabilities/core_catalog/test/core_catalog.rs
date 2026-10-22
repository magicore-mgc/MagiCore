//! Core catalog behavior tests — test hành vi catalog core.
//! Locks canonical order and the unknown-core diagnostic.
//! Khóa thứ tự chuẩn và thông báo core không biết.

use super::super::{ALL_CORES, capabilities_for_core};

#[test]
fn canonical_core_order_is_stable() {
    assert_eq!(
        ALL_CORES,
        [
            "web", "lib", "ai", "app", "game", "iot", "clo", "cicd", "hardware"
        ]
    );
}

#[test]
fn unknown_core_keeps_the_legacy_diagnostic() {
    let error = capabilities_for_core("not-a-core").err().unwrap();
    assert_eq!(error.to_string(), "Unknown core 'not-a-core'");
}
