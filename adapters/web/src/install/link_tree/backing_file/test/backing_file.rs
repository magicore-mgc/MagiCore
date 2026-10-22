//! Verify reflink-disabled isolation and stale-target replacement.
//! Kiểm tra cô lập khi tắt reflink và thay target cũ.

use crate::install::link_tree::backing_link_file;
use crate::profile::MaterializationProfile;

fn tempdir_real() -> std::io::Result<tempfile::TempDir> {
    let canonical = std::env::temp_dir().canonicalize()?;
    tempfile::tempdir_in(canonical)
}

#[test]
fn test_backing_link_fallback_isolated_when_reflink_disabled() {
    let temp = tempdir_real().unwrap();
    let source = temp.path().join("source.txt");
    let target = temp.path().join("target.txt");
    std::fs::write(&source, b"payload-123").unwrap();

    let profile = MaterializationProfile::default();
    backing_link_file(&source, &target, Some(&profile), false).unwrap();

    assert!(target.exists());
    assert_eq!(std::fs::read(&target).unwrap(), b"payload-123");

    // Materialized dependency files must not alias the shared extracted cache.
    // File dependency đã materialize không được dùng chung inode với cache.
    std::fs::write(&source, b"updated-through-source").unwrap();
    assert_eq!(
        std::fs::read(&target).unwrap(),
        b"payload-123",
        "a source mutation must not alter the materialized project copy"
    );

    std::fs::write(&target, b"project-local-edit").unwrap();
    assert_eq!(
        std::fs::read(&source).unwrap(),
        b"updated-through-source",
        "a project-local edit must not poison the shared extracted cache"
    );
}

#[test]
fn test_backing_link_rematerializes_stale_target() {
    let temp = tempdir_real().unwrap();
    let source = temp.path().join("source.txt");
    let target = temp.path().join("target.txt");
    std::fs::write(&source, b"fresh-content").unwrap();
    std::fs::write(&target, b"stale-content").unwrap();

    let profile = MaterializationProfile::default();
    backing_link_file(&source, &target, Some(&profile), true).unwrap();

    assert_eq!(std::fs::read(&target).unwrap(), b"fresh-content");
}
