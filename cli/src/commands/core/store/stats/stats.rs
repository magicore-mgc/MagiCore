//! `mgc store stats --json` — measurable cache and store state.
//! `mgc store stats --json` — trạng thái cache/store đo được.
//!
//! The goal requires a store whose hit/miss/dedupe/bytes are observable before
//! any "smarter cache" claim is made. Every field here is read from the live
//! filesystem and the CAS memo counters, never estimated.
//! (Mục tiêu yêu cầu store có hit/miss/dedupe/bytes quan sát được trước khi
//! đưa ra bất kỳ claim "cache thông minh" nào. Mọi trường ở đây đọc từ
//! filesystem thật và bộ đếm memo CAS, không ước lượng.)

use std::collections::HashSet;
use std::path::Path;

use serde::Serialize;

/// One store snapshot.
/// Một ảnh chụp store.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct StoreStats {
    /// Store root the numbers describe.
    /// Root store mà các số liệu mô tả.
    pub root: String,
    /// Content-addressed blobs (files under `cas/`).
    /// Blob theo nội dung (file dưới `cas/`).
    pub cas_blobs: u64,
    /// Bytes held by content-addressed blobs.
    /// Số byte do blob theo nội dung giữ.
    pub cas_bytes: u64,
    /// Tarball cache entries.
    /// Entry cache tarball.
    pub cache_tarballs: u64,
    /// Bytes held by the tarball cache.
    /// Số byte cache tarball giữ.
    pub cache_bytes: u64,
    /// Memo verification hits since this process started reading the store.
    /// Số lần memo verify hit từ khi process này bắt đầu đọc store.
    pub memo_hits: u64,
    /// Memo verification misses (a miss re-hashes, never a wrong serve).
    /// Số lần memo miss (miss sẽ hash lại, không bao giờ serve sai).
    pub memo_misses: u64,
    /// Memo evictions caused by the bounded LRU.
    /// Số lần memo bị đẩy ra do LRU có giới hạn.
    pub memo_evictions: u64,
    /// Distinct payloads observed in the store: dedupe baseline.
    /// Số payload phân biệt quan sát trong store: mốc dedupe.
    pub distinct_payloads: u64,
    /// Payload copies counted in the store (hardlink/reflink aware: an inode
    /// reached twice counts once).
    /// Số bản payload đếm trong store (hiểu hardlink/reflink: một inode gặp
    /// hai lần chỉ tính một).
    pub payload_copies: u64,
    /// Share of payload copies saved by inode sharing, `0.0` when nothing is
    /// stored or nothing is shared.
    /// Phần bản copy payload được tiết kiệm nhờ chia sẻ inode, `0.0` khi store
    /// rỗng hoặc chưa chia sẻ gì.
    pub dedupe_ratio: f64,
}

/// Collect a snapshot for the store rooted at `root`.
/// Thu ảnh chụp cho store gốc `root`.
pub fn collect(root: &Path) -> std::io::Result<StoreStats> {
    let cas_root = root.join("cas");
    let cache_root = root.join("cache");
    let (cas_blobs, cas_bytes) = measure(&cas_root, |_| true)?;
    let (cache_tarballs, cache_bytes) = measure(&cache_root, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("tgz")
    })?;

    // Dedupe is measured on inode identity, not path count: two hardlinks to
    // one blob are one payload, while two copies of identical bytes on
    // different inodes are a dedupe gap the number must expose.
    // (Dedupe đo theo identity inode, không theo số path: hai hardlink tới một
    // blob là một payload, còn hai bản bytes giống nhau trên inode khác là lỗ
    // hổng dedupe mà con số phải phơi ra.)
    let mut seen_inodes = HashSet::new();
    let (copies, _) = measure(&cas_root, |path| {
        seen_inodes.insert(inode_key(path));
        true
    })?;
    let distinct = seen_inodes.len() as u64;
    // Dedupe ratio expressed as saved copies: paths that share an inode are
    // one payload stored once, so `1 - distinct/copies` is 0 when every path
    // is a separate copy and grows as sharing improves.
    // (Tỉ lệ dedupe là phần bản copy được tiết kiệm: các path chung inode là
    // một payload lưu một lần, nên `1 - distinct/copies` bằng 0 khi mỗi path
    // là bản riêng và tăng khi chia sẻ tốt hơn.)
    let dedupe_ratio = if copies == 0 {
        0.0
    } else {
        1.0 - (distinct as f64 / copies as f64)
    };

    let (memo_hits, memo_misses, memo_evictions) = memo_counters(root);
    Ok(StoreStats {
        root: root.display().to_string(),
        cas_blobs,
        cas_bytes,
        cache_tarballs,
        cache_bytes,
        memo_hits,
        memo_misses,
        memo_evictions,
        distinct_payloads: distinct,
        payload_copies: copies,
        dedupe_ratio,
    })
}

