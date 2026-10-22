//! Behavior tests for the dependency_graph owner.
//! Test hành vi cho owner dependency_graph.

use super::super::*;
use mgc_types::MgError;

#[test]
fn divergent_tfm_dependency_sets_fail_without_a_consumer_framework() {
    let nuspec = r#"
        <dependencies>
            <group targetFramework="net6.0"><dependency id="A" version="[1.0]" /></group>
            <group targetFramework="net8.0"><dependency id="B" version="[2.0]" /></group>
        </dependencies>
    "#;

    let error = resolve_nuspec_dependencies("Demo", "1.0.0", nuspec, None)
        .expect_err("test operation must fail");

    assert!(matches!(
        error,
        MgError::Other(message) if message.contains("target-framework selection is required")
    ));
}

#[test]
fn divergent_tfm_dependency_sets_select_only_the_nearest_consumer_group() {
    let nuspec = r#"
        <dependencies>
            <group targetFramework="net6.0"><dependency id="A" version="[1.0]" /></group>
            <group targetFramework="net8.0"><dependency id="B" version="[2.0]" /></group>
        </dependencies>
    "#;

    let graph = resolve_nuspec_dependencies("Demo", "1.0.0", nuspec, Some("net8.0"))
        .expect("test operation must succeed");

    assert_eq!(
        graph.dependencies,
        vec![("B".to_string(), "[2.0]".to_string())]
    );
    assert!(graph.markers.contains(&"tfm-selected:net8.0".to_string()));
}

#[test]
fn identical_tfm_dependency_sets_merge_and_emit_marker() {
    let nuspec = r#"
        <dependencies>
            <group targetFramework="net6.0"><dependency id="A" version="[1.0]" /></group>
            <group targetFramework="net8.0"><dependency id="A" version="[1.0]" /></group>
        </dependencies>
    "#;

    let graph = resolve_nuspec_dependencies("Demo", "1.0.0", nuspec, None)
        .expect("test operation must succeed");

    assert_eq!(graph.dependencies.len(), 2);
    assert!(
        graph
            .markers
            .contains(&"multi-tfm-identical:net6.0,net8.0".to_string())
    );
}

#[test]
fn dependency_without_version_is_skipped_and_marked() {
    let nuspec = r#"<dependencies><group><dependency id="NoVersion" /></group></dependencies>"#;

    let graph = resolve_nuspec_dependencies("Demo", "1.0.0", nuspec, None)
        .expect("test operation must succeed");

    assert!(graph.dependencies.is_empty());
    assert_eq!(
        graph.markers,
        vec!["unresolved-version:NoVersion".to_string()]
    );
}
