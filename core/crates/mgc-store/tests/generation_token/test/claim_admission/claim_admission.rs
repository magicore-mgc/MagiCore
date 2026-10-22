//! CAS claim admission, batch atomicity and API-level token guards.
//! Điều kiện nhận CAS claim, tính nguyên tử của batch và cổng token tại API.

use crate::generation_token::support::{PROJ_A, PROJ_B, TestDb, assert_orphan_claim_rejected};
use mgc_store::database::CasGenerationError;

pub(crate) fn cas_claim_batch_is_atomic_and_rejects_non_staging_tokens() {
    let db = TestDb::new();
    let db = &db.db;
    let generation = db.begin_cas_generation(PROJ_A).unwrap();

    db.cas_claim_batch(PROJ_A, generation, &["hash-a", "hash-b", "hash-a"])
        .unwrap();
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-a".to_string(), "hash-b".to_string()]
    );

    let error = db
        .cas_claim_batch(PROJ_A, generation + 100, &["hash-forged"])
        .unwrap_err();
    assert!(matches!(error, CasGenerationError::UnknownToken { .. }));
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-a".to_string(), "hash-b".to_string()],
        "invalid token must leave the entire claim set unchanged"
    );

    db.promote_cas_generation(PROJ_A, generation).unwrap();
    let error = db
        .cas_claim_batch(PROJ_A, generation, &["hash-late"])
        .unwrap_err();
    assert!(matches!(error, CasGenerationError::AlreadyPromoted { .. }));
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-a".to_string(), "hash-b".to_string()],
        "promoted tokens must not accept late claims"
    );
}

pub(crate) fn cas_claim_batch_reports_failing_insert_and_rolls_back_prior_claims() {
    let db = TestDb::new();
    let generation = db.db.begin_cas_generation(PROJ_A).unwrap();
    db.db
        .conn_for_tests()
        .execute_batch(
            "CREATE TRIGGER reject_claim_for_test
             BEFORE INSERT ON cas_blob_refs
             WHEN NEW.hash = 'hash-reject'
             BEGIN
                 SELECT RAISE(ABORT, 'injected claim failure');
             END;",
        )
        .unwrap();

    let error = db
        .db
        .cas_claim_batch(
            PROJ_A,
            generation,
            &["hash-first", "hash-reject", "hash-last"],
        )
        .unwrap_err();
    assert!(
        matches!(error, CasGenerationError::Io(ref message)
            if message.contains("insert claim 2/3")
                && message.contains("injected claim failure")),
        "claim batch must identify the operation and item that failed: {error}"
    );
    let claim_count: i64 = db
        .db
        .conn_for_tests()
        .query_row(
            "SELECT COUNT(*) FROM cas_blob_refs
             WHERE project_root = ?1 AND generation = ?2",
            rusqlite::params![PROJ_A, generation],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(
        claim_count, 0,
        "failed claim batches must roll back all rows"
    );
}

pub(crate) fn claim_rejects_forged_token() {
    let db = TestDb::new();
    let db = &db.db;

    let err = db.cas_claim(PROJ_A, 999_999, "hash-x").unwrap_err();
    assert!(
        matches!(err, CasGenerationError::UnknownToken { .. }),
        "claim with a forged token must be UnknownToken, got {err:?}"
    );
    assert!(db.list_cas_live_refs().unwrap().is_empty());
}

pub(crate) fn claim_rejects_cross_project_token() {
    // A token minted for project B must not claim for project A — the
    // v2 API accepted it because nothing scoped (project, generation)
    // pairs together.
    // (Token đúc cho project B không được claim cho project A — API v2
    // nhận nó vì không gì ràng buộc cặp (project, generation).)
    let db = TestDb::new();
    let db = &db.db;

    let t_b = db.begin_cas_generation(PROJ_B).unwrap();
    let err = db.cas_claim(PROJ_A, t_b, "hash-x").unwrap_err();
    assert!(
        matches!(err, CasGenerationError::UnknownToken { .. }),
        "cross-project claim must be UnknownToken, got {err:?}"
    );
    assert!(db.list_cas_live_refs().unwrap().is_empty());
}

pub(crate) fn claim_rejects_promoted_token() {
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    db.promote_cas_generation(PROJ_A, t0).unwrap();

    let err = db.cas_claim(PROJ_A, t0, "hash-b").unwrap_err();
    assert!(
        matches!(err, CasGenerationError::AlreadyPromoted { .. }),
        "claim into a promoted token must be AlreadyPromoted, got {err:?}"
    );
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-a".to_string()],
        "the promoted generation keeps exactly its pre-promote claims"
    );
}

pub(crate) fn aborted_generation_rejects_claim_via_api() {
    // This covers the public API gate after abort; it does not issue raw SQL to assert the FK.
    // Test này kiểm tra cổng API sau abort; không chạy raw SQL để xác nhận foreign key.
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    db.abort_cas_generation(PROJ_A, t0).unwrap();

    // The marker is gone; the API must reject a later claim as UnknownToken.
    // (Marker đã mất; API phải từ chối claim tiếp theo bằng UnknownToken.)
    assert!(matches!(
        db.cas_claim(PROJ_A, t0, "hash-z").unwrap_err(),
        CasGenerationError::UnknownToken { .. }
    ));
}

pub(crate) fn foreign_key_rejects_orphan_claim_in_fresh_schema() {
    let db = TestDb::new();
    assert_orphan_claim_rejected(&db.db, PROJ_A, 777);
}
