//! Staging lease metadata and legacy rows without leases.
//! Metadata lease staging và row cũ không có lease.

use crate::generation_token::support::{PROJ_A, TestDb};
use mgc_store::Database;

pub(crate) fn staging_leases_report_pid_and_age() {
    // The lease columns reach the API: every staging marker carries the
    // BEGINNING process's pid + timestamp, and the claim count rides
    // along — the doctor's GC needs all three to tell garbage from a
    // live install.
    // (Cột lease tới được API: mọi marker staging mang pid + timestamp
    // của process bắt đầu, kèm số claim — GC của doctor cần cả ba để
    // phân biệt rác với install đang chạy.)
    let db = TestDb::new();
    let db = &db.db;

    let t0 = db.begin_cas_generation(PROJ_A).unwrap();
    db.cas_claim(PROJ_A, t0, "hash-a").unwrap();
    let t1 = db.begin_cas_generation(PROJ_A).unwrap(); // claim-less begin

    let leases = db.list_staging_leases().unwrap();
    assert_eq!(leases.len(), 2, "both staging generations report leases");
    let by_token = |g: i64| leases.iter().find(|l| l.generation == g).unwrap();
    let l0 = by_token(t0);
    assert_eq!(l0.claim_count, 1);
    assert_eq!(l0.lease_pid, Some(std::process::id() as i64));
    assert!(l0.lease_started_at.is_some());
    let l1 = by_token(t1);
    assert_eq!(l1.claim_count, 0, "the second begin filed no claims yet");
    assert!(l1.lease_pid.is_some());

    // Promote retires the marker — the lease list shrinks.
    // (Promote nghỉ hưu marker — danh sách lease ngắn lại.)
    db.promote_cas_generation(PROJ_A, t0).unwrap();
    let leases = db.list_staging_leases().unwrap();
    assert_eq!(leases.len(), 1);
    assert_eq!(leases[0].generation, t1);
}

pub(crate) fn legacy_staging_without_lease_still_lists() {
    // Markers created BEFORE the lease columns exist (v2/v3 store) have
    // NULL lease fields — the GC must treat them as claim-count-only
    // (the old conservative contract), never panic on the NULLs.
    // (Marker tạo TRƯỚC khi cột lease tồn tại (store v2/v3) có trường
    // lease NULL — GC phải xử lý chúng chỉ theo số claim (hợp đồng bảo
    // toàn cũ), không bao giờ panic trên NULL.)
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("store.db");
    let db = Database::open(&db_path).unwrap();
    let conn = db.conn_for_tests();
    // Hand-insert a staging marker WITHOUT lease columns (the legacy
    // shape) — bypassing begin's lease stamp on purpose.
    // (Tự chèn marker staging KHÔNG qua lease — cố tình bypass phần
    // đóng dấu lease của begin.)
    conn.execute(
        "INSERT INTO cas_generations (project_root, generation, state)
         VALUES ('/proj/legacy-lease', 7, 'staging')",
        [],
    )
    .unwrap();
    let leases = db.list_staging_leases().unwrap();
    assert_eq!(leases.len(), 1);
    assert_eq!(leases[0].lease_pid, None, "legacy marker: NULL lease pid");
    assert_eq!(leases[0].lease_started_at, None);
    assert_eq!(leases[0].claim_count, 0);
}
