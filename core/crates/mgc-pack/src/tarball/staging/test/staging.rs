//! Tests for staged output publication — test publish output staging.

use super::{create_staged_output, publish_staged_output};
use std::io::Write;

#[test]
fn completed_stage_replaces_the_previous_output() {
    let directory = tempfile::tempdir().expect("create archive output directory");
    let output = directory.path().join("archive.tgz");
    std::fs::write(&output, b"old archive").expect("write previous archive");
    let mut staged = create_staged_output(directory.path()).expect("create staged archive output");
    staged
        .as_file_mut()
        .write_all(b"new archive")
        .expect("write staged archive");

    publish_staged_output(staged, &output).expect("publish staged archive");

    assert_eq!(
        std::fs::read(output).expect("read published archive"),
        b"new archive"
    );
}
