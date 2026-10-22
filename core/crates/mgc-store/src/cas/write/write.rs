//! Writes CAS bytes or streams while producing the integrity value.
//! Ghi bytes hoặc stream CAS đồng thời tạo giá trị integrity.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use super::error::{CasWriteOperation, map_io_error};
use super::permissions::set_permissions;
use crate::cas::integrity::IntegrityHash;
use crate::cas::store::StoreError;

/// Files larger than this threshold will be streamed through a buffered reader
/// instead of being fully loaded into memory.
/// File lớn hơn ngưỡng này được stream qua buffered reader thay vì load hết
/// vào memory.
pub const STREAM_THRESHOLD: usize = 1024 * 1024; // 1 MB

/// Write data to file, fsync, and set permissions. The caller writes to a
/// unique temp path and renames into place — the fsync makes the rename
/// durable so a crash cannot publish a torn CAS blob.
///
/// Ghi data vào file, fsync rồi đặt permission. Caller ghi vào temp duy nhất
/// và rename vào vị trí cuối — fsync giúp rename bền vững, crash không thể
/// công bố blob CAS nửa vời.
pub fn write_all_verify_and_set_perms(
    writer: fs::File,
    dest: &Path,
    data: &[u8],
    executable: bool,
) -> Result<IntegrityHash, StoreError> {
    write_all_verify_and_set_perms_mode(writer, dest, data, executable, true)
}

pub(in crate::cas) fn write_all_verify_and_set_perms_mode(
    mut writer: fs::File,
    dest: &Path,
    data: &[u8],
    executable: bool,
    sync_file_data: bool,
) -> Result<IntegrityHash, StoreError> {
    let expected = IntegrityHash::from_bytes(data, executable);

    writer
        .write_all(data)
        .map_err(|error| map_io_error(dest, CasWriteOperation::Write, error))?;
    let flush_result = if sync_file_data {
        writer.flush().and_then(|_| writer.sync_all())
    } else {
        writer.flush()
    };
    let operation = if sync_file_data {
        CasWriteOperation::FlushAndSync
    } else {
        CasWriteOperation::Flush
    };
    flush_result.map_err(|error| map_io_error(dest, operation, error))?;

    set_permissions(dest, executable)?;

    Ok(expected)
}

/// Stream data from reader to writer while computing the content hash.
/// Durability (fsync) is the caller's contract via `finalize_streamed_write`.
/// Stream data từ reader sang writer trong lúc tính hash. Độ bền (fsync) do
/// caller đảm nhận qua `finalize_streamed_write`.
pub fn stream_write_verify_and_set_perms(
    writer: fs::File,
    dest: &Path,
    reader: impl Read,
    executable: bool,
) -> Result<IntegrityHash, StoreError> {
    stream_write_verify_and_set_perms_mode(writer, dest, reader, executable, true)
}

pub(in crate::cas) fn stream_write_verify_and_set_perms_mode(
    mut writer: fs::File,
    dest: &Path,
    mut reader: impl Read,
    executable: bool,
    sync_file_data: bool,
) -> Result<IntegrityHash, StoreError> {
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; 65536];

    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|error| map_io_error(dest, CasWriteOperation::Read, error))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        writer
            .write_all(&buf[..n])
            .map_err(|error| map_io_error(dest, CasWriteOperation::Write, error))?;
    }

    let hash = hasher.finalize().to_hex().to_string();
    // Private-field invariant (P0-1): construct through the validating path.
    // (Invariant field private (P0-1): dựng qua đường có validate.)
    let integrity = IntegrityHash::from_hash_trusted(&hash, executable)
        .map_err(|e| StoreError::InvalidHash(e.input))?;

    let flush_result = if sync_file_data {
        writer.flush().and_then(|_| writer.sync_all())
    } else {
        writer.flush()
    };
    let operation = if sync_file_data {
        CasWriteOperation::FlushAndSync
    } else {
        CasWriteOperation::Flush
    };
    flush_result.map_err(|error| map_io_error(dest, operation, error))?;
    set_permissions(dest, executable)?;

    Ok(integrity)
}
