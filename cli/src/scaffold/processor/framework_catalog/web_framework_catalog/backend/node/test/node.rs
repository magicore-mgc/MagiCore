//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn node_backend_frameworks_keep_their_language_base() {
    let rows = [EXPRESS, FASTIFY, NESTJS, HONO, TRPC];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec!["express", "fastify", "nestjs", "hono", "trpc",]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "backend" && row.base == Some("node"))
    );
}
