use crate::allowlist::QUALIFIED_COMPAT_RUNNERS;

#[test]
fn only_currently_qualified_javascript_managers_are_catalogued() {
    assert_eq!(QUALIFIED_COMPAT_RUNNERS, &["npm", "pnpm", "yarn", "bun"]);
}
