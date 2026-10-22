//! Safety and project isolation of promoted-claim coverage queries.
//! An toàn và cô lập project của truy vấn claim được promoted che phủ.

use crate::generation_token::support::{PROJ_A, PROJ_B, TestDb};

pub(crate) fn covered_by_promoted_names_only_safe_claimful_staging() {
    let db = TestDb::new();
    let db = &db.db;

    // Promoted baseline claims hash-a.
    // (Baseline promoted claim hash-a.)
    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    db.promote_cas_generation(PROJ_A, t0).unwrap();

    // A CRASHED staging (dead lease, has claims): all its hashes are
    // covered by the promoted baseline → SAFE to retire.
    // (Staging ĐỨT (lease chết, có claim): mọi hash được baseline
    // promoted giữ → AN TOÀN để nghỉ hưu.)
    let t_crash = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t_crash, "hash-a").unwrap(); // covered
    db.cas_claim(PROJ_A, t_crash, "hash-b").unwrap(); // uncovered

    // Sanity: with hash-b uncovered, the crashed staging must NOT be
    // named (hash-b would lose its only protection).
    // (Kiểm tra: còn hash-b chưa được giữ, staging đứt KHÔNG được gọi
    // tên (hash-b sẽ mất bảo vệ duy nhất).)
    let covered = db.list_staging_leases_covered_by_promoted().unwrap();
    assert!(
        covered.iter().all(|l| l.generation != t_crash),
        "staging with an UNCOVERED claim (hash-b) must not be named for retirement"
    );

    // Promote a new generation that also claims hash-b — now the
    // crashed staging's claims are FULLY covered.
    // (Promote generation mới cũng claim hash-b — giờ claim của staging
    // đứt được phủ ĐẦY ĐỦ.)
    let t_next = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t_next, "hash-a").unwrap();
    db.cas_claim(PROJ_A, t_next, "hash-b").unwrap();
    db.promote_cas_generation(PROJ_A, t_next).unwrap();

    let covered = db.list_staging_leases_covered_by_promoted().unwrap();
    assert_eq!(covered.len(), 1, "exactly the crashed staging is named");
    assert_eq!(covered[0].generation, t_crash);
    assert_eq!(covered[0].claim_count, 2);
    assert_eq!(covered[0].project_root, PROJ_A);

    // Retiring it through the TOKEN-GATED abort keeps every live ref:
    // the promoted generation still claims hash-a and hash-b.
    // (Nghỉ hưu qua abort CÓ CỔNG TOKEN giữ mọi ref sống: generation
    // promoted vẫn claim hash-a và hash-b.)
    db.abort_cas_generation(PROJ_A, t_crash).unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert_eq!(
        live,
        vec!["hash-a".to_string(), "hash-b".to_string()],
        "retiring the covered staging must not lose any live ref"
    );
}

pub(crate) fn covered_by_promoted_is_project_scoped() {
    let db = TestDb::new();
    let db = &db.db;

    let t_b = db.begin_cas_generation(PROJ_B).unwrap();
    db.cas_claim(PROJ_B, t_b, "hash-shared").unwrap();
    db.promote_cas_generation(PROJ_B, t_b).unwrap();

    // Crashed A-staging claiming the same hash string — B's promotion
    // does NOT cover it.
    // (Staging A đứt claim cùng chuỗi hash — promotion của B KHÔNG phủ
    // nó.)
    let t_a = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t_a, "hash-shared").unwrap();

    let covered = db.list_staging_leases_covered_by_promoted().unwrap();
    assert!(
        covered.iter().all(|l| l.project_root != PROJ_A),
        "project A's staging must not be retired on project B's coverage"
    );
}

pub(crate) fn covered_by_promoted_carries_lease_fields() {
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    db.promote_cas_generation(PROJ_A, t0).unwrap();

    let t_crash = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t_crash, "hash-a").unwrap();

    let covered = db.list_staging_leases_covered_by_promoted().unwrap();
    let row = covered
        .iter()
        .find(|l| l.generation == t_crash)
        .expect("the covered crashed staging must be named");
    assert!(row.lease_pid.is_some(), "lease pid rides along for the GC");
    assert!(row.lease_started_at.is_some());
    assert_eq!(row.claim_count, 1);
}