/// Memo counters for a store root; a store that was never opened in this
/// process reports zeros rather than inventing numbers.
/// (Bộ đếm memo cho một root; store chưa từng mở trong process báo 0 thay vì
/// bịa số.)
fn memo_counters(root: &Path) -> (u64, u64, u64) {
    match mgc_store::ContentStore::new(root.to_path_buf()) {
        Ok(store) => {
            let stats = store.memo_stats();
            (stats.hits, stats.misses, stats.evictions)
        }
        Err(_) => (0, 0, 0),
    }
}

/// Count files (and their bytes) under a directory that pass `keep`.
/// Đếm file (và byte) dưới một thư mục thỏa `keep`.
fn measure<F>(root: &Path, mut keep: F) -> std::io::Result<(u64, u64)>
where
    F: FnMut(&Path) -> bool,
{
    let mut count = 0u64;
    let mut bytes = 0u64;
    if !root.exists() {
        return Ok((0, 0));
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(directory) = stack.pop() {
        let entries = match std::fs::read_dir(&directory) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(metadata) = std::fs::symlink_metadata(&path) else {
                continue;
            };
            if metadata.is_dir() {
                stack.push(path);
            } else if metadata.is_file() && keep(&path) {
                count += 1;
                bytes += metadata.len();
            }
        }
    }
    Ok((count, bytes))
}

/// Stable identity for a file used by the dedupe measurement.
/// Identity ổn định của file dùng cho phép đo dedupe.
#[cfg(unix)]
fn inode_key(path: &Path) -> (u64, u64) {
    use std::os::unix::fs::MetadataExt;
    match std::fs::symlink_metadata(path) {
        Ok(metadata) => (metadata.dev(), metadata.ino()),
        // A vanished file gets a unique key so it cannot inflate dedupe.
        // (File biến mất nhận key duy nhất để không làm tăng dedupe.)
        Err(_) => (u64::MAX, path_hash(path)),
    }
}

#[cfg(not(unix))]
fn inode_key(path: &Path) -> (u64, u64) {
    (0, path_hash(path))
}

fn path_hash(path: &Path) -> u64 {
    u64::from(mgc_crypto::Blake3Hasher::hash_bytes(path.to_string_lossy().as_bytes()).0[0])
}

/// Print the snapshot as JSON, or as a short human summary.
/// In ảnh chụp dạng JSON, hoặc tóm tắt người đọc.
pub fn print(stats: &StoreStats, json: bool) {
    if json {
        match serde_json::to_string_pretty(stats) {
            Ok(text) => println!("{text}"),
            Err(error) => eprintln!("WARNING: cannot serialize store stats: {error}"),
        }
        return;
    }
    println!("Store: {}", stats.root);
    println!(
        "  CAS:     {} blobs, {} bytes",
        stats.cas_blobs, stats.cas_bytes
    );
    println!(
        "  Cache:   {} tarballs, {} bytes",
        stats.cache_tarballs, stats.cache_bytes
    );
    println!(
        "  Memo:    {} hits / {} misses / {} evictions",
        stats.memo_hits, stats.memo_misses, stats.memo_evictions
    );
    println!(
        "  Dedupe:  {} distinct payloads over {} copies (ratio {:.3})",
        stats.distinct_payloads, stats.payload_copies, stats.dedupe_ratio
    );
}

#[cfg(test)]
#[path = "test/stats.rs"]
mod test;
