//! Topological-order error display tests.
//! Test hiển thị lỗi sắp thứ tự topo.

use crate::topo::TopoError;

#[test]
fn topo_error_display_preserves_cycle_path() {
    let error = TopoError::Cycle(vec!["'core'".to_string(), "'web'".to_string()]);

    assert_eq!(
        error.to_string(),
        "workspace dependency cycle: 'core' -> 'web'"
    );
}
