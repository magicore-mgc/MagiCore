//! Tests for ordered Web global package roots.
//! Test thứ tự root package Web global.

use super::super::implementation::global_package_roots;
use std::path::PathBuf;

#[test]
fn configured_root_precedes_managed_and_user_cache_roots() {
    let roots = global_package_roots(
        Some(PathBuf::from("configured")),
        PathBuf::from("data"),
        Some(PathBuf::from("cache")),
    );

    assert_eq!(
        roots,
        vec![
            PathBuf::from("configured"),
            PathBuf::from("data/cache/web/global/node_modules"),
            PathBuf::from("cache/magicore/global/web/node_modules"),
        ]
    );
}
