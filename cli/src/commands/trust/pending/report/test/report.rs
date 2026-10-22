//! Trust-pending classification tests — test phân loại trust-pending.

use super::*;

#[test]
fn classification_keeps_deny_precedence_and_sorted_output_groups() {
    let scripted = [
        ("gamma".to_string(), "3.0.0".to_string()),
        ("delta".to_string(), "4.0.0".to_string()),
        ("beta".to_string(), "2.0.0".to_string()),
        ("alpha".to_string(), "1.0.0".to_string()),
    ]
    .into_iter()
    .collect();
    let db_policies = [
        ("alpha@1.0.0".to_string(), "approved".to_string()),
        ("beta".to_string(), "denied".to_string()),
        ("delta@4.0.0".to_string(), "approved".to_string()),
    ]
    .into_iter()
    .collect();
    let file_policy = mgc_config::project::ScriptsPolicy {
        policy: None,
        allow: Vec::new(),
        deny: vec!["delta@4.0.0".to_string()],
    };

    let summary = classify_scripted_packages(scripted, &db_policies, Some(&file_policy));

    assert_eq!(summary.approved, vec!["alpha@1.0.0".to_string()]);
    assert_eq!(
        summary.denied,
        vec!["beta@2.0.0".to_string(), "delta@4.0.0".to_string()]
    );
    assert_eq!(summary.pending, vec!["gamma@3.0.0".to_string()]);
    assert_eq!(
        report_text(&summary),
        concat!(
            "Denied (scripts blocked):\n  ✗ beta@2.0.0\n  ✗ delta@4.0.0\n",
            "Approved (scripts will run):\n  ✓ alpha@1.0.0\n",
            "Pending review (scripts SKIPPED on install until approved):\n",
            "  ? gamma@3.0.0 — approve with: mgc trust approve gamma@3.0.0\n"
        )
    );
}

#[test]
fn exact_version_database_policy_precedes_package_name_fallback() {
    let scripted = [("sample".to_string(), "2.0.0".to_string())]
        .into_iter()
        .collect();
    let db_policies = [
        ("sample".to_string(), "approved".to_string()),
        ("sample@2.0.0".to_string(), "denied".to_string()),
    ]
    .into_iter()
    .collect();

    let summary = classify_scripted_packages(scripted, &db_policies, None);

    assert!(summary.approved.is_empty());
    assert_eq!(summary.denied, vec!["sample@2.0.0".to_string()]);
    assert!(summary.pending.is_empty());
    assert_eq!(
        report_text(&summary),
        "Denied (scripts blocked):\n  ✗ sample@2.0.0\nNo pending packages — every installed lifecycle script has a policy.\n"
    );
}

fn report_text(summary: &PendingSummary) -> String {
    let mut output = Vec::new();
    write_report_lines(summary, &mut output).expect("writing to a memory buffer must succeed");
    String::from_utf8(output).expect("report output must be valid UTF-8")
}
