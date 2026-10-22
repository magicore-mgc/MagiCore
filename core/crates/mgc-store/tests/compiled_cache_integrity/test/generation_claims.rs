//! Existing CAS generation-claim lifecycle assertions.
//! Assertion vòng đời claim generation CAS hiện có.

use crate::compiled_cache_integrity::support::tmp_store_dir;

pub(crate) fn staging_claims_survive_other_promote_and_abort_is_scoped() {
    // This test covers generation-claim lifecycle only; it does not invoke cache prune.
    // Test này chỉ bao phủ vòng đời generation claim; không gọi thao tác cache prune.
    let root = tmp_store_dir("prune-db-gate");
    let db_path = root.join("store.db");
    let db = mgc_store::Database::open(&db_path).unwrap();

    // Generation-token flow (P0-A): begin returns the install's TOKEN →
    // claims file into THAT token → promote names the token.
    // (Luồng generation-token: begin trả TOKEN của install → claim rơi
    // vào đúng token đó → promote gọi đúng token.)
    let generation = db.begin_cas_generation("/proj/x").unwrap();
    assert!(generation >= 1);
    db.cas_claim("/proj/x", generation, "hash-a").unwrap();
    db.cas_claim("/proj/x", generation, "hash-b").unwrap();
    assert_eq!(db.list_cas_live_refs().unwrap().len(), 2);

    // A SECOND install generation: old claims stay live (crash safety).
    // (Generation install THỨ HAI: claim cũ vẫn live (an toàn crash).)
    let gen2 = db.begin_cas_generation("/proj/x").unwrap();
    db.cas_claim("/proj/x", gen2, "hash-c").unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert!(
        live.contains(&"hash-a".to_string()),
        "old generation claims must stay live (P0-C)"
    );
    assert!(live.contains(&"hash-c".to_string()));

    // Promote the second token: retires non-staging claims not vouched by
    // it. t1 was NEVER promoted or aborted — it is still 'staging' (an
    // in-flight or crashed install), so ITS claims survive the other
    // token's promote (over-retention is safe; only t1's own abort — or
    // doctor GC — may drop them). This is the P0-A contract: no promote
    // may delete another install's staging claims.
    // (Promote token thứ hai: nghỉ hưu claim không staging mà nó không
    // bảo chứng. t1 CHƯA TỪNG promote hay abort — vẫn 'staging' (install
    // đang chạy hoặc đứt) nên claim của NÓ sống sót qua promote của token
    // khác (giữ thừa an toàn; chỉ abort của chính t1 — hoặc doctor GC —
    // mới được dọn). Đây là hợp đồng P0-A: promote nào cũng KHÔNG được xóa
    // claim staging của install khác.)
    db.promote_cas_generation("/proj/x", gen2).unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert_eq!(
        live,
        vec![
            "hash-a".to_string(),
            "hash-b".to_string(),
            "hash-c".to_string()
        ],
        "a still-staging (crashed/in-flight) generation's claims must survive another token's promote"
    );

    // The crashed install's own ABORT retires its claims — nobody else's.
    // (ABORT của chính install đứt nghỉ hưu claim của nó — không ai khác.)
    db.abort_cas_generation("/proj/x", generation).unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert_eq!(live, vec!["hash-c".to_string()]);

    // Idempotent re-promote is safe.
    // (Re-promote idempotent là an toàn.)
    db.promote_cas_generation("/proj/x", gen2).unwrap();
    assert_eq!(db.list_cas_live_refs().unwrap().len(), 1);
}
