use crate::allowlist::{ALLOWED_TOOLS, FORBIDDEN_TOOLS};

#[test]
fn allowed_catalog_has_expected_tools_and_no_forbidden_overlap() {
    assert!(
        ALLOWED_TOOLS
            .iter()
            .all(|tool| !FORBIDDEN_TOOLS.contains(tool))
    );
    for expected in ["cargo", "python", "node", "docker", "flutter"] {
        assert!(ALLOWED_TOOLS.contains(&expected));
    }
}
