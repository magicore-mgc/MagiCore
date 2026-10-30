//! Checks the stable JSON field contract of the public report model.
//! Kiểm tra contract field JSON ổn định của report công khai.

use crate::commands::doctor::{DiagnosticIssue, DoctorReport, RegistryStatus, Tool};

#[test]
fn report_json_keeps_all_public_fields_and_names() {
    let report = DoctorReport {
        toolchain: vec![Tool {
            name: "git",
            path: Some("/usr/bin/git".to_string()),
            version: "git version 2".to_string(),
            required_by_core: None,
        }],
        store_dir: "/tmp/mgc-store".to_string(),
        store_writable: true,
        disk_free_bytes: 42,
        registries_reachable: vec![RegistryStatus {
            host: "example.invalid".to_string(),
            reachable: false,
        }],
        health_status: "DEGRADED",
        detected_issues: vec![DiagnosticIssue {
            code: "WARN_TEST",
            severity: "WARNING",
            message: "diagnostic".to_string(),
            fix_command: None,
        }],
        suggested_actions: vec!["inspect".to_string()],
    };

    let value = serde_json::to_value(report).unwrap();
    let object = value.as_object().unwrap();
    for field in [
        "toolchain",
        "store_dir",
        "store_writable",
        "disk_free_bytes",
        "registries_reachable",
        "health_status",
        "detected_issues",
        "suggested_actions",
    ] {
        assert!(object.contains_key(field), "missing report field {field}");
    }
    assert_eq!(object["health_status"], "DEGRADED");
    assert_eq!(object["detected_issues"][0]["code"], "WARN_TEST");
}
