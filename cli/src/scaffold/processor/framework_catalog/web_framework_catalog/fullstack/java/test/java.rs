//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn java_fullstack_frameworks_keep_no_inferred_backend_base() {
    let rows = [REACT_SPRING, REACT_QUARKUS, VUE_QUARKUS, SVELTE_QUARKUS];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec![
            "react-spring",
            "react-quarkus",
            "vue-quarkus",
            "svelte-quarkus",
        ]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "fullstack" && row.base.is_none())
    );
}
