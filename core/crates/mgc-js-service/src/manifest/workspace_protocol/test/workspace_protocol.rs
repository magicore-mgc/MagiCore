//! Workspace protocol range classification tests.
//! Test phân loại workspace protocol range.

use super::super::is_workspace_protocol_range;

#[test]
fn recognizes_workspace_ranges_and_rejects_registry_ranges() {
    for range in [
        "workspace:*",
        "workspace:^",
        "workspace:~",
        " workspace:1.2.3 ",
    ] {
        assert!(
            is_workspace_protocol_range(range),
            "expected workspace range: {range}"
        );
    }
    for range in ["*", "^1.2.3", "npm:workspace:*"] {
        assert!(
            !is_workspace_protocol_range(range),
            "expected registry range: {range}"
        );
    }
}
