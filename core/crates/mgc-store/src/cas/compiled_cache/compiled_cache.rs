//! Compiled-cache path, lookup and primary publication API.
//! Path, tra cứu và API publish chính của compiled cache.

use super::temp_guard::TmpGuard;
use super::{CompilationKey, CompiledModule, CompiledRecord, VerifyOutcome};
use crate::cas::store::StoreError;
use crate::cas::write::{sync_parent_dir, unique_tmp_path};
use std::fs;
use std::path::PathBuf;

pub struct CompiledCache {
    pub(in crate::cas::compiled_cache) root: PathBuf,
}

impl CompiledCache {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Cache entry path for a key (sharded 2-hex, same layout as the CAS).
    /// (Path entry cache cho 1 key (shard 2-hex, cùng layout với CAS).)
    fn module_path(&self, key: &CompilationKey) -> PathBuf {
        let master = key.master_digest();
        let algo_dir = self.root.join("compiled").join("blake3");
        // Safe accessor: IntegrityHash type guarantees 64 hex chars (P0-1).
        // (Accessor an toàn: type IntegrityHash đảm bảo 64 ký tự hex (P0-1).)
        let hex = master.as_hex();
        let first2 = &hex[..2];
        algo_dir.join(first2).join(hex).with_extension("json")
    }

    /// Read + verify a compiled entry (P0-3). Returns:
    /// - `Ok(Some(module))` — entry exists and PASSED self-validation;
    /// - `Ok(None)` — miss, or STALE schema (old-format entry: recompile
    ///   and `put` replaces it);
    /// - `Err(CacheCorrupt)` — well-formed-looking entry whose
    ///   key/digest verification FAILED (poison/tamper): quarantined for
    ///   forensics, never served.
    ///
    /// Đọc + verify entry đã biên dịch (P0-3). Trả về:
    /// - `Ok(Some(module))` — entry tồn tại và ĐÃ qua tự kiểm tra;
    /// - `Ok(None)` — miss, hoặc schema STALE (entry cũ: biên dịch lại rồi
    ///   `put` sẽ thay);
    /// - `Err(CacheCorrupt)` — entry trông hợp lệ nhưng verify key/digest
    ///   THẤT BẠI (đầu độc/gian lận): bị cách ly để điều tra, không phục vụ.
    pub fn get(&self, key: &CompilationKey) -> Result<Option<CompiledModule>, StoreError> {
        let path = self.module_path(key);
        let Some(meta) = fs::symlink_metadata(&path).ok() else {
            return Ok(None);
        };
        if meta.file_type().is_symlink() {
            return Err(StoreError::CacheCorrupt {
                path,
                detail: "compiled cache entry is a symlink".to_string(),
            });
        }

        let data = fs::read(&path)?;
        let record: CompiledRecord = match serde_json::from_slice(&data) {
            Ok(r) => r,
            // Torn/unparseable bytes — NOT a serve-able entry. Quarantine
            // for forensics and fail closed (P0-3): the dev server treats
            // the error as a miss and recompiles; its next put replaces
            // the quarantined slot.
            // (Bytes đứt/không parse được — KHÔNG phải entry phục vụ được.
            // Cách ly để điều tra và fail cứng (P0-3): dev server coi lỗi
            // như miss và biên dịch lại; put kế tiếp thay slot đã cách ly.)
            Err(e) => {
                let detail = format!("failed to parse compiled record: {e}");
                self.quarantine_corrupt(&path, key, &detail)?;
                return Err(StoreError::CacheCorrupt { path, detail });
            }
        };

        match record.verify_against(key) {
            VerifyOutcome::Authentic => Ok(Some(record.to_module())),
            VerifyOutcome::StaleSchema => {
                // Old-format entry: NOT corruption, report a miss so the
                // caller recompiles; the following put replaces it.
                // (Entry format cũ: KHÔNG phải hỏng, báo miss để caller
                // biên dịch lại; put kế tiếp sẽ thay nó.)
                Ok(None)
            }
            VerifyOutcome::Failed(reason) => {
                // Verification failed — poison or tampering. Quarantine
                // for forensics and fail closed (P0-3).
                // (Verify fail — độc hoặc gian lận. Cách ly để điều tra và
                // fail cứng (P0-3).)
                self.quarantine_corrupt(&path, key, &reason)?;
                Err(StoreError::CacheCorrupt {
                    path,
                    detail: reason,
                })
            }
        }
    }

