//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn go_fullstack_frameworks_keep_no_inferred_backend_base() {
    let rows = [
        REACT_GIN,
        REACT_ECHO,
        REACT_FIBER,
        VUE_GIN,
        VUE_ECHO,
        VUE_FIBER,
        SVELTE_GIN,
    ];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec![
            "react-gin",
            "react-echo",
            "react-fiber",
            "vue-gin",
            "vue-echo",
            "vue-fiber",
            "svelte-gin",
        ]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "fullstack" && row.base.is_none())
    );
}
