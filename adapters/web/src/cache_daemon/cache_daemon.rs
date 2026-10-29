//! In-memory package cache used by warm installs.
//! Cache package trong bộ nhớ cho lượt cài ấm.
//!
//! The map is bounded: a warm install must not be able to grow the process
//! without limit, and the eviction must not throw every entry away at once.
//! Bounded map: cài ấm không được phình process vô hạn, và khi đầy phải evict
//! từng phần thay vì xoá sạch toàn bộ.

use mgc_types::PackageId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Maximum entries retained before the oldest quarter is evicted.
///
/// Overridable with `MAGICORE_WEB_CACHE_DAEMON_MAX_ENTRIES` so a machine with
/// a small memory budget can lower it without a rebuild.
/// (Số entry tối đa trước khi evict 1/4 cũ nhất. Ghi đè bằng
/// `MAGICORE_WEB_CACHE_DAEMON_MAX_ENTRIES` để máy ít RAM hạ xuống không cần
/// build lại.)
pub const DEFAULT_MAX_ENTRIES: usize = 10000;

/// Environment variable that overrides [`DEFAULT_MAX_ENTRIES`].
/// Biến môi trường ghi đè [`DEFAULT_MAX_ENTRIES`].
pub const MAX_ENTRIES_ENV: &str = "MAGICORE_WEB_CACHE_DAEMON_MAX_ENTRIES";

/// Fraction of entries dropped when the map is full (one quarter).
/// Phần entry bị bỏ khi map đầy (một phần tư).
const EVICTION_DIVISOR: usize = 4;

/// One cached package with its insertion sequence number.
/// Một package đã cache kèm số thứ tự chèn.
struct Entry {
    data: Arc<[u8]>,
    sequence: u64,
}

pub struct MemoryCacheDaemon {
    entries: Mutex<CacheState>,
    max_entries: usize,
}

struct CacheState {
    map: HashMap<String, Entry>,
    next_sequence: u64,
}

impl MemoryCacheDaemon {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<MemoryCacheDaemon> = OnceLock::new();
        INSTANCE.get_or_init(|| MemoryCacheDaemon::with_capacity(resolve_max_entries()))
    }

    /// Build a daemon with an explicit bound; tests use a small one.
    /// Dựng daemon với giới hạn tường minh; test dùng giới hạn nhỏ.
    pub fn with_capacity(max_entries: usize) -> Self {
        MemoryCacheDaemon {
            entries: Mutex::new(CacheState {
                map: HashMap::new(),
                next_sequence: 0,
            }),
            max_entries: max_entries.max(1),
        }
    }

    pub fn get(&self, id: &PackageId) -> Option<Arc<[u8]>> {
        let key = format!("{}@{}", id.name_str(), id.version());
        let guard = self.entries.lock().ok()?;
        guard.map.get(&key).map(|entry| Arc::clone(&entry.data))
    }

    pub fn insert(&self, id: &PackageId, data: &[u8]) {
        let key = format!("{}@{}", id.name_str(), id.version());
        let Ok(mut guard) = self.entries.lock() else {
            return;
        };
        let sequence = guard.next_sequence;
        guard.next_sequence = guard.next_sequence.wrapping_add(1);
        guard.map.insert(
            key,
            Entry {
                data: Arc::from(data),
                sequence,
            },
        );
        if guard.map.len() > self.max_entries {
            evict_oldest_quarter(&mut guard.map);
        }
    }

    /// Number of entries currently held; used by tests and diagnostics.
    /// Số entry hiện có; test và chẩn đoán dùng.
    pub fn len(&self) -> usize {
        self.entries
            .lock()
            .map(|guard| guard.map.len())
            .unwrap_or(0)
    }

    /// Whether the cache currently holds nothing.
    /// Cache hiện rỗng hay không.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Drop the oldest `1 / EVICTION_DIVISOR` of the map.
///
/// Clearing the whole map (the previous behaviour) threw away entries a warm
/// install was still about to read, turning one over-limit insert into a full
/// miss storm.
/// (Xoá sạch map như trước làm mất entry mà lượt cài ấm sắp đọc, biến một lần
/// vượt giới hạn thành bão miss.)
fn evict_oldest_quarter(map: &mut HashMap<String, Entry>) {
    let target = (map.len() / EVICTION_DIVISOR).max(1);
    let mut sequences: Vec<u64> = map.values().map(|entry| entry.sequence).collect();
    sequences.sort_unstable();
    // Drop exactly the `target` oldest sequences: the newest entry — the one
    // that triggered this eviction — must always survive.
    // Bỏ đúng `target` sequence cũ nhất: entry mới nhất (vừa gây evict) phải
    // luôn sống sót.
    let cutoff = sequences[target - 1];
    map.retain(|_, entry| entry.sequence > cutoff);
}

/// Read the entry limit from the environment, falling back to the default.
/// Đọc giới hạn entry từ môi trường, mặc định nếu không có.
fn resolve_max_entries() -> usize {
    std::env::var(MAX_ENTRIES_ENV)
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_MAX_ENTRIES)
}
