//! ContentStore construction and shared root-level operations.
//! Khởi tạo ContentStore và thao tác gốc dùng chung.

use super::error::StoreError;
use super::export::try_cow_clone;
#[cfg(unix)]
use super::types::{AtomicMemoCounters, MEMO_NONZERO};
use super::types::{ContentStore, MemoStats, StagingKind};
use crate::cas::integrity::IntegrityHash;
use crate::cas::lifecycle::{ensure_cas_dirs, set_cas_root_permissions, validate_cas_root};
use crate::cas::write::unique_tmp_path;
use std::fs;
use std::path::{Path, PathBuf};

impl ContentStore {
    pub fn new(root: PathBuf) -> Result<Self, StoreError> {
        validate_cas_root(&root)?;
        ensure_cas_dirs(&root)?;
        set_cas_root_permissions(&root)?;
        // Bounded LRU memo (P1-2 audit vòng-4) — capacity from the central
        // const, overridable per-store for tests via `with_memo_capacity`.
        // (Memo LRU có giới hạn (P1-2 audit vòng-4) — capacity từ const tập
        // trung, ghi đè được từng store cho test qua `with_memo_capacity`.)
        Ok(Self {
            root,
            #[cfg(unix)]
            verified: std::sync::Arc::new(parking_lot::Mutex::new(lru::LruCache::new(
                MEMO_NONZERO,
            ))),
            #[cfg(unix)]
            memo_counters: std::sync::Arc::new(AtomicMemoCounters::default()),
        })
    }

    /// Test-observable memo stats (P1-2): hits/misses/evictions since store
    /// creation. Read by the LRU-bound test; not part of the serving API.
    ///
    /// (Chỉ số memo quan sát được cho test (P1-2): hit/miss/eviction từ lúc
    /// dựng store. Test LRU-bound đọc; không thuộc serving API.)
    // Counters are plain atomics on every platform; the CLI's store-stats
    // command reads them, so this cannot be Unix-only.
    // (Bộ đếm là atomic trên mọi nền tảng; lệnh store-stats đọc chúng nên không
    // thể chỉ dành cho Unix.)
    #[doc(hidden)]
    pub fn memo_stats(&self) -> MemoStats {
        let c = &self.memo_counters;
        MemoStats {
            hits: c.hits.load(std::sync::atomic::Ordering::Relaxed),
            misses: c.misses.load(std::sync::atomic::Ordering::Relaxed),
            evictions: c.evictions.load(std::sync::atomic::Ordering::Relaxed),
        }
    }

    /// Test hook: build a store sharing this root with a TINY memo capacity
    /// to prove the LRU bound + eviction accounting (P1-2). Counters reset
    /// with the fresh handle. Not part of the serving API.
    ///
    /// (Hook test: dựng store cùng root này với capacity memo NHỎ để chứng
    /// minh giới hạn LRU + đếm eviction (P1-2). Bộ đếm reset cùng handle
    /// mới. Không thuộc serving API.)
    #[cfg(unix)]
    #[doc(hidden)]
    pub fn with_memo_capacity(&self, capacity: std::num::NonZeroUsize) -> Self {
        // NonZeroUsize (P1-3, Tech Lead vòng-5/6/7): the old `usize` +
        // `expect` panicked on 0 at runtime; the type now makes a
        // zero-capacity memo UNREPRESENTABLE — no panic path exists.
        // (NonZeroUsize (P1-3): kiểu cũ `usize` + `expect` panic khi 0
        // lúc runtime; giờ type khiến memo capacity-0 KHÔNG THỂ BIỂU DIỄN —
        // không còn đường panic.)
        Self {
            root: self.root.clone(),
            verified: std::sync::Arc::new(parking_lot::Mutex::new(lru::LruCache::new(capacity))),
            memo_counters: std::sync::Arc::new(AtomicMemoCounters::default()),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn compiled_cache(&self) -> crate::cas::CompiledCache {
        crate::cas::CompiledCache::new(self.root.clone())
    }

    pub(in crate::cas::store) fn tmp_path(&self, prefix: &str) -> PathBuf {
        let path = unique_tmp_path(&self.root.join("tmp"), prefix);
        // Parent always exists by construction (join appends components),
        // but a degenerate root must degrade to a downstream I/O error,
        // never a panic — mirror the call-site guards above.
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            let _ = fs::create_dir_all(parent);
        }
        path
    }

    /// Test-only probe: which staging strategy `export_to` would use on
    /// this machine/filesystem pair. Lets tests assert the CowClone branch
    /// is LIVE on COW-capable filesystems (a fallback that silently never
    /// runs is a false-green — audit vòng-3 P0-2). Not part of the serving
    /// API.
    ///
    /// Probe chỉ dành cho test: chiến lược staging `export_to` sẽ dùng cho
    /// cặp máy/filesystem này. Để test assert nhánh CowClone còn SỐNG trên
    /// filesystem hỗ trợ COW (fallback im lặng không chạy là false-green —
    /// P0-2 audit vòng-3). Không thuộc serving API.
    #[doc(hidden)]
    pub fn staging_kind_probe(&self, hash: &IntegrityHash, dest_dir: &Path) -> StagingKind {
        // Stage into a scratch file next to a probe destination, mirror of
        // the real export staging path.
        // (Staging vào file scratch cạnh đích probe, phản chiếu đúng đường
        // staging export thật.)
        let src = hash.cas_path(&self.root);
        let tmp = unique_tmp_path(dest_dir, "mgc-export-probe");
        match try_cow_clone(&src, &tmp) {
            Some(()) => {
                let _ = fs::remove_file(&tmp);
                StagingKind::CowClone
            }
            None => {
                let _ = fs::remove_file(&tmp);
                StagingKind::PlainCopy
            }
        }
    }
}
