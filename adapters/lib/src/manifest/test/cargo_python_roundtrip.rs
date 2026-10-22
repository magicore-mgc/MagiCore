//! Cargo and Python manifest round-trip tests.
//! Test round-trip manifest Cargo và Python.

use super::support::*;

/// pyproject round-trip (C0 FIX2): star deps serialize as the bare PEP 508
/// name (never dropped), `==` pins stay exact (never widened to `>=`),
/// `>=` bounds survive untouched. A hand-written `["six"]` must survive
/// parse → write → parse — dropping it faked a manifest mutation.
/// (Round-trip pyproject: `*` thành tên trần, `==` giữ exact.)
#[test]
fn pyproject_star_and_pins_round_trip_losslessly() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"rt\"\ndependencies = [\"six\", \"attrs==23.1.0\", \"req>=2.0\"]\n",
    )
    .unwrap();
    let manifest = parse_pyproject_manifest(dir.path()).unwrap();
    assert!(manifest.find_dep("six").unwrap().range.is_star());
    write_pyproject_manifest(dir.path(), &manifest).unwrap();
    let body = std::fs::read_to_string(dir.path().join("pyproject.toml")).unwrap();
    assert!(
        body.contains("\"six\""),
        "star must persist as bare name: {body}"
    );
    assert!(
        body.contains("attrs==23.1.0"),
        "exact pin must stay ==: {body}"
    );
    assert!(
        body.contains("req>=2.0"),
        "lower bound must survive: {body}"
    );
    let again = parse_pyproject_manifest(dir.path()).unwrap();
    assert!(again.find_dep("six").is_some());
    assert!(again.find_dep("attrs").is_some());
    assert!(again.find_dep("req").is_some());
}
/// Cargo round-trip (C0 FIX2): `*` is valid Cargo — it serializes back as
/// `*` instead of vanishing on the next add.
/// (Round-trip Cargo: `*` giữ lại, không mất.)
#[test]
fn cargo_star_round_trips_instead_of_vanishing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"rt\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nserde = \"*\"\n",
    )
    .unwrap();
    let manifest = parse_cargo_manifest(dir.path()).unwrap();
    assert!(manifest.find_dep("serde").unwrap().range.is_star());
    write_cargo_manifest(dir.path(), &manifest).unwrap();
    let body = std::fs::read_to_string(dir.path().join("Cargo.toml")).unwrap();
    assert!(body.contains("serde"), "star dep must persist: {body}");
    let again = parse_cargo_manifest(dir.path()).unwrap();
    assert!(again.find_dep("serde").is_some(), "star dep must re-parse");
}
