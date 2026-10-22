//! Native adapters must be bound to each project instead of reused by core id.
//! Kiểm tra bốn core không dùng lại adapter chứa cấu hình project khác.

#![allow(clippy::unwrap_used)]

use mgc_types::Ecosystem;
use std::sync::Arc;

fn assert_distinct_project_adapters(ecosystem: Ecosystem, files: &[(&str, &str)]) {
    let first = tempfile::tempdir().unwrap();
    let second = tempfile::tempdir().unwrap();
    for root in [first.path(), second.path()] {
        for (name, body) in files {
            std::fs::write(root.join(name), body).unwrap();
        }
    }
    let a = mgc::factory::create_adapter_for(
        first.path(),
        &ecosystem,
        Some("https://first.invalid/registry"),
        None,
        &[],
    )
    .unwrap();
    let b = mgc::factory::create_adapter_for(
        second.path(),
        &ecosystem,
        Some("https://second.invalid/registry"),
        None,
        &[],
    )
    .unwrap();
    assert!(
        !Arc::ptr_eq(&a, &b),
        "native adapters must not share project-specific configuration"
    );
    assert_eq!(a.ecosystem(), ecosystem);
    assert_eq!(b.ecosystem(), ecosystem);
}

#[cfg(feature = "web")]
#[test]
fn web_registry_context_is_not_reused() {
    assert_distinct_project_adapters(
        Ecosystem::Web,
        &[("package.json", "{\"name\":\"fixture\"}")],
    );
}

#[cfg(feature = "ai")]
#[test]
fn ai_project_context_is_not_reused() {
    assert_distinct_project_adapters(
        Ecosystem::Ai,
        &[
            ("mgc.toml", "[ai]\nframework=\"python-agent\"\n"),
            (
                "pyproject.toml",
                "[project]\nname=\"fixture\"\nversion=\"1.0.0\"\n",
            ),
        ],
    );
}

#[cfg(feature = "app")]
#[test]
fn app_project_context_is_not_reused() {
    assert_distinct_project_adapters(Ecosystem::App, &[("pubspec.yaml", "name: fixture\n")]);
}

#[cfg(feature = "lib")]
#[test]
fn lib_project_context_is_not_reused() {
    assert_distinct_project_adapters(
        Ecosystem::Lib,
        &[(
            "pyproject.toml",
            "[project]\nname=\"fixture\"\nversion=\"1.0.0\"\n",
        )],
    );
}
