//! Temporary keyring-file lifecycle facade — Facade vòng đời temporary keyring.
//! Wire the unpublished-file cleanup guard — Nối guard dọn file chưa publish.

mod temporary_keyring_file;
pub(super) use temporary_keyring_file::TemporaryKeyringFile;

#[cfg(test)]
#[path = "test/temporary_keyring_file.rs"]
mod tests;
