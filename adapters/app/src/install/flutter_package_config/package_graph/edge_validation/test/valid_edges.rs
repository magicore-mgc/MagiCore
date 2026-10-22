//! Verify all present package/root edges pass.
//! Kiểm tra mọi edge package/root hiện diện đều được nhận.

use super::*;

#[test]
fn resolved_package_and_root_edges_are_accepted() {
    let graph = entries(&[("library", &[]), ("consumer", &["library"])]);

    assert!(
        super::validate_package_edges(&graph, &["consumer".to_string()], &["library".to_string()])
            .is_ok()
    );
}
