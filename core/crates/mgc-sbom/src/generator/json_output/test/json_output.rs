#![allow(clippy::unwrap_used)]
//! JSON output owner tests — Kiểm thử owner xuất JSON

use super::super::*;

#[test]
fn json_output_serializes_the_cyclonedx_document() {
    let json = SbomGenerator::default()
        .generate_json(&mgc_lockfile::Lockfile::new())
        .unwrap();

    assert!(json.contains("\"bomFormat\": \"CycloneDX\""));
}
