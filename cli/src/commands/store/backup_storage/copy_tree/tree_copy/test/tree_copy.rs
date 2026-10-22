//! Tree entry copy cases.
//! Ca copy entry trong cây.

use super::super::copy_directory;
use std::fs;

#[test]
fn copies_regular_files_into_the_prepared_tree() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("source");
    let destination = temp.path().join("destination");
    fs::create_dir_all(source.join("pkg/nested")).unwrap();
    fs::create_dir(&destination).unwrap();
    fs::write(source.join("pkg/nested/file"), b"package bytes").unwrap();
    let source = fs::canonicalize(source).unwrap();
    let destination = fs::canonicalize(destination).unwrap();

    copy_directory(&source, &destination, &source, &destination).unwrap();

    assert_eq!(
        fs::read(destination.join("pkg/nested/file")).unwrap(),
        b"package bytes"
    );
}
