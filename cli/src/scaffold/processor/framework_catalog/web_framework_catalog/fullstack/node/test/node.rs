//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn node_fullstack_frameworks_keep_no_inferred_backend_base() {
    let rows = [
        REACT_FASTIFY,
        REACT_EXPRESS,
        REACT_HONO,
        REACT_NESTJS,
        REACT_TRPC,
        VUE_EXPRESS,
        VUE_HONO,
        VUE_NESTJS,
        SVELTE_EXPRESS,
        SVELTE_HONO,
    ];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec![
            "react-fastify",
            "react-express",
            "react-hono",
            "react-nestjs",
            "react-trpc",
            "vue-express",
            "vue-hono",
            "vue-nestjs",
            "svelte-express",
            "svelte-hono",
        ]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "fullstack" && row.base.is_none())
    );
}
