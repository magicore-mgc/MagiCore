//! Tests core-specific outdated query routing.
//! Kiểm tra định tuyến truy vấn outdated theo core.

#![allow(clippy::unwrap_used)]

use super::super::is_web_core;

#[test]
fn only_web_uses_the_web_registry_lane_and_every_other_known_core_is_native() {
    for core in mgc_config::project::ProjectConfig::KNOWN_CORES
        .iter()
        .copied()
    {
        assert_eq!(is_web_core(core), core == "web", "core: {core}");
    }
}
