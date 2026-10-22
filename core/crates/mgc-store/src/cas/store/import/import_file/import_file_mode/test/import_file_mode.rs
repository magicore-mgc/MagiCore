//! Tests temporary staging cleanup when a stream fails after writing bytes.
//! Kiểm thử dọn staging tạm khi stream lỗi sau khi đã ghi dữ liệu.

use std::io::{self, Read};

use super::super::import_streamed_reader;
use crate::cas::store::{ContentStore, StoreError};

struct FailsAfterPrefix(bool);

impl Read for FailsAfterPrefix {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.0 {
            return Err(io::Error::other("injected stream read failure"));
        }
        self.0 = true;
        let prefix = b"partial stream";
        let count = prefix.len().min(buffer.len());
        buffer[..count].copy_from_slice(&prefix[..count]);
        Ok(count)
    }
}

#[test]
fn removes_staging_file_after_stream_read_failure() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("cas");
    let store = ContentStore::new(root.clone()).unwrap();

    let result = import_streamed_reader(&store, FailsAfterPrefix(false), false, false, false);

    assert!(matches!(result, Err(StoreError::Io { .. })));
    let staged_files = std::fs::read_dir(root.join("tmp"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    assert!(
        staged_files.is_empty(),
        "partial staging files remained: {staged_files:?}"
    );
}
