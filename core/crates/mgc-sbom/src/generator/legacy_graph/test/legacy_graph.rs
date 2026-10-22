#![allow(clippy::unwrap_used)]
//! Legacy graph owner tests — Kiểm thử owner graph legacy

use super::super::*;

#[test]
fn empty_legacy_lockfile_has_no_components_or_edges() {
    let bom = generate(&mgc_lockfile::Lockfile::new(), false).unwrap();

    assert!(bom.components.is_empty());
    assert!(bom.dependencies.is_none());
}
