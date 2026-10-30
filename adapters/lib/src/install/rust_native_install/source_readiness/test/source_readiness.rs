//! Sẵn sàng source Cargo sau install — every materialized source must be usable by both readers.
//! Every registry source MagiCore materializes must carry BOTH the verified
//! checksum record and Cargo's own readiness marker; install paths that write
//! only one of them leave a source Cargo silently re-extracts on the next build.
//! Mọi source registry do MagiCore materialize phải có ĐỦ checksum record đã xác
//! minh và marker sẵn sàng của Cargo; đường install chỉ ghi một trong hai sẽ để
//! lại source bị Cargo giải nén lại ở lần build sau.

use super::super::{finalize_source, require_cargo_source_marker, source_root};
use crate::install::rust_native_install::test::{resolved_rust, test_crate_archive};
use mgc_resolver::protocols::{CARGO_SOURCE_MARKER, has_cargo_source_marker};

#[test]
fn finalize_writes_the_checksum_record_and_cargos_marker() {
    let cargo_home = tempfile::tempdir().expect("test operation must succeed");
    let archive = test_crate_archive("mgcfixture", "1.0.0");
    let root = source_root(cargo_home.path(), "mgcfixture", "1.0.0");
    // Materialization already ran in a real flow; this test supplies the
    // extracted files directly so it can isolate the finalize step.
    // (Trong luồng thật, materialize đã chạy; test này cung cấp file đã giải nén
    // trực tiếp để cô lập bước finalize.)
    extract_fixture(&archive, &root, "mgcfixture-1.0.0");
    let package = resolved_rust("mgcfixture", "1.0.0", &archive);
    let sha = package.integrity.trim_start_matches("sha256-").to_string();

    finalize_source(&root, &sha, &archive).expect("finalize must succeed");

    assert!(
        root.join(".cargo-checksum.json").is_file(),
        "the build gate needs the checksum record"
    );
    assert!(
        root.join(CARGO_SOURCE_MARKER).is_file(),
        "cargo needs its own readiness marker"
    );
    assert!(has_cargo_source_marker(&root));
    assert!(require_cargo_source_marker(&root).is_ok());
}

#[test]
fn a_source_without_cargos_marker_is_reported_as_not_materialized() {
    // Cargo re-extracts a directory without `.cargo-ok`, which destroys the
    // checksum record; detecting that state early keeps `mgc build` from showing
    // the downstream "failed to load checksum ... No such file or directory".
    // (Cargo giải nén lại thư mục thiếu `.cargo-ok` và phá checksum record; phát
    // hiện sớm trạng thái đó để `mgc build` không hiện lỗi "failed to load
    // checksum ... No such file or directory".)
    let source = tempfile::tempdir().expect("test operation must succeed");
    std::fs::write(source.path().join("Cargo.toml"), b"[package]\n").expect("test");

    let error = require_cargo_source_marker(source.path())
        .expect_err("an unmarked source must not be trusted");

    assert!(
        error.to_string().contains("not fully materialized"),
        "{error}"
    );
}

#[test]
fn finalize_rejects_a_record_that_does_not_match_the_archive() {
    let cargo_home = tempfile::tempdir().expect("test operation must succeed");
    let archive = test_crate_archive("mgcfixture", "1.0.0");
    let root = source_root(cargo_home.path(), "mgcfixture", "1.0.0");
    extract_fixture(&archive, &root, "mgcfixture-1.0.0");

    let error = finalize_source(&root, &"00".repeat(32), &archive)
        .expect_err("a mismatched archive digest must be rejected");

    assert!(error.to_string().contains("digest"), "{error}");
    assert!(
        !root.join(CARGO_SOURCE_MARKER).exists(),
        "a rejected record must never leave a marker claiming the source is ready"
    );
}

/// Extract the fixture archive under `root` using the same archive reader the
/// production materializer uses.
/// Giải nén archive fixture vào `root` bằng đúng reader mà materializer dùng.
fn extract_fixture(archive: &[u8], root: &std::path::Path, expected_root: &str) {
    let parent = root.parent().expect("test source root has a parent");
    std::fs::create_dir_all(parent).expect("test operation must succeed");
    mgc_resolver::protocols::archive::extract_crate_tar_gz(archive, parent, expected_root)
        .expect("test operation must succeed");
}
