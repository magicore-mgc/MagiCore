//! Manifest round-trip tests for the shared Python native service.
//! Test vòng lặp manifest cho service Python native dùng chung.
//!
//! These tests pin the canonical PEP 621 ownership predicate and the
//! pyproject parse/write behavior at its new owner (moved verbatim from
//! `mgc-lib-adapter` in S09a). The lib facades delegate here, so this suite is
//! the nearest-owner proof; lib's own suites cover the facade path.
//! Các test này ghim predicate ownership PEP 621 và behavior parse/write
//! pyproject tại owner mới (chuyển nguyên văn từ `mgc-lib-adapter` trong
//! S09a). Facade lib ủy quyền về đây nên đây là proof gần owner nhất; suite
//! của lib phủ đường facade.

use mgc_python_service::manifest::{
    parse_pyproject_manifest, supports_native_python_project, write_pyproject_manifest,
};

fn real_root(dir: &tempfile::TempDir) -> std::path::PathBuf {
    dir.path()
        .canonicalize()
        .expect("canonicalize temp project root")
}

#[test]
fn pyproject_ownership_predicate_and_manifest_round_trip() {
    let dir = tempfile::tempdir().expect("temp project");
    let root = real_root(&dir);
    std::fs::write(
        root.join("pyproject.toml"),
        "[project]\nname=\"demo\"\ndependencies=[\"requests>=2\"]\n[project.optional-dependencies]\ntest=[\"pytest>=8\"]\nmodel=[\"torch>=2\"]\n",
    )
    .expect("write pyproject.toml");
    assert!(supports_native_python_project(&root));

    let manifest = parse_pyproject_manifest(&root).expect("parse pyproject.toml");
    assert!(manifest.find_dep("requests").is_some());
    assert!(manifest.find_dep("pytest").is_none());
    assert!(manifest.find_dep("torch").is_none());

    write_pyproject_manifest(&root, &manifest).expect("rewrite pyproject.toml");
    let rewritten =
        std::fs::read_to_string(root.join("pyproject.toml")).expect("read rewritten pyproject");
    assert!(
        rewritten.contains("pytest>=8"),
        "shared-service writes must preserve extras metadata: {rewritten}"
    );
}
