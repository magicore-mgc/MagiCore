//! Read-only CAS integrity count API behavior.
//! Hành vi API đếm toàn vẹn CAS chỉ đọc.

use crate::generation_token::support::TestDb;
use mgc_store::CasIntegrityCounts;

pub(crate) fn cas_integrity_snapshot_counts_corrupt_rows() {
    let db = TestDb::new();
    let conn = db.db.conn_for_tests();
    conn.pragma_update(None, "foreign_keys", false).unwrap();
    conn.execute_batch(
        "INSERT INTO cas_generations (project_root, generation, state)
             VALUES ('/proj/integrity', 1, 'promoted');
         INSERT INTO cas_generations (project_root, generation, state)
             VALUES ('/proj/integrity', 2, 'promoted');
         INSERT INTO cas_blob_refs (project_root, generation, hash)
             VALUES ('/proj/orphan', 404, 'hash-orphan');",
    )
    .unwrap();

    assert_eq!(
        db.db.cas_integrity_counts().unwrap(),
        CasIntegrityCounts {
            multi_promoted_generations: 2,
            orphan_claims: 1,
        }
    );
}
