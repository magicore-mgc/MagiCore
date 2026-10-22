//! Native engine wiring tests for the lib adapter — hermetic via mockito.
//! Test đi dây engine native cho lib adapter — hermetic qua mockito.

#![allow(clippy::unwrap_used)]
#![allow(unsafe_code)]

#[path = "native_engine_tests/mod.rs"]
mod native_engine_tests;

use native_engine_tests::support::environment_override as environment;
use native_engine_tests::{
    crates_graph as crates, go_module_graph as go, maven_project as maven,
    nuget_integrity as nuget, nuget_project, rust_adapter as rust,
};

#[tokio::test]
async fn resolve_with_protocol_builds_graph_and_v3_lock_entries() {
    crates::resolve_with_protocol_builds_graph_and_v3_lock_entries().await;
}

#[tokio::test]
async fn native_engine_preserves_multiple_versions_and_exact_dependency_edges() {
    crates::native_engine_preserves_multiple_versions_and_exact_dependency_edges().await;
}

#[tokio::test]
async fn resolve_with_protocol_rejects_cross_root_version_conflicts() {
    crates::resolve_with_protocol_rejects_cross_root_version_conflicts().await;
}

#[tokio::test]
async fn nuget_resolution_rejects_missing_malformed_or_ambiguous_sha512_markers() {
    nuget::nuget_resolution_rejects_missing_malformed_or_ambiguous_sha512_markers().await;
}

#[tokio::test]
async fn lib_adapter_resolve_uses_native_engine_via_env() {
    rust::lib_adapter_resolve_uses_native_engine_via_env().await;
}

#[tokio::test]
async fn resolve_with_protocol_builds_go_module_graph_and_v3_lock_entries() {
    go::resolve_with_protocol_builds_go_module_graph_and_v3_lock_entries().await;
}

#[tokio::test]
async fn lib_adapter_maven_pom_project_resolves_natively_via_env() {
    maven::lib_adapter_maven_pom_project_resolves_natively_via_env().await;
}

#[tokio::test]
async fn lib_adapter_csproj_project_resolves_natively_via_env() {
    nuget_project::lib_adapter_csproj_project_resolves_natively_via_env().await;
}

#[test]
fn environment_overrides_restore_preexisting_and_absent_values() {
    environment::environment_override_restores_preexisting_and_absent_values();
}
