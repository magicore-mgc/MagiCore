//! Trạng thái sẵn sàng source Cargo — repairing a legacy store must keep the trust bar of install.
//! A source directory is only given Cargo's readiness marker after its checksum
//! record has been re-verified against the cached archive and the extracted
//! files; anything else is left for `mgc install` to rebuild.
//! Thư mục source chỉ được gắn marker sẵn sàng của Cargo sau khi checksum record
//! được xác minh lại với archive đã cache và file đã giải nén; mọi trường hợp
//! khác để `mgc install` dựng lại.

use super::super::{RepairOutcome, repair_package, split_package_dir};

const MARKER: &str = mgc_resolver::protocols::CARGO_SOURCE_MARKER;

/// Build a `.crate` archive with one `Cargo.toml`, the shape install accepts.
/// Tạo archive `.crate` chỉ có `Cargo.toml`, đúng hình dạng install chấp nhận.
fn crate_archive(name: &str, version: &str) -> Vec<u8> {
    let manifest = format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\n");
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut archive = tar::Builder::new(encoder);
    let mut header = tar::Header::new_gnu();
    header.set_size(manifest.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive
        .append_data(
            &mut header,
            format!("{name}-{version}/Cargo.toml"),
            manifest.as_bytes(),
        )
        .expect("test operation must succeed");
    archive
        .into_inner()
        .expect("test operation must succeed")
        .finish()
        .expect("test operation must succeed")
}

/// Materialize the archive the same way install does, but stop before the
/// marker so the legacy state can be reproduced.
/// Materialize archive đúng như install nhưng dừng trước bước marker để tái tạo
/// trạng thái legacy.
fn legacy_materialized_source(root: &std::path::Path, name: &str, version: &str, archive: &[u8]) {
    std::fs::create_dir_all(root).expect("test operation must succeed");
    mgc_resolver::protocols::archive::extract_crate_tar_gz(
        archive,
        root,
        &format!("{name}-{version}"),
    )
    .expect("test operation must succeed");
    let package_root = root.join(format!("{name}-{version}"));
    let sha = {
        use sha2::Digest as _;
        let mut digest = sha2::Sha256::new();
        digest.update(archive);
        hex::encode(digest.finalize())
    };
    let files = mgc_resolver::protocols::archive::cargo_source_file_hashes(
        archive,
        &format!("{name}-{version}"),
    )
    .expect("test operation must succeed");
    // Recompute the record without Cargo's marker, exactly what older releases
    // left on disk. (Ghi record không kèm marker Cargo, đúng thứ bản cũ để lại.)
    std::fs::write(
        package_root.join(".cargo-checksum.json"),
        serde_json::to_vec(&serde_json::json!({"files": files, "package": sha}))
            .expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
}

#[test]
fn a_legacy_source_without_cargos_marker_is_repaired() {
    let registry = tempfile::tempdir().expect("test operation must succeed");
    let src = registry.path().join("registry/src");
    let archive = crate_archive("icu_segmenter", "2.4.0");
    legacy_materialized_source(&src, "icu_segmenter", "2.4.0", &archive);
    let package_root = src.join("icu_segmenter-2.4.0");
    assert!(!package_root.join(MARKER).exists());

    let outcome = repair_package(&package_root, "icu_segmenter", "2.4.0", &archive)
        .expect("a verified legacy source must be repairable");

    assert_eq!(outcome, RepairOutcome::Repaired);
    assert!(package_root.join(MARKER).is_file());
    assert!(
        package_root.join(".cargo-checksum.json").is_file(),
        "repair must not drop the checksum record the build gate reads"
    );
}

#[test]
fn a_source_whose_checksum_record_does_not_match_the_archive_is_not_marked() {
    // A tampered tree must never gain a marker that claims MagiCore vetted it.
    // (Cây bị sửa không bao giờ được gắn marker khẳng định MagiCore đã xác minh.)
    let registry = tempfile::tempdir().expect("test operation must succeed");
    let archive = crate_archive("icu_segmenter", "2.4.0");
    legacy_materialized_source(
        &registry.path().join("src"),
        "icu_segmenter",
        "2.4.0",
        &archive,
    );
    let package_root = registry.path().join("src/icu_segmenter-2.4.0");
    std::fs::write(
        package_root.join("Cargo.toml"),
        b"[package]\nname = \"other\"\n",
    )
    .expect("test operation must succeed");

    let error = repair_package(&package_root, "icu_segmenter", "2.4.0", &archive)
        .expect_err("a tampered source must be refused");

    assert!(
        error.to_string().contains("failed re-verification"),
        "{error}"
    );
    assert!(!package_root.join(MARKER).exists());
}

#[test]
fn a_source_without_a_checksum_record_is_left_to_install() {
    let registry = tempfile::tempdir().expect("test operation must succeed");
    let archive = crate_archive("icu_segmenter", "2.4.0");
    std::fs::create_dir_all(registry.path().join("icu_segmenter-2.4.0"))
        .expect("test operation must succeed");
    let package_root = registry.path().join("icu_segmenter-2.4.0");

    let outcome = repair_package(&package_root, "icu_segmenter", "2.4.0", &archive)
        .expect("an unmanaged directory is skipped, not an error");

    assert_eq!(outcome, RepairOutcome::Ready);
    assert!(!package_root.join(MARKER).exists());
}

#[test]
fn an_already_marked_source_is_ready_and_untouched() {
    let registry = tempfile::tempdir().expect("test operation must succeed");
    let archive = crate_archive("icu_segmenter", "2.4.0");
    legacy_materialized_source(
        &registry.path().join("src"),
        "icu_segmenter",
        "2.4.0",
        &archive,
    );
    let package_root = registry.path().join("src/icu_segmenter-2.4.0");
    mgc_resolver::protocols::write_cargo_source_marker(&package_root)
        .expect("test operation must succeed");

    let outcome = repair_package(&package_root, "icu_segmenter", "2.4.0", &archive)
        .expect("a marked source needs no work");

    assert_eq!(outcome, RepairOutcome::Ready);
}

#[test]
fn package_directory_names_split_at_the_version_boundary() {
    assert_eq!(
        split_package_dir("icu_segmenter-2.4.0"),
        Some(("icu_segmenter", "2.4.0"))
    );
    assert_eq!(
        split_package_dir("rustls-webpki-0.103.1"),
        Some(("rustls-webpki", "0.103.1"))
    );
    assert_eq!(split_package_dir("no-version"), None);
}
