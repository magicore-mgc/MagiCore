//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn php_backend_frameworks_keep_their_language_base() {
    let rows = [LARAVEL, SYMFONY];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec!["laravel", "symfony",]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "backend" && row.base == Some("php"))
    );
}
