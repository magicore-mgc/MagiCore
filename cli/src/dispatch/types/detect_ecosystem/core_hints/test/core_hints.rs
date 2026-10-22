//! Core-hint normalization and error contracts.

use super::super::resolve;

#[test]
fn canonicalizes_cloud_to_clo() {
    assert_eq!(
        resolve(vec!["Cloud".to_owned()]).unwrap().as_deref(),
        Some("clo")
    );
}

#[test]
fn conflicting_hints_keep_sorted_display_text() {
    let error = resolve(vec!["web".to_owned(), "game".to_owned()]).unwrap_err();
    let expected = concat!(
        "conflicting MagiCore core declarations in project manifests: game, web. ",
        "Set one authoritative `.mgc.core` marker or `mgc.toml` ecosystem ",
        "before running core-aware commands."
    );
    assert_eq!(error.to_string(), expected);
}

#[test]
fn unsupported_hint_keeps_existing_display_text() {
    let error = resolve(vec!["unknown".to_owned()]).unwrap_err();
    assert_eq!(
        error.to_string(),
        "manifest declares unknown MagiCore core 'unknown'; use `.mgc.core` or `mgc.toml` with a supported core"
    );
}
