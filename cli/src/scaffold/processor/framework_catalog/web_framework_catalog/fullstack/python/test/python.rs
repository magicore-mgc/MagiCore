//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn python_fullstack_frameworks_keep_no_inferred_backend_base() {
    let rows = [
        REACT_FASTAPI,
        REACT_DJANGO,
        REACT_FLASK,
        VUE_FASTAPI,
        VUE_DJANGO,
        VUE_FLASK,
        SVELTE_FASTAPI,
    ];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec![
            "react-fastapi",
            "react-django",
            "react-flask",
            "vue-fastapi",
            "vue-django",
            "vue-flask",
            "svelte-fastapi",
        ]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "fullstack" && row.base.is_none())
    );
}
