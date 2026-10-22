//! Validates an import source and selects its bounded I/O path.
//! Xác minh file nguồn và chọn luồng I/O có giới hạn.

use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

use super::super::super::super::error::StoreError;
use super::super::super::super::types::ContentStore;
use crate::cas::integrity::IntegrityHash;
use crate::cas::write::{STREAM_THRESHOLD, stream_write_verify_and_set_perms_mode};

pub(in crate::cas::store::import::import_file) fn import_file_mode(
    store: &ContentStore,
    source: &Path,
    executable: bool,
    sync_file_data: bool,
    sync_parent_directory: bool,
) -> Result<IntegrityHash, StoreError> {
    if source.is_symlink() {
        return Err(StoreError::Io {
            path: source.to_path_buf(),
            msg: "source path is a symlink".to_string(),
        });
    }
    let metadata = fs::metadata(source)?;
    if !metadata.is_file() {
        return Err(StoreError::Io {
            path: source.to_path_buf(),
            msg: "source path is not a regular file".to_string(),
        });
    }

    if metadata.len() >= STREAM_THRESHOLD as u64 {
        return import_streamed_file(
            store,
            source,
            executable,
            sync_file_data,
            sync_parent_directory,
        );
    }
    let data = fs::read(source)?;
    let hash = IntegrityHash::from_bytes(&data, executable);
    store.write_generated_bytes_with_hash_mode(
        &data,
        &hash,
        executable,
        sync_parent_directory,
        sync_file_data,
    )?;
    Ok(hash)
}

fn import_streamed_file(
    store: &ContentStore,
    source: &Path,
    executable: bool,
    sync_file_data: bool,
    sync_parent_directory: bool,
) -> Result<IntegrityHash, StoreError> {
    let input = fs::File::open(source)?;
    import_streamed_reader(
        store,
        BufReader::new(input),
        executable,
        sync_file_data,
        sync_parent_directory,
    )
}

fn import_streamed_reader(
    store: &ContentStore,
    input: impl Read,
    executable: bool,
    sync_file_data: bool,
    sync_parent_directory: bool,
) -> Result<IntegrityHash, StoreError> {
    use crate::cas::write::sync_parent_dir;
    use mgc_platform::atomic_replace::atomic_replace_file_with_write_through;

    let temp = store.tmp_path("import-file");
    let writer = fs::File::create_new(&temp)?;
    let mut staged_file = StagedImportFile::new(temp.clone());
    let hash =
        stream_write_verify_and_set_perms_mode(writer, &temp, input, executable, sync_file_data)?;
    let _publication_guard = super::super::super::import_lock::lock_digest(&hash);
    let destination = hash.cas_path(&store.root);
    if destination.exists() {
        match store.verify_stored_blob(&hash) {
            Ok(()) => {
                staged_file.remove()?;
                return Ok(hash);
            }
            Err(StoreError::HashMismatch { .. }) => {}
            Err(error) => return Err(error),
        }
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Err(source) = atomic_replace_file_with_write_through(&temp, &destination, sync_file_data)
    {
        return Err(StoreError::Io {
            path: destination,
            msg: format!("move streamed file into CAS failed: {}", source.io_source()),
        });
    }
    staged_file.disarm();
    if sync_parent_directory && let Some(parent) = destination.parent() {
        sync_parent_dir(parent)?;
    }
    Ok(hash)
}

struct StagedImportFile {
    path: std::path::PathBuf,
    remove_on_drop: bool,
}

impl StagedImportFile {
    fn new(path: std::path::PathBuf) -> Self {
        Self {
            path,
            remove_on_drop: true,
        }
    }

    fn disarm(&mut self) {
        self.remove_on_drop = false;
    }

    fn remove(&mut self) -> std::io::Result<()> {
        fs::remove_file(&self.path)?;
        self.remove_on_drop = false;
        Ok(())
    }
}

impl Drop for StagedImportFile {
    fn drop(&mut self) {
        if self.remove_on_drop {
            let _ = fs::remove_file(&self.path);
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
