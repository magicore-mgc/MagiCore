#![allow(clippy::unwrap_used)]
//! Generator API owner tests — Kiểm thử owner API generator

use super::super::*;

#[test]
fn public_generator_handles_an_empty_legacy_lockfile() {
    let bom = SbomGenerator::default()
        .generate(&mgc_lockfile::Lockfile::new())
        .unwrap();

    assert!(bom.components.is_empty());
    assert!(bom.dependencies.is_none());
}
