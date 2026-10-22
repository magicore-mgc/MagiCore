//! Fail-closed and extra-marker contracts for dependency projection.
//! Hợp đồng fail-closed và extra marker của dependency projection.

use super::super::project_dependencies;
use mgc_types::Version;

#[test]
fn unrequested_extra_marker_is_recorded_without_adding_an_edge() {
    let mut markers = Vec::new();
    let deps = project_dependencies(
        Some(vec!["attrs[tests]>=22; extra == \"cov\"".to_string()]),
        "demo",
        &Version::parse("1.0.0").expect("test operation must succeed"),
        &mut markers,
    )
    .expect("test operation must succeed");

    assert!(deps.is_empty());
    assert_eq!(markers, vec![String::from("marker:extra == \"cov\"")]);
}

#[test]
fn malformed_requirement_is_not_silently_dropped() {
    let error = project_dependencies(
        Some(vec![">=1".to_string()]),
        "demo",
        &Version::parse("1.0.0").expect("test operation must succeed"),
        &mut Vec::new(),
    )
    .expect_err("malformed Requires-Dist must fail closed");

    assert!(error.to_string().contains("Requires-Dist"));
}
