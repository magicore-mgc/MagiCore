//! Trust-pending behavior tests — test hành vi của trust pending.

use super::*;
#[test]
fn unreadable_trust_database_is_not_reported_as_empty_policy_set() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let blocker = temp.path().join("not-a-directory");
    std::fs::write(&blocker, "file blocks database parent")
        .expect("database parent blocker must be written");
    let db_path = blocker.join("trust.db");

    let result = load_db_policies(&db_path);

    assert!(
        result.is_err(),
        "an unreadable trust database must fail instead of appearing empty"
    );
}

#[test]
fn readable_trust_database_returns_saved_policies() {
    let temp = tempfile::tempdir().expect("temporary directory must be created");
    let db_path = temp.path().join("trust.db");
    let db = mgc_store::Database::open(&db_path).expect("trust database must open");
    db.upsert_trust_policy("sample-package@1.2.3", "denied")
        .expect("trust policy must be saved");
    drop(db);

    let policies = load_db_policies(&db_path).expect("saved policy must be readable");

    assert_eq!(
        policies.get("sample-package@1.2.3").map(String::as_str),
        Some("denied")
    );
}

#[test]
fn malformed_file_policy_is_not_reported_as_absent() {
    let temp = tempfile::tempdir().expect("temporary project must be created");
    std::fs::write(temp.path().join("mgc.toml"), "[scripts]\nallow = true\n")
        .expect("malformed script policy must be written");

    let result = super::load_file_policy(temp.path());

    assert!(
        result.is_err(),
        "a malformed committed trust policy must not be treated as absent"
    );
}
