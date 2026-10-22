//! Temporary keyring-file cleanup guard — Guard cleanup file keyring tạm.
//! Drop cleans temp unless publication disarms the guard — Drop dọn temp nếu guard chưa disarm.

use std::fs;
use std::path::PathBuf;

pub(in crate::keyring::storage) struct TemporaryKeyringFile {
    path: PathBuf,
    armed: bool,
}

impl TemporaryKeyringFile {
    pub(in crate::keyring::storage) fn new(path: PathBuf) -> Self {
        Self { path, armed: true }
    }

    pub(in crate::keyring::storage) fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for TemporaryKeyringFile {
    fn drop(&mut self) {
        if self.armed {
            let _ = fs::remove_file(&self.path);
        }
    }
}
