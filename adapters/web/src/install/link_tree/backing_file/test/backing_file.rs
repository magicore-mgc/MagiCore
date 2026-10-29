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

/// P-2.5: when copy-on-write is unavailable (the Windows case) every
/// materialized file must be a real copy — its own inode, `nlink == 1` — so an
/// edit in one project can never reach another project or the package cache.
/// (P-2.5: khi không có copy-on-write (trường hợp Windows) mọi file được
/// materialize phải là bản copy thật — inode riêng, `nlink == 1` — để sửa ở
/// một project không bao giờ chạm project khác hay cache package.)
#[cfg(unix)]
#[test]
fn reflink_disabled_materialization_uses_a_private_copy() {
    use std::os::unix::fs::MetadataExt;

    let dir = tempdir_real().unwrap();
    let source = dir.path().join("cache-blob");
    std::fs::write(&source, b"shared payload").unwrap();
    let target = dir.path().join("project/node_modules/pkg/blob");

    // `reflink_enabled = false` is exactly the path Windows takes when the
    // filesystem reports NotSupported.
    // (`reflink_enabled = false` đúng là đường Windows đi khi filesystem báo
    // NotSupported.)
    backing_link_file(&source, &target, None, false).unwrap();

    assert_eq!(std::fs::read(&target).unwrap(), b"shared payload");
    let source_meta = std::fs::metadata(&source).unwrap();
    let target_meta = std::fs::metadata(&target).unwrap();
    assert_eq!(
        target_meta.nlink(),
        1,
        "a copy fallback must not create a second link to the shared blob"
    );
    assert_ne!(
        (source_meta.dev(), source_meta.ino()),
        (target_meta.dev(), target_meta.ino()),
        "the project copy must be its own inode"
    );

    // Editing the project copy leaves the cached blob untouched.
    // (Sửa bản trong project không đụng blob trong cache.)
    std::fs::write(&target, b"edited in project").unwrap();
    assert_eq!(std::fs::read(&source).unwrap(), b"shared payload");
}
