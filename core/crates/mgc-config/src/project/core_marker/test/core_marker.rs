//! Stable core-marker facade contract tests.
//! Test contract facade core marker ổn định.
use super::super::super::ProjectConfig;

#[test]
fn marker_filename_and_core_order_remain_stable() {
    assert_eq!(ProjectConfig::CORE_MARKER_FILE, ".mgc.core");
    assert_eq!(
        ProjectConfig::KNOWN_CORES,
        &[
            "web", "game", "ai", "clo", "cicd", "iot", "app", "lib", "hardware"
        ]
    );
}
