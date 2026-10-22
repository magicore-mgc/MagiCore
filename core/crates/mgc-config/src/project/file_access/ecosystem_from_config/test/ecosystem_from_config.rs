//! Behavior tests for top-level TOML ecosystem parsing.
//! Test hành vi parse ecosystem TOML cấp cao nhất.
use super::super::ecosystem_from_config;

#[test]
fn absent_content_or_ecosystem_returns_none() {
    assert_eq!(ecosystem_from_config(None).unwrap(), None);
    assert_eq!(
        ecosystem_from_config(Some("name = 'sample'\n")).unwrap(),
        None
    );
}

#[test]
fn top_level_ecosystem_is_returned_unchanged() {
    assert_eq!(
        ecosystem_from_config(Some("ecosystem = 'clo'\n"))
            .unwrap()
            .as_deref(),
        Some("clo")
    );
}

#[test]
fn malformed_toml_preserves_its_source_chain() {
    let error =
        ecosystem_from_config(Some("ecosystem = [\n")).expect_err("malformed TOML must fail");

    assert!(std::error::Error::source(AsRef::<dyn std::error::Error>::as_ref(&error)).is_some());
}
