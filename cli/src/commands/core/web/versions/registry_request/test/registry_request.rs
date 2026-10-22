//! Registry failure behavior tests.
//! Test hành vi lỗi registry.

use super::super::super as versions;

#[test]
#[cfg(feature = "web")]
fn scaffold_age_gate_fails_closed_when_registry_metadata_is_unavailable() {
    let error = versions::fallback_scaffold_version(
        "react",
        anyhow::anyhow!("registry is offline"),
        Some(versions::ScaffoldAgePolicy {
            cutoff_hours: 24,
            allow_missing_time: false,
        }),
    )
    .unwrap_err();

    let message = error.to_string();
    assert!(message.contains("minimum-release-age"));
    assert!(message.contains("react"));
}

#[test]
#[cfg(feature = "web")]
fn scaffold_version_resolution_never_uses_a_stale_static_fallback() {
    let error =
        versions::fallback_scaffold_version("react", anyhow::anyhow!("registry is offline"), None)
            .expect_err("scaffolding must not substitute a pinned historical version");

    let message = error.to_string();
    assert!(message.contains("current registry version"));
    assert!(message.contains("react"));
    assert!(message.contains("registry is offline"));
}
