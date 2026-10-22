//! Existing capability ownership test — test capability ownership hiện có.
//! Its assertion set remains intact and runs in the same order.
//! Giữ nguyên bộ assertion và thứ tự chạy.

#[path = "capabilities_base_assertions.rs"]
mod base_assertions;
#[path = "capabilities_manifest_assertions.rs"]
mod manifest_assertions;

#[test]
fn capabilities_json_carries_dep_gate_ownership() {
    base_assertions::assert_base_rows();
    manifest_assertions::assert_manifest_and_core_overrides();
}
