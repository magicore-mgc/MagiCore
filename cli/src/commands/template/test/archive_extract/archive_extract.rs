use std::fs;
use tempfile;

use super::super::*;

#[test]
fn valid_archive_replaces_cache_and_strips_the_package_prefix() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let target = root.join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("old.txt"), b"old").unwrap();

    extract_tarball_into(&valid_archive(), &target, None).unwrap();

    assert!(target.join("template.toml").is_file());
    assert_eq!(
        fs::read(target.join("sources/main.txt")).unwrap(),
        b"new content"
    );
    assert!(!target.join("package").exists());
    assert!(!target.join("old.txt").exists());
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
}

#[test]
fn traversal_archive_preserves_the_previous_cache() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let target = root.join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("old.txt"), b"preserve").unwrap();
    let malicious = archive(&[
        (
            "package/template.toml",
            b"valid first entry",
            tar::EntryType::Regular,
        ),
        ("package/../../escaped", b"attack", tar::EntryType::Regular),
    ]);

    assert!(extract_tarball_into(&malicious, &target, None).is_err());
    assert_eq!(fs::read(target.join("old.txt")).unwrap(), b"preserve");
    assert!(!target.join("template.toml").exists());
    assert!(!root.join("escaped").exists());
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
}

#[test]
fn invalid_manifest_does_not_replace_the_previous_cache() {
    let root = tempfile::tempdir().unwrap();
    let root = root.path().canonicalize().unwrap();
    let target = root.join("layer");
    fs::create_dir(&target).unwrap();
    fs::write(target.join("sentinel"), b"preserve").unwrap();
    let malformed = archive(&[
        (
            "package/template.toml",
            b"not = [valid",
            tar::EntryType::Regular,
        ),
        (
            "package/sources/main.txt",
            b"new content",
            tar::EntryType::Regular,
        ),
    ]);
    assert!(extract_tarball_into(&malformed, &target, None).is_err());
    assert_eq!(fs::read(target.join("sentinel")).unwrap(), b"preserve");
    assert_eq!(fs::read_dir(&target).unwrap().count(), 1);
}

#[test]
fn archive_links_and_multiple_roots_do_not_publish_a_cache() {
    for malicious in [
        archive(&[("package/sources/link", b"", tar::EntryType::Symlink)]),
        archive(&[("package/sources/link", b"", tar::EntryType::Link)]),
        archive(&[
            ("one/template.toml", b"one", tar::EntryType::Regular),
            ("two/template.toml", b"two", tar::EntryType::Regular),
        ]),
    ] {
        let root = tempfile::tempdir().unwrap();
        let root = root.path().canonicalize().unwrap();
        let target = root.join("layer");
        assert!(extract_tarball_into(&malicious, &target, None).is_err());
        assert!(!target.exists());
        assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
    }
}
