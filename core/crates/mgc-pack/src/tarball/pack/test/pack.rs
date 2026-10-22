//! Tests for archive coordination and output exclusion.
//! Kiểm tra điều phối archive và loại file output.

use super::pack::pack;
use std::fs;

#[test]
fn output_inside_project_is_not_added_to_its_own_archive() {
    let root = tempfile::tempdir().expect("create package fixture directory");
    fs::write(root.path().join("package.json"), "{} ").expect("write package manifest fixture");
    let output = root.path().join("package.tgz");

    let result = pack(root.path(), &output, "package").expect("pack project fixture");
    let archive = fs::File::open(&output).expect("open package archive");
    let mut tarball = tar::Archive::new(flate2::read::GzDecoder::new(archive));
    let names = tarball
        .entries()
        .expect("read archive entries")
        .map(|entry| {
            entry
                .expect("read archive entry")
                .path()
                .expect("read archive entry path")
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();

    assert_eq!(result.entry_count, 1);
    assert_eq!(names, vec![String::from("package/package.json")]);
}
