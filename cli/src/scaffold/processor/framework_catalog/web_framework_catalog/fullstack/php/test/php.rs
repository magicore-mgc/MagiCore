//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn php_fullstack_frameworks_keep_no_inferred_backend_base() {
    let rows = [VUE_LARAVEL, REACT_SYMFONY, VUE_SYMFONY];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec!["vue-laravel", "react-symfony", "vue-symfony",]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "fullstack" && row.base.is_none())
    );
}
