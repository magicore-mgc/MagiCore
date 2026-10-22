//! Internal links are rebased; links outside the source tree are refused.
//! Link nội bộ được rebase; link ra ngoài source bị từ chối.

use super::super::copy_dir;
use std::fs;
use std::path::Path;

#[test]
fn copies_regular_tree_and_payload() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let destination_parent = temp.path().join("backup");
    let destination = destination_parent.join("vstore");
    fs::create_dir_all(source.join("pkg/nested")).unwrap();
    fs::create_dir_all(&destination_parent).unwrap();
    fs::write(source.join("pkg/nested/file.bin"), b"payload").unwrap();

    copy_dir(&source, &destination).unwrap();

    assert_eq!(
        fs::read(destination.join("pkg/nested/file.bin")).unwrap(),
        b"payload"
    );
}

#[test]
fn refuses_missing_source_without_creating_destination() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("missing-source");
    let destination = temp.path().join("backup/vstore");

    assert!(copy_dir(&source, &destination).is_err());
    assert!(!destination.exists());
}

#[test]
fn refuses_to_merge_into_a_non_empty_destination() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let destination = temp.path().join("backup/vstore");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&destination).unwrap();
    fs::write(destination.join("stale.bin"), b"old snapshot").unwrap();

    assert!(copy_dir(&source, &destination).is_err());
    assert_eq!(
        fs::read(destination.join("stale.bin")).unwrap(),
        b"old snapshot"
    );
}

#[test]
fn refuses_destination_ancestor_of_source_before_writing() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("project/node_modules/.magicore");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("package.bin"), b"package").unwrap();
    let destination = temp.path().join("project");

    assert!(copy_dir(&source, &destination).is_err());
    assert_eq!(fs::read(source.join("package.bin")).unwrap(), b"package");
}

#[cfg(any(unix, windows))]
#[test]
fn rebases_absolute_internal_package_link_to_destination_tree() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("project/node_modules/.magicore");
    let destination_parent = temp.path().join("backup");
    let destination = destination_parent.join("vstore");
    let target = source.join("node_modules/react@18.2.0");
    let link = source.join("node_modules/app@1.0.0/node_modules/react");
    fs::create_dir_all(&target).unwrap();
    fs::create_dir_all(link.parent().unwrap()).unwrap();
    fs::create_dir_all(&destination_parent).unwrap();
    fs::write(target.join("index.js"), b"react").unwrap();
    create_directory_symlink(&target, &link).unwrap();

    copy_dir(&source, &destination).unwrap();

    let copied_target = destination.join("node_modules/react@18.2.0");
    let copied_link = destination.join("node_modules/app@1.0.0/node_modules/react");
    assert!(fs::read_link(&copied_link).unwrap().is_relative());
    assert_eq!(
        fs::canonicalize(copied_link).unwrap(),
        fs::canonicalize(copied_target).unwrap()
    );
}

#[cfg(any(unix, windows))]
#[test]
fn accepts_absolute_internal_link_through_source_path_alias() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let alias = temp.path().join("source-alias");
    let destination = temp.path().join("backup/vstore");
    let target = source.join("target");
    let link = source.join("links/package");
    fs::create_dir_all(&target).unwrap();
    fs::create_dir_all(link.parent().unwrap()).unwrap();
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::write(target.join("package.json"), b"package").unwrap();
    create_directory_symlink(&source, &alias).unwrap();
    create_directory_symlink(&alias.join("target"), &link).unwrap();

    copy_dir(&source, &destination).unwrap();

    let copied_link = destination.join("links/package");
    assert_eq!(
        fs::canonicalize(copied_link).unwrap(),
        fs::canonicalize(destination.join("target")).unwrap()
    );
}

#[cfg(any(unix, windows))]
#[test]
fn rebases_relative_internal_link() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let destination_parent = temp.path().join("backup");
    let destination = destination_parent.join("copy");
    let target = source.join("target");
    let link = source.join("nested/link");
    fs::create_dir_all(&target).unwrap();
    fs::create_dir_all(link.parent().unwrap()).unwrap();
    fs::create_dir_all(&destination_parent).unwrap();
    create_directory_symlink(Path::new("../target"), &link).unwrap();

    copy_dir(&source, &destination).unwrap();

    assert!(
        fs::canonicalize(destination.join("nested/link"))
            .unwrap()
            .is_dir()
    );
    assert_eq!(
        fs::read_link(destination.join("nested/link")).unwrap(),
        Path::new("../target")
    );
}

#[cfg(any(unix, windows))]
#[test]
fn refuses_external_symlink_before_creating_copy_root() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let external = temp.path().join("external");
    let destination = temp.path().join("backup/copy");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&external).unwrap();
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::write(external.join("keep.bin"), b"keep").unwrap();
    create_directory_symlink(&external, &source.join("outside")).unwrap();

    assert!(copy_dir(&source, &destination).is_err());
    assert!(!destination.exists());
    assert_eq!(fs::read(external.join("keep.bin")).unwrap(), b"keep");
}

#[cfg(any(unix, windows))]
#[test]
fn refuses_dangling_symlink() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let destination_parent = temp.path().join("backup");
    fs::create_dir(&source).unwrap();
    fs::create_dir(&destination_parent).unwrap();
    create_directory_symlink(Path::new("missing-target"), &source.join("dangling")).unwrap();

    assert!(copy_dir(&source, &destination_parent.join("copy")).is_err());
}

#[cfg(unix)]
#[test]
fn refuses_symlink_resolution_cycle() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    fs::create_dir(&source).unwrap();
    symlink("second", source.join("first")).unwrap();
    symlink("first", source.join("second")).unwrap();
    let destination = temp.path().join("backup/copy");

    assert!(copy_dir(&source, &destination).is_err());
    assert!(!destination.exists());
}

#[cfg(unix)]
#[test]
fn refuses_link_that_lexically_escapes_then_reenters_source_root() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    fs::create_dir_all(source.join("target")).unwrap();
    fs::create_dir_all(source.join("sub")).unwrap();
    let link_target = Path::new("sub/../../source/target");
    symlink(link_target, source.join("escape-and-return")).unwrap();
    let destination = temp.path().join("backup/copy");

    assert!(copy_dir(&source, &destination).is_err());
    assert!(!destination.exists());
}

#[cfg(unix)]
#[test]
fn refuses_directory_link_to_its_ancestor() {
    use std::os::unix::fs::symlink;
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    fs::create_dir_all(source.join("nested")).unwrap();
    symlink(source.join("nested"), source.join("nested/loop")).unwrap();
    let destination = temp.path().join("backup/copy");

    assert!(copy_dir(&source, &destination).is_err());
    assert!(!destination.exists());
}

#[cfg(any(unix, windows))]
#[test]
fn refuses_symlink_source_and_destination_roots() {
    let temp = tempfile::tempdir().unwrap();
    let real_source = temp.path().join("real-source");
    let source_link = temp.path().join("source-link");
    let destination_parent = temp.path().join("backup");
    let real_destination = temp.path().join("real-destination");
    let destination_link = temp.path().join("destination-link");
    fs::create_dir(&real_source).unwrap();
    fs::create_dir(&destination_parent).unwrap();
    fs::create_dir(&real_destination).unwrap();
    create_directory_symlink(&real_source, &source_link).unwrap();
    create_directory_symlink(&real_destination, &destination_link).unwrap();

    assert!(copy_dir(&source_link, &destination_parent.join("copy")).is_err());
    assert!(copy_dir(&real_source, &destination_link).is_err());
}

#[cfg(unix)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(target, link)
}

#[cfg(windows)]
fn create_directory_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_dir(target, link)
}
