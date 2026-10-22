//! Core identity normalization and membership tests.
//! Test chuẩn hóa và kiểm tra tập identity core.
use super::super::{canonical_core, is_known_core};

#[test]
fn canonical_core_trims_normalizes_case_and_maps_cloud_alias() {
    assert_eq!(canonical_core(" Cloud "), "clo");
    assert_eq!(canonical_core(" WEb "), "web");
    assert_eq!(canonical_core("unknown"), "unknown");
}

#[test]
fn accepted_core_set_is_checked_after_canonicalization() {
    for core in [
        "web", "game", "ai", "clo", "cicd", "iot", "app", "lib", "hardware",
    ] {
        assert!(is_known_core(core));
    }
    assert!(!is_known_core("cloud"));
    assert!(!is_known_core("unknown"));
}
