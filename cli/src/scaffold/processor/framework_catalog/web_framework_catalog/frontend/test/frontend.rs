//! Tests for the grouped framework configuration rows.
//! Test các row cấu hình framework đã được nhóm.

use super::*;

#[test]
fn frontend_frameworks_keep_their_current_metadata() {
    let rows = [
        VANILLA, REACT_VITE, VUE_VITE, NEXTJS, SVELTEKIT, NUXT, ANGULAR, SOLIDJS, QWIK, ASTRO,
    ];
    assert_eq!(
        rows.iter().map(|row| row.name).collect::<Vec<_>>(),
        vec![
            "vanilla",
            "react-vite",
            "vue-vite",
            "nextjs",
            "sveltekit",
            "nuxt",
            "angular",
            "solidjs",
            "qwik",
            "astro",
        ]
    );
    assert!(
        rows.iter()
            .all(|row| row.sub_type == "frontend" && row.base.is_none())
    );
}
