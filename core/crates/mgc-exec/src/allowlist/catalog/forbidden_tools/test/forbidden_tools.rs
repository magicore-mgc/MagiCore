use crate::allowlist::{ALLOWED_TOOLS, FORBIDDEN_TOOLS};
#[test]
fn forbidden_catalog_covers_external_managers_without_allow_overlap() {
    for manager in ["npm", "npx", "pnpm", "yarn", "bun", "pip", "uv"] {
        assert!(FORBIDDEN_TOOLS.contains(&manager));
    }
    assert!(
        FORBIDDEN_TOOLS
            .iter()
            .all(|tool| !ALLOWED_TOOLS.contains(tool))
    );
}
