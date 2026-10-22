//! Prune CAS blobs only after reading live store refcounts.
//! Chỉ prune CAS sau khi đọc được refcount sống của store.

use crate::commands::cache::filesystem::remove_path;
use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

/// Prune CAS blobs using the SQLite refcount (slice 5 of T1): a blob is
/// prunable when it has no live refcount in the DB. FAIL-CLOSED (P0-C,
/// Tech Lead vòng-7 2026-09-13): exports are independent copies since the
/// hardlink removal, so a LIVE blob has nlink == 1 — the old nlink-only
/// fallback (on DB missing/corrupt/unreadable) treated live blobs as
/// unreferenced and DELETED the shared cache projects still warm-claim.
/// When the DB cannot be read, prune REFUSES to run and points at
/// `mgc store doctor` — it never guesses.
/// (Prune blob CAS theo refcount SQLite: xóa blob khi không còn ref sống
/// trong DB. FAIL-CLOSED (P0-C): export là bản sao độc lập từ khi bỏ
/// hardlink nên blob SỐNG có nlink == 1 — fallback nlink-only cũ (khi DB
/// mất/hỏng/không đọc được) coi blob sống là unreferenced và XÓA cache
/// chia sẻ mà project còn đang warm-claim. Khi DB không đọc được, prune
/// TỪ CHỐI chạy và trỏ tới `mgc store doctor` — không bao giờ đoán mò.)
pub(in crate::commands::cache) fn prune_cas_blobs_under(
    cas_root: &Path,
    store_root: &Path,
    dry_run: bool,
) -> Result<usize> {
    if !cas_root.exists() {
        return Ok(0);
    }

    // The DB must ALREADY exist — `Database::open` is open-OR-CREATE: on a
    // missing store.db it would silently build a FRESH empty refset, and
    // pruning against zero claims would delete every live warm-cache blob
    // (exactly the failure mode P0-C exists to prevent). A missing DB is
    // indistinguishable from a lost refset — refuse.
    // (DB phải ĐÃ TỒN TẠI — `Database::open` là open-HOẶC-CREATE: với
    // store.db mất, nó âm thầm dựng refset MỚI RỖNG, và prune theo 0
    // claim sẽ xóa mọi blob warm-cache đang sống (đúng chế độ hỏng mà
    // P0-C tồn tại để chặn). DB vắng không phân biệt được với refset
    // bị mất — từ chối.)
    let db_path = store_root.join("store.db");
    if !db_path.exists() {
        return Err(anyhow::anyhow!(
            "store database missing for prune ({}); refusing to build an empty \
             refset — run `mgc store doctor` first",
            db_path.display()
        ));
    }

    let live: HashSet<String> = match mgc_store::Database::open(&db_path) {
        Ok(db) => match db.list_cas_live_refs() {
            Ok(live_refs) => live_refs.into_iter().collect(),
            // Refcount unreadable on a healthy-looking DB (corrupt table,
            // locked, I/O error) — refuse: nlink is NOT a source of truth
            // after the export-to-independent-copy switch.
            // (Refcount không đọc được trên DB trông khỏe (bảng hỏng, bị
            // khóa, lỗi I/O) — từ chối: nlink KHÔNG còn là nguồn chân lý
            // sau khi export chuyển sang bản sao độc lập.)
            Err(err) => {
                return Err(anyhow::anyhow!(
                    "cannot read CAS refcount for prune (store.db: {err}); \
                     refusing to prune by nlink — run `mgc store doctor`"
                ));
            }
        },
        // DB missing/corrupt/unopenable — refuse to prune. Pruning without
        // refcounts would delete live warm-cache blobs (they are plain
        // files with nlink 1 now).
        // (DB mất/hỏng/không mở được — từ chối prune. Prune không có
        // refcount sẽ xóa blob warm-cache đang sống (giờ chỉ là file
        // thường nlink 1).)
        Err(err) => {
            return Err(anyhow::anyhow!(
                "store database unavailable for prune ({err}); refusing to \
                 prune by nlink — run `mgc store doctor` first"
            ));
        }
    };

    // A blob is prunable when the DB has no live claim for it.
    // (Blob xóa được khi DB không còn claim sống nào cho nó.)
    let mut pruned = 0usize;
    let mut directories = Vec::new();
    for entry in WalkDir::new(cas_root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.file_type().is_dir() {
            directories.push(entry.path().to_path_buf());
            continue;
        }
        if !entry.file_type().is_file() {
            continue;
        }
        let blob_hash = entry
            .path()
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.split('.').next().unwrap_or(name).to_string())
            .unwrap_or_default();
        if live.contains(&blob_hash) {
            continue;
        }
        pruned += 1;
        if !dry_run {
            remove_path(entry.path())?;
        }
    }
    if !dry_run {
        directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
        for dir in directories {
            let _ = std::fs::remove_dir(&dir);
        }
    }
    Ok(pruned)
}
