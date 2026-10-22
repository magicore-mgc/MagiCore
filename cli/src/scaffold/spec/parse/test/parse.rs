//! Behavior tests for scaffold request parsing.
//! Test hành vi cho parse yêu cầu scaffold.

use super::super::parse_scaffold_spec;
use crate::scaffold::spec::{CoreKind, ScaffoldRef};

#[test]
fn parses_default_request_and_normalizes_name() {
    let parsed = parse_scaffold_spec(CoreKind::App, "React_Native").unwrap();

    assert_eq!(parsed.core, CoreKind::App);
    assert_eq!(parsed.name, "React_Native");
    assert_eq!(parsed.normalized_name, "react-native");
    assert_eq!(parsed.requested_ref, ScaffoldRef::Default);
}

#[test]
fn classifies_numeric_tag_and_range_references_by_existing_precedence() {
    assert_eq!(
        parse_scaffold_spec(CoreKind::Web, "react@18.3.0")
            .unwrap()
            .requested_ref,
        ScaffoldRef::Version("18.3.0".to_string())
    );
    assert_eq!(
        parse_scaffold_spec(CoreKind::Web, "react@beta")
            .unwrap()
            .requested_ref,
        ScaffoldRef::DistTag("beta".to_string())
    );
    assert_eq!(
        parse_scaffold_spec(CoreKind::Web, "react@^1.0.0")
            .unwrap_err()
            .to_string(),
        "Version ranges not supported yet"
    );
}

#[test]
fn rejects_empty_name_and_preserves_the_existing_error_text() {
    assert_eq!(
        parse_scaffold_spec(CoreKind::Web, "@latest")
            .unwrap_err()
            .to_string(),
        "Template name cannot be empty"
    );
}

#[test]
fn empty_reference_uses_default() {
    assert_eq!(
        parse_scaffold_spec(CoreKind::Lib, "rust@")
            .unwrap()
            .requested_ref,
        ScaffoldRef::Default
    );
}
