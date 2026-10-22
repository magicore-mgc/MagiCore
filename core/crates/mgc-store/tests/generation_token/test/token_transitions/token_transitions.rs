//! Generation promote and abort transition contracts.
//! Hợp đồng chuyển trạng thái generation khi promote và abort.

use crate::generation_token::support::{PROJ_A, TestDb};
use mgc_store::database::CasGenerationError;

pub(crate) fn promote_unknown_token_is_zero_mutation() {
    // The round-10 code ran its DELETEs before checking the token, so
    // promote(project, 999999) retired every promoted generation older
    // than the forged number and still returned Ok(()).
    // (Code vòng-10 chạy DELETE trước khi check token, nên
    // promote(project, 999999) nghỉ hưu mọi generation promoted cũ hơn
    // con số giả rồi vẫn Ok(()).)
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    db.cas_claim(PROJ_A, t0, "hash-b").unwrap();
    db.promote_cas_generation(PROJ_A, t0).unwrap();
    let live_before = db.list_cas_live_refs().unwrap();

    let err = db.promote_cas_generation(PROJ_A, 999_999).unwrap_err();
    assert!(
        matches!(err, CasGenerationError::UnknownToken { .. }),
        "forged promote must be a typed UnknownToken error, got {err:?}"
    );

    let live_after = db.list_cas_live_refs().unwrap();
    assert_eq!(
        live_before, live_after,
        "P0-2: promote with an unknown token must retire NOTHING (zero mutation)"
    );
}

pub(crate) fn promote_promoted_token_is_verified_noop() {
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    db.promote_cas_generation(PROJ_A, t0).unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert_eq!(live, vec!["hash-a".to_string()]);

    // Re-promote of the same token: PROVEN no-op (state checked first).
    // (Re-promote cùng token: no-op ĐƯỢC KIỂM CHỨNG — state check trước.)
    db.promote_cas_generation(PROJ_A, t0).unwrap();
    assert_eq!(db.list_cas_live_refs().unwrap(), vec!["hash-a".to_string()]);
}

pub(crate) fn abort_promoted_token_is_hard_error_claims_intact() {
    // The round-10 abort deleted the claims of ANY generation named —
    // aborting a PROMOTED token erased the live refset while its marker
    // stayed, and prune then ate blobs the project was using.
    // (Abort vòng-10 xóa claim của BẤT KỲ generation nào được nêu —
    // abort token ĐÃ PROMOTE xóa refset sống trong khi marker còn, rồi
    // prune ăn blob project đang dùng.)
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-live").unwrap();
    db.promote_cas_generation(PROJ_A, t0).unwrap();

    let err = db.abort_cas_generation(PROJ_A, t0).unwrap_err();
    assert!(
        matches!(err, CasGenerationError::AlreadyPromoted { .. }),
        "abort of a promoted token must be a typed AlreadyPromoted error, got {err:?}"
    );
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-live".to_string()],
        "P0-3: abort of a promoted token must keep every live claim"
    );
}

pub(crate) fn abort_unknown_token_is_idempotent_noop() {
    let db = TestDb::new();
    let db = &db.db;
    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();

    // Missing token: no-op (crash-restart retried the abort).
    // (Token mất: no-op — restart sau crash chạy lại abort.)
    db.abort_cas_generation(PROJ_A, 424_242).unwrap();
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-a".to_string()],
        "abort of an unknown token must touch nothing"
    );
}

pub(crate) fn crashed_staging_claims_survive_other_promote_v3() {
    // The v3 gates must NOT regress the v2 crash-safety contract: a
    // crashed staging install's claims stay live across another token's
    // promote; only the crashed token's own abort retires them.
    // (Cổng v3 không được làm hỏng hợp đồng an toàn crash v2: claim của
    // install staging đứt sống qua promote của token khác; chỉ abort của
    // chính token đứt mới nghỉ hưu chúng.)
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    db.promote_cas_generation(PROJ_A, t0).unwrap();

    let t_crash = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t_crash, "hash-z").unwrap();

    let t_next = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t_next, "hash-a").unwrap();
    db.cas_claim(PROJ_A, t_next, "hash-c").unwrap();
    db.promote_cas_generation(PROJ_A, t_next).unwrap();

    let live = db.list_cas_live_refs().unwrap();
    assert_eq!(
        live,
        vec![
            "hash-a".to_string(),
            "hash-c".to_string(),
            "hash-z".to_string()
        ],
        "crashed staging claims survive another promote (P0-A contract)"
    );

    db.abort_cas_generation(PROJ_A, t_crash).unwrap();
    assert_eq!(
        db.list_cas_live_refs().unwrap(),
        vec!["hash-a".to_string(), "hash-c".to_string()],
        "the crashed token's own abort retires exactly its claims"
    );
}
