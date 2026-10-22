//! Checks terminal sections and JSON output from a deterministic report.
//! Kiểm tra section terminal và JSON từ report xác định.

use crate::commands::doctor::presentation::{render_human, render_json};
use crate::commands::doctor::storage::{RemediationOutcome, RemediationStatus};
use crate::commands::doctor::{DoctorReport, Tool};

fn report() -> DoctorReport {
    DoctorReport {
        toolchain: vec![Tool {
            name: "git",
            path: None,
            version: "MISSING".to_string(),
        }],
        store_dir: "/tmp/mgc-store".to_string(),
        store_writable: false,
        disk_free_bytes: 0,
        registries_reachable: Vec::new(),
        health_status: "UNHEALTHY",
        detected_issues: Vec::new(),
        suggested_actions: Vec::new(),
    }
}

#[test]
fn human_report_keeps_tool_and_store_rows_and_omits_unknown_disk_size() {
    let output = render_human(&report(), None);
    assert!(output.contains("Environment Health: UNHEALTHY"));
    assert!(output.contains("✗ MISS git"));
    assert!(output.contains("Store: /tmp/mgc-store (writable: false)"));
    assert!(!output.contains("Disk Free:"));
    assert!(output.ends_with('\n'));
}

#[test]
fn json_report_keeps_machine_readable_fields() {
    let report = report();
    let json = render_json(&report, None).unwrap();
    let baseline_json = serde_json::to_string_pretty(&report).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(json, baseline_json);
    assert_eq!(value["health_status"], "UNHEALTHY");
    assert_eq!(value["store_writable"], false);
    assert!(value.get("suggested_actions").is_some());
    assert!(value.get("remediation").is_none());
}

#[test]
fn remediation_is_present_in_human_and_top_level_json_output() {
    let result = RemediationOutcome {
        status: RemediationStatus::Created,
        message: "Created and verified store.".to_string(),
        error: None,
    };
    let human = render_human(&report(), Some(&result));
    let json = render_json(&report(), Some(&result)).unwrap();
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert!(human.contains("Remediation [created]: Created and verified store."));
    assert_eq!(value["health_status"], "UNHEALTHY");
    assert_eq!(value["remediation"]["status"], "created");
    assert_eq!(
        value["remediation"]["message"],
        "Created and verified store."
    );
    assert!(value["remediation"].get("error").is_none());
}
