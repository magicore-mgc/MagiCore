//! Covers deterministic ledger output and unknown-status rejection.
//! Kiểm tra output ledger xác định và từ chối status lạ.

use super::super::load_ownership_ledger;

#[test]
fn sorts_conflicting_owner_records_independently_of_input_order() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let first_path = temp.path().join("first.json");
    let second_path = temp.path().join("second.json");
    let allowed = serde_json::json!({
        "file": "adapters/web/src/allowed.rs",
        "tool": "cargo",
        "status": "allowed"
    });
    let delegated = serde_json::json!({
        "file": "adapters/web/src/delegated.rs",
        "tool": "cargo",
        "status": "delegated-documented"
    });
    let first = serde_json::json!({
        "generated_at": "2026-10-17T00:00:00Z",
        "findings": [allowed.clone(), delegated.clone()]
    });
    let second = serde_json::json!({
        "generated_at": "2026-10-17T00:00:00Z",
        "findings": [delegated, allowed]
    });
    std::fs::write(
        &first_path,
        serde_json::to_vec(&first).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
    std::fs::write(
        &second_path,
        serde_json::to_vec(&second).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");

    let first_entries = load_ownership_ledger(&first_path).expect("test operation must succeed");
    let second_entries = load_ownership_ledger(&second_path).expect("test operation must succeed");
    assert_eq!(first_entries, second_entries);
    assert_eq!(
        first_entries
            .iter()
            .map(|entry| entry.owner.as_str())
            .collect::<Vec<_>>(),
        vec!["delegated", "mgc-native"]
    );
}

#[test]
fn rejects_unknown_ledger_status() {
    let temp = tempfile::tempdir().expect("test operation must succeed");
    let path = temp.path().join("ledger.json");
    std::fs::write(
        &path,
        br#"{"generated_at":"2026-10-17T00:00:00Z","findings":[{"file":"adapters/web/src/lib.rs","tool":"cargo","status":"unknown"}]}"#,
    )
    .expect("test operation must succeed");

    let error = load_ownership_ledger(&path).expect_err("test operation must fail");
    assert!(error.to_string().contains("refusing to certify ownership"));
}
