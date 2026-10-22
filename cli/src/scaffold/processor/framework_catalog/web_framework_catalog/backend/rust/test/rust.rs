//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn rust_backend_frameworks_keep_their_language_base() {
    let rows = [AXUM, ACTIX_WEB];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec!["axum", "actix-web",]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "backend" && row.base == Some("rust"))
    );
}
