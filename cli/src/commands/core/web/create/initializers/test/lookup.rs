//! Lookup tests: the catalog answers only for frameworks it really supports.
//! Test lookup: catalog chỉ trả lời cho framework nó thật sự hỗ trợ.

use super::super::{Mode, lookup, supported};

#[test]
fn every_catalog_id_is_found_by_lookup() {
    for id in supported(Mode::Frontend) {
        assert!(lookup(Mode::Frontend, id).is_some(), "frontend '{id}'");
    }
    for id in supported(Mode::Backend) {
        assert!(lookup(Mode::Backend, id).is_some(), "backend '{id}'");
    }
}

#[test]
fn an_unknown_framework_has_no_initializer() {
    assert!(lookup(Mode::Frontend, "ember-no-such-framework").is_none());
    assert!(lookup(Mode::Backend, "rails-no-such-framework").is_none());
}

#[test]
fn a_frontend_id_is_not_served_as_a_backend_initializer() {
    // `nextjs` publishes a frontend initializer only; asking for it in backend
    // mode must not silently pick a different program.
    // `nextjs` chỉ phát hành initializer frontend; hỏi nó ở mode backend không
    // được lặng lẽ chọn chương trình khác.
    assert!(lookup(Mode::Frontend, "nextjs").is_some());
    assert!(lookup(Mode::Backend, "nextjs").is_none());
}

#[test]
fn mode_parse_rejects_unknown_scaffold_types() {
    assert_eq!(Mode::parse("frontend"), Some(Mode::Frontend));
    assert_eq!(Mode::parse("fullstack"), Some(Mode::Frontend));
    assert_eq!(Mode::parse("backend"), Some(Mode::Backend));
    assert_eq!(Mode::parse("monorepo"), None);
    assert_eq!(Mode::parse(""), None);
}
