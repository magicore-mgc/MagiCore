//! Stage Export owner.
//! Phụ trách stage export trong luồng export CAS.

use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom};
use std::path::Path;

use crate::cas::store::StagingKind;

use super::super::cow_clone::try_cow_clone_from_file;
use super::error::StageExportError;

/// Stage export bytes: descriptor-based platform COW clone, then handle copy.
/// Returns WHICH strategy ran so tests can prove the fast path is live
/// (and callers/tests can detect unsupported filesystems).
///
/// COW clones are INDEPENDENT files (copy-on-write) — a later mutation of
/// the export rehydrates a private block, it never writes through into the
/// CAS blob (the P0-1 hardlink-poisoning property is preserved).
/// NOTE (audit vòng-3 P0-2): macOS `fclonefileat` requires the destination
/// to NOT exist; Linux FICLONE requires the destination to BE a freshly-created
/// EMPTY file (open O_CREAT|O_EXCL). Both create-or-fail semantics are
/// handled per platform; fallback copying uses the same open source handle
/// and creates its destination exclusively.
///
/// Staging bytes export: clone COW theo platform trước, copy thường dự
/// phòng. Trả về chiến lược ĐÃ CHẠY để test chứng minh fast path còn sống
/// (và caller/test phát hiện filesystem không hỗ trợ).
/// Clone COW là file ĐỘC LẬP (copy-on-write) — sau này sửa file export sẽ
/// rehydrate block riêng, không bao giờ ghi ngược vào blob CAS (giữ tính
/// chống đầu độc hardlink của P0-1). LƯU Ý (P0-2 audit vòng-3):
/// `fclonefileat` yêu cầu đích CHƯA tồn tại; FICLONE Linux yêu cầu đích là
/// file rỗng MỚI TẠO (open O_CREAT|O_EXCL). Mỗi platform tự xử lý semantics
/// create-hoặc-lỗi; fallback đọc cùng handle nguồn và tạo đích bằng
/// create_new.
pub(in crate::cas::store::export) fn stage_export_bytes(
    source: &File,
    tmp: &Path,
) -> Result<StagingKind, StageExportError> {
    match try_cow_clone_from_file(source, tmp) {
        Some(()) => Ok(StagingKind::CowClone),
        None => {
            let mut input = source
                .try_clone()
                .map_err(|error| StageExportError::copy(tmp, error))?;
            input
                .seek(SeekFrom::Start(0))
                .map_err(|error| StageExportError::copy(tmp, error))?;
            let mut output = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(tmp)
                .map_err(|error| StageExportError::copy(tmp, error))?;
            io::copy(&mut input, &mut output)
                .map_err(|error| StageExportError::copy(tmp, error))?;
            Ok(StagingKind::PlainCopy)
        }
    }
}
