//! Covers ordinary extraction and linked-parent rejection.
//! Kiểm tra giải nén thường và từ chối parent dạng link.

use super::super::super::test_support::{gzip_tar_file, gzip_tar_link};

#[test]
fn extracts_file_below_destination() {
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("dest");
    let archive = gzip_tar_file("pkg/file.txt", b"content");
    super::super::extract_tarball_from_reader(archive.as_slice(), &dest).unwrap();
    assert_eq!(
        std::fs::read(dest.join("pkg/file.txt")).unwrap(),
        b"content"
    );
    #[cfg(unix)]
    assert_eq!(
        std::fs::metadata(dest.join("pkg/file.txt"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o644
    );
}

#[test]
fn the_last_duplicate_plain_file_entry_wins() {
    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("dest");
    let archive = super::super::super::test_support::gzip_tar_files(&[
        ("pkg/file.txt", b"first"),
        ("pkg/file.txt", b"last"),
    ]);
    super::super::extract_tarball_from_reader(archive.as_slice(), &dest).unwrap();
    assert_eq!(std::fs::read(dest.join("pkg/file.txt")).unwrap(), b"last");
}

#[test]
fn rejects_archive_symlink_and_hard_link_entries() {
    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("dest");

    for kind in [tar::EntryType::Symlink, tar::EntryType::Link] {
        let archive = gzip_tar_link("pkg/link", "../outside", kind);
        let error =
            super::super::extract_tarball_from_reader(archive.as_slice(), &dest).unwrap_err();
        assert!(error.to_string().contains("tar links are not allowed"));
        assert!(!dest.join("pkg/link").exists());
    }
}

#[cfg(unix)]
#[test]
fn refuses_archive_path_through_existing_symlink() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("dest");
    let outside = temp.path().join("outside");
    std::fs::create_dir(&dest).unwrap();
    std::fs::create_dir(&outside).unwrap();
    symlink(&outside, dest.join("redirect")).unwrap();
    let archive = gzip_tar_file("redirect/escaped.txt", b"blocked");
    let error = super::super::extract_tarball_from_reader(archive.as_slice(), &dest).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("refusing linked tar extraction parent")
    );
    assert!(!outside.join("escaped.txt").exists());
}

#[cfg(unix)]
#[test]
fn refuses_archive_file_over_existing_symlink_target() {
    use std::os::unix::fs::symlink;

    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("dest");
    let outside = temp.path().join("outside.txt");
    std::fs::create_dir_all(dest.join("pkg")).unwrap();
    std::fs::write(&outside, b"outside-original").unwrap();
    symlink(&outside, dest.join("pkg/file.txt")).unwrap();
    let archive = gzip_tar_file("pkg/file.txt", b"blocked");

    let error = super::super::extract_tarball_from_reader(archive.as_slice(), &dest).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("refusing tar extraction target symlink")
    );
    assert_eq!(std::fs::read(outside).unwrap(), b"outside-original");
}

#[cfg(any(unix, windows))]
#[test]
fn replaces_preexisting_hard_link_without_modifying_its_other_link() {
    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("dest");
    let external = temp.path().join("external.txt");
    std::fs::create_dir(&dest).unwrap();
    std::fs::write(&external, b"outside-original").unwrap();
    std::fs::hard_link(&external, dest.join("package.txt")).unwrap();
    let archive = gzip_tar_file("package.txt", b"inside-new");
    super::super::extract_tarball_from_reader(archive.as_slice(), &dest).unwrap();
    assert_eq!(std::fs::read(&external).unwrap(), b"outside-original");
    assert_eq!(
        std::fs::read(dest.join("package.txt")).unwrap(),
        b"inside-new"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        assert_ne!(
            std::fs::metadata(&external).unwrap().ino(),
            std::fs::metadata(dest.join("package.txt")).unwrap().ino()
        );
    }
}

#[cfg(windows)]
#[test]
fn applies_archive_readonly_mode_to_extracted_file() {
    use super::super::super::test_support::gzip_tar_file_with_mode;

    let temp = tempfile::tempdir().unwrap();
    let dest = temp.path().join("dest");
    let archive = gzip_tar_file_with_mode("pkg/readonly.txt", b"content", 0o444);
    super::super::extract_tarball_from_reader(archive.as_slice(), &dest).unwrap();
    let target = dest.join("pkg/readonly.txt");
    let metadata = std::fs::metadata(&target).unwrap();
    assert!(metadata.permissions().readonly());
    let mut permissions = metadata.permissions();
    permissions.set_readonly(false);
    std::fs::set_permissions(target, permissions).unwrap();
}
