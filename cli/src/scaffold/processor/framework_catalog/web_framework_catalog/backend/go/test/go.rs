//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn go_backend_frameworks_keep_their_language_base() {
    let rows = [GIN, ECHO, FIBER];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec!["gin", "echo", "fiber",]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "backend" && row.base == Some("go"))
    );
}
