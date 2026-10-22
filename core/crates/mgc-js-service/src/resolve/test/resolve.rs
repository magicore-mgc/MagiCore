//! The former public helper path remains reachable through the facade.
//! Path helper public cũ vẫn truy cập được qua facade.

use super::super::is_dist_tag_spec;

#[test]
fn stable_resolve_route_reaches_exported_helper() {
    assert!(is_dist_tag_spec("latest"));
}
