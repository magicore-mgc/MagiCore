//! npm alias parsing behavior / hành vi parse npm alias.

use crate::provider::NpmDependencyProvider;

#[test]
fn parses_scoped_aliases_and_rejects_incomplete_specs() {
    assert_eq!(
        NpmDependencyProvider::parse_alias_spec("npm:@scope/pkg@^1.2.0"),
        Some(("@scope/pkg".to_string(), "^1.2.0".to_string()))
    );
    assert_eq!(NpmDependencyProvider::parse_alias_spec("npm:react"), None);
    assert_eq!(NpmDependencyProvider::parse_alias_spec("npm:react@"), None);
}
