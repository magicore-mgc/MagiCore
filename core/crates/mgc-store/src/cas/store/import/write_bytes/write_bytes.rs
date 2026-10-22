//! CAS import owner implementation.
//! Triển khai owner import CAS.

use mgc_platform::atomic_replace::atomic_replace_file_with_write_through;
use std::fs;
use std::io::BufReader;

use super::super::super::error::StoreError;
use super::super::super::types::ContentStore;
use crate::cas::integrity::IntegrityHash;
use crate::cas::write::{
    STREAM_THRESHOLD, stream_write_verify_and_set_perms_mode, sync_parent_dir,
    write_all_verify_and_set_perms_mode,
};

impl ContentStore {
    pub(in crate::cas::store::import) fn write_bytes_with_hash(
        &self,
        data: &[u8],
        hash: &IntegrityHash,
        executable: bool,
    ) -> Result<IntegrityHash, StoreError> {
        self.write_bytes_with_hash_mode(data, hash, executable, true)
    }

    fn write_bytes_with_hash_mode(
        &self,
        data: &[u8],
        hash: &IntegrityHash,
        executable: bool,
        sync_parent_directory: bool,
    ) -> Result<IntegrityHash, StoreError> {
        // ADVERSARIAL GATE (vòng-2 review finding R3): the declared hash must
        // match the actual bytes BEFORE any cache-hit shortcut. Without this
        // gate, importing content-A while declaring digest-B (whose blob
        // already exists in the CAS) would verify the OLD valid blob-B,
        // return Ok, and let the caller believe content-A was stored under
        // digest-B — a poisoning bypass.
        //
        // CỔNG ĐỐI KHÁNG (finding R3 vòng-2 review): digest khai phải khớp
        // bytes thực TRƯỚC mọi đường tắt cache-hit. Không có cổng này, import
        // content-A nhưng khai digest-B (mà blob-B đã có trong CAS) sẽ verify
        // blob-B cũ hợp lệ, trả Ok, và để caller tin content-A đã được lưu
        // dưới digest-B — bypass đầu độc.
        let actual = IntegrityHash::from_bytes(data, executable);
        if actual.as_hex() != hash.as_hex() {
            return Err(StoreError::HashMismatch {
                expected: hash.as_hex().to_string(),
                actual: actual.as_hex().to_string(),
            });
        }

        self.write_generated_bytes_with_hash_mode(
            data,
            hash,
            executable,
            sync_parent_directory,
            sync_parent_directory,
        )
    }

    // The caller generated `hash` from these exact bytes; the writer still
    // verifies bytes as it persists them before any atomic publish.
    // (Caller đã tạo `hash` từ đúng bytes này; bộ ghi vẫn xác minh bytes
    // trong lúc lưu trước khi publish nguyên tử.)
    pub(in crate::cas::store::import) fn write_generated_bytes_with_hash_mode(
        &self,
        data: &[u8],
        hash: &IntegrityHash,
        executable: bool,
        sync_parent_directory: bool,
        sync_file_data: bool,
    ) -> Result<IntegrityHash, StoreError> {
        let _publication_guard = super::super::import_lock::lock_digest(hash);
        let dest = hash.cas_path(&self.root);
        if dest.exists() {
            // Existing content wins (dedup) but ONLY after re-hashing it.
            // Nội dung đã có (dedup) chỉ được dùng SAU khi rehash xong.
            match self.verify_stored_blob(hash) {
                Ok(()) => return Ok(hash.clone()),
                Err(StoreError::HashMismatch { .. }) => {
                    // A verified import repairs the corrupt address below.
                    // Import hợp lệ sẽ sửa entry hỏng bằng publish atomic bên dưới.
                }
                Err(error) => return Err(error),
            }
        }

        fs::create_dir_all(dest.parent().ok_or_else(|| StoreError::Io {
            path: dest.to_path_buf(),
            msg: "write destination has no parent directory".to_string(),
        })?)?;
        let tmp = self.tmp_path("write-bytes");
        fs::create_dir_all(tmp.parent().ok_or_else(|| StoreError::Io {
            path: tmp.clone(),
            msg: "staging temp path has no parent directory".to_string(),
        })?)?;
        let writer = fs::File::create(&tmp)?;

        // The digest gate above already proved data==hash, and the write
        // primitives hash-while-writing for the torn-write check — the
        // returned hash must equal `actual` by construction.
        //
        // Cổng digest phía trên đã chứng minh data==hash, và nguyên thủy ghi
        // hash-while-writing để check torn-write — hash trả về buộc phải
        // bằng `actual` theo cấu trúc.
        let written = if data.len() >= STREAM_THRESHOLD {
            let cursor = std::io::Cursor::new(data);
            let reader = BufReader::new(cursor);
            stream_write_verify_and_set_perms_mode(writer, &tmp, reader, executable, sync_file_data)
        } else {
            write_all_verify_and_set_perms_mode(writer, &tmp, data, executable, sync_file_data)
        }?;
        if written.as_hex() != hash.as_hex() {
            let _ = fs::remove_file(&tmp);
            return Err(StoreError::HashMismatch {
                expected: hash.as_hex().to_string(),
                actual: written.as_hex().to_string(),
            });
        }

        if let Err(source) = atomic_replace_file_with_write_through(&tmp, &dest, sync_file_data) {
            let _ = fs::remove_file(&tmp);
            return Err(StoreError::Io {
                path: dest.clone(),
                msg: format!("move tmp file into CAS failed: {}", source.io_source()),
            });
        }
        let _ = fs::remove_file(&tmp);

        // Durable imports fsync the CAS parent dir after rename. Rebuildable
        // package-cache imports omit this sync and must be verified/rebuilt
        // before reuse after a crash.
        // (Import cần durability fsync thư mục CAS sau rename. Import cache
        // package có thể dựng lại bỏ sync này và phải verify/dựng lại trước
        // khi dùng sau crash.)
        if sync_parent_directory && let Some(parent) = dest.parent() {
            sync_parent_dir(parent)?;
        }

        Ok(hash.clone())
    }
}