    /// Publish a compiled output atomically and deterministically (P0-2).
    ///
    /// Race contract (identical on Unix and Windows — publication uses
    /// `hard_link`, which fails when the destination exists on BOTH):
    /// 1. Serialize the self-validating record → unique temp
    ///    (create_new) → write → fsync.
    /// 2. `hard_link(temp → entry)`: the FIRST complete writer wins; a
    ///    loser never overwrites the winner (no last-wins nondeterminism).
    /// 3. On AlreadyExists the loser READS and VERIFIES the winner:
    ///    - identical output digest → Ok (legit dedup);
    ///    - stale/corrupt winner → quarantine/remove + retry link once;
    ///    - divergent payload, same key → typed `CacheConflict` — the API
    ///      never fakes success for a divergent result.
    ///
    /// Publish output đã biên dịch nguyên tử và tất định (P0-2).
    ///
    /// Hợp đồng race (giống hệt trên Unix và Windows — publish dùng
    /// `hard_link`, lỗi khi đích tồn tại trên CẢ HAI nền tảng):
    /// 1. Serialize record tự kiểm tra (self-validating) → temp duy nhất
    ///    (create_new) → ghi → fsync.
    /// 2. `hard_link(temp → entry)`: writer HOÀN CHỈNH ĐẦU TIÊN thắng; bên
    ///    thua không bao giờ đè winner (không có last-wins không tất định).
    /// 3. Khi AlreadyExists, bên thua ĐỌC và VERIFY winner:
    ///    - digest output giống nhau → Ok (dedup hợp lệ);
    ///    - winner stale/hỏng → quarantine/remove + retry link đúng 1 lần;
    ///    - payload phân kỳ cùng key → `CacheConflict` typed — API không
    ///      bao giờ giả thành công cho kết quả phân kỳ.
    pub fn put(&self, key: &CompilationKey, module: &CompiledModule) -> Result<(), StoreError> {
        let path = self.module_path(key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let record = CompiledRecord::from_module(key, module);
        let data = serde_json::to_vec(&record).map_err(|e| StoreError::Io {
            path: path.clone(),
            msg: format!("failed to serialize compiled record: {e}"),
        })?;

        let parent = path.parent().unwrap_or(&self.root);
        let tmp = unique_tmp_path(parent, "compiled");
        let mut guard = TmpGuard::new(&tmp);

        let mut file = fs::File::create_new(&tmp)?;
        std::io::Write::write_all(&mut file, &data)?;
        std::io::Write::flush(&mut file)?;
        let sync = file.sync_all();
        drop(file);
        sync?;

        match fs::hard_link(&tmp, &path) {
            Ok(()) => {
                // First writer: the link IS the publish — drop our temp name
                // and make the link durable. REMOVE FIRST, disarm SECOND
                // (P1-4, Tech Lead vòng-6/7): the old order disarmed the
                // RAII guard before removing, so a failed remove left an
                // ORPHAN temp forever (no retry possible). Keeping the
                // guard armed through the remove means a failure still
                // retries via Drop — and a success path that cannot
                // remove its own just-written temp is a real filesystem
                // problem worth surfacing, not swallowing.
                // (Writer đầu: link chính là publish — bỏ tên temp và
                // khiến link bền vững. REMOVE TRƯỚC, disarm SAU (P1-4):
                // thứ tự cũ disarm guard RAII trước khi remove nên remove
                // fail để lại temp MỒ CÔI vĩnh viễn (không retry được).
                // Giữ guard armed qua remove nghĩa là fail vẫn retry qua
                // Drop — còn đường thành công mà không xóa được temp do
                // chính mình vừa ghi là lỗi filesystem thật đáng báo, không
                // đáng nuốt.)
                match fs::remove_file(&tmp) {
                    Ok(()) => {
                        guard.disarm();
                        sync_parent_dir(parent)?;
                        Ok(())
                    }
                    Err(e) => Err(StoreError::Io {
                        path: tmp.to_path_buf(),
                        msg: format!("published the entry but failed to remove its temp: {e}"),
                    }),
                }
            }
            Err(e) if path.exists() => {
                // Lost the publication race — the winner exists. Verify it
                // before deciding anything (P0-2: no silent Ok).
                // (Thua race publish — winner đã có. Verify winner trước khi
                // quyết định bất cứ gì (P0-2: không Ok im lặng).)
                let _ = e;
                self.resolve_race(key, &path, &tmp, &record)
            }
            Err(e) => Err(StoreError::Io {
                path: path.clone(),
                msg: format!("atomic compiled-cache publish (hard_link) failed: {e}"),
            }),
        }
    }
}
