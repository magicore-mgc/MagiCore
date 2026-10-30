//! Registry source materialization always leaves a Cargo-usable source tree.
//! Materialize source registry luôn để lại cây source dùng được với Cargo.

use super::materialize::materialize;
use super::source_readiness;
use crate::protocols::ResolvedEntry;
use crate::protocols::archive::test_support::tar_gz_entries;

fn fixture_entry() -> ResolvedEntry {
    ResolvedEntry {
        name: "mgcfixture".to_string(),
        version: "1.0.0".to_string(),
        artifact_url: "https://example.invalid/mgcfixture-1.0.0.crate".to_string(),
        deps: Vec::new(),
        sha256: String::new(),
        extra_markers: Vec::new(),
    }
}

fn fixture_archive() -> Vec<u8> {
    tar_gz_entries(&[
        (
            "mgcfixture-1.0.0/Cargo.toml",
            b"[package]\nname = \"mgcfixture\"\n",
        ),
        (
            "mgcfixture-1.0.0/src/lib.rs",
            b"pub fn value() -> u8 { 7 }\n",
        ),
    ])
}

#[test]
fn materialize_writes_the_cargo_readiness_marker_next_to_the_source() {
    let cargo_home = tempfile::tempdir().expect("test operation must succeed");
    let source_root = cargo_home.path().join("registry/src/mgcfixture-1.0.0");

    materialize(&fixture_entry(), &fixture_archive(), cargo_home.path())
        .expect("test operation must succeed");

    assert!(source_root.join("Cargo.toml").is_file());
    assert!(
        source_root
            .join(source_readiness::CARGO_SOURCE_MARKER)
            .is_file(),
        "a materialized source must carry Cargo's readiness marker"
    );
    assert!(source_readiness::has_cargo_source_marker(&source_root));
}

#[test]
fn materialize_rebuilds_a_stale_unmarked_source_tree() {
    // A crashed install can leave files behind without the marker. Cargo would
    // re-extract such a directory itself and then fail on the missing checksum
    // record, so materialization must replace the tree instead of merging.
    // (Install bị ngắt có thể để lại file mà không có marker. Cargo sẽ tự giải
    // nén lại thư mục đó rồi lỗi vì thiếu checksum, nên materialize phải thay
    // cây thay vì hòa vào.)
    let cargo_home = tempfile::tempdir().expect("test operation must succeed");
    let source_root = cargo_home.path().join("registry/src/mgcfixture-1.0.0");
    std::fs::create_dir_all(source_root.join("src")).expect("test operation must succeed");
    std::fs::write(source_root.join("Cargo.toml"), b"stale manifest").expect("test");
    std::fs::write(source_root.join(".cargo-checksum.json"), b"{}").expect("test");

    materialize(&fixture_entry(), &fixture_archive(), cargo_home.path())
        .expect("test operation must succeed");

    assert!(
        std::fs::read(source_root.join("Cargo.toml"))
            .expect("test operation must succeed")
            .starts_with(b"[package]"),
        "the stale tree must be replaced by the verified archive"
    );
    assert!(
        !source_root.join(".cargo-checksum.json").exists(),
        "materialize must not carry a stale checksum record into the new tree"
    );
    assert!(source_readiness::has_cargo_source_marker(&source_root));
}

#[test]
fn materialize_refuses_to_replace_a_symlinked_source_directory() {
    #[cfg(unix)]
    {
        let cargo_home = tempfile::tempdir().expect("test operation must succeed");
        let outside = tempfile::tempdir().expect("test operation must succeed");
        let registry_src = cargo_home.path().join("registry/src");
        std::fs::create_dir_all(&registry_src).expect("test operation must succeed");
        std::os::unix::fs::symlink(outside.path(), registry_src.join("mgcfixture-1.0.0"))
            .expect("test operation must succeed");

        let error = materialize(&fixture_entry(), &fixture_archive(), cargo_home.path())
            .expect_err("a symlinked source directory must be refused");

        assert!(error.to_string().contains("refusing to replace"));
        assert!(outside.path().read_dir().expect("test").next().is_none());
    }
}

#[test]
fn readiness_marker_rejects_links_named_like_the_marker() {
    #[cfg(unix)]
    {
        let source_root = tempfile::tempdir().expect("test operation must succeed");
        let outside = source_root.path().join("outside.json");
        std::fs::write(&outside, b"{\"v\":1}").expect("test");
        std::os::unix::fs::symlink(
            &outside,
            source_root
                .path()
                .join(source_readiness::CARGO_SOURCE_MARKER),
        )
        .expect("test operation must succeed");

        assert!(
            !source_readiness::has_cargo_source_marker(source_root.path()),
            "a link is not Cargo's readiness marker"
        );
        assert!(source_readiness::write_cargo_source_marker(source_root.path()).is_err());
    }
}

#[test]
fn readiness_marker_round_trips_and_matches_cargo_current_version() {
    let source_root = tempfile::tempdir().expect("test operation must succeed");
    assert!(!source_readiness::has_cargo_source_marker(
        source_root.path()
    ));

    source_readiness::write_cargo_source_marker(source_root.path())
        .expect("test operation must succeed");

    assert_eq!(
        std::fs::read_to_string(
            source_root
                .path()
                .join(source_readiness::CARGO_SOURCE_MARKER)
        )
        .expect("test operation must succeed"),
        "{\"v\":1}"
    );
    assert!(source_readiness::has_cargo_source_marker(
        source_root.path()
    ));
}
