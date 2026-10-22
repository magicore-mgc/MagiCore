//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn rust_fullstack_frameworks_keep_no_inferred_backend_base() {
    let rows = [
        REACT_AXUM,
        REACT_ACTIX_WEB,
        VUE_AXUM,
        VUE_ACTIX_WEB,
        SVELTE_AXUM,
    ];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec![
            "react-axum",
            "react-actix-web",
            "vue-axum",
            "vue-actix-web",
            "svelte-axum",
        ]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "fullstack" && row.base.is_none())
    );
}
