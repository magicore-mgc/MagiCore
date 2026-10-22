//! Gradle exact-pin collapse tests.
//! Test gộp pin chính xác Gradle.

use super::super::super::GradleLockEntry;
use super::*;

#[test]
fn repeated_configurations_collapse_to_one_pin() {
    let entries = vec![
        GradleLockEntry {
            group: "com.example".to_string(),
            artifact: "core".to_string(),
            version: "1.2.0".to_string(),
            configuration: "debug".to_string(),
        },
        GradleLockEntry {
            group: "com.example".to_string(),
            artifact: "core".to_string(),
            version: "1.2.0".to_string(),
            configuration: "release".to_string(),
        },
    ];
    assert_eq!(
        collapse_entries(&entries).expect("test operation must succeed"),
        vec![("com.example:core".to_string(), "1.2.0".to_string())]
    );
}

#[test]
fn conflicting_versions_fail_closed() {
    let entries = vec![
        GradleLockEntry {
            group: "a.b".to_string(),
            artifact: "c".to_string(),
            version: "1.0.0".to_string(),
            configuration: "debug".to_string(),
        },
        GradleLockEntry {
            group: "a.b".to_string(),
            artifact: "c".to_string(),
            version: "2.0.0".to_string(),
            configuration: "release".to_string(),
        },
    ];
    let error = collapse_entries(&entries).expect_err("test operation must fail");
    assert!(error.to_string().contains("refusing to guess"));
}
