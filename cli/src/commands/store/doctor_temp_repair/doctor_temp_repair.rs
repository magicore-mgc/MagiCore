//! Removes only old, recognized MagiCore temp entries after path-safety checks.
//! Chỉ xóa temp MagiCore hợp lệ đã cũ sau kiểm tra an toàn path.

use std::path::Path;

pub(in crate::commands::store) fn repair_orphan_temps(
    tmp_dir: &Path,
    repair: bool,
    json: bool,
    report: &mut super::super::doctor_report::DoctorReport,
) -> anyhow::Result<bool> {
    let mut hard_failure = false;
    // Orphan temp GC (--repair, P0-C HARDENING vòng-9): the old code took
    // `tmp_dir` from `cas_root.parent()` — correct path, but it derefed
    // entries via `entry.metadata()` (follows symlinks) and would happily
    // recurse through a symlinked `tmp` into files OUTSIDE the store. The
    // GC now (1) refuses to run through a symlinked tmp (no-follow
    // `symlink_metadata` on the dir itself), (2) skips every entry whose
    // own metadata is a symlink — never follows into a redirect, (3) only
    // removes names carrying the MGC temp-name schema
    // (`<prefix>-<pid>-<tid>-<nanos>-<counter>`), leaving foreign files
    // alone, and (4) reports — never guesses — when the tmp dir itself is
    // unsafe.
    // (GC temp mồ côi (--repair, cứng hóa P0-C vòng-9): code cũ lấy
    // `tmp_dir` từ `cas_root.parent()` — path đúng, nhưng deref entry qua
    // `entry.metadata()` (follow symlink) và sẵn sàng đệ quy qua `tmp`
    // là symlink vào file NGOÀI store. GC giờ (1) từ chối chạy qua tmp
    // là symlink (no-follow `symlink_metadata` trên chính dir), (2) bỏ
    // qua mọi entry mà metadata của nó là symlink — không đi theo
    // redirect, (3) chỉ xóa tên theo schema temp của MGC
    // (`<prefix>-<pid>-<tid>-<nanos>-<counter>`), file ngoài để nguyên,
    // (4) báo — không đoán — khi chính tmp dir không an toàn.)
    let tmp_meta_ok = std::fs::symlink_metadata(tmp_dir)
        .map(|m| m.is_dir() && !m.file_type().is_symlink())
        .unwrap_or(false);
    if repair && tmp_dir.exists() {
        if !tmp_meta_ok {
            hard_failure = true;
            report.tmp_dir_ok = false;
            if !json {
                println!(
                    "  [FAIL] store tmp is not a real directory ({}): refusing \
                     to sweep through a redirect",
                    tmp_dir.display()
                );
                println!(
                    "  repair hint: replace the tmp entry with a real \
                     directory, then re-run mgc store doctor --repair"
                );
            }
        } else {
            let day = std::time::Duration::from_secs(24 * 60 * 60);
            let mut removed = 0usize;
            let mut skipped_foreign = 0usize;
            for entry in std::fs::read_dir(tmp_dir)? {
                let entry = entry?;
                // No-follow: the ENTRY's own metadata decides — a symlinked
                // child is skipped, never traversed.
                // (No-follow: metadata của chính ENTRY quyết định — con là
                // symlink bị bỏ qua, không đệ quy.)
                let Ok(meta) = entry.metadata() else {
                    continue;
                };
                if meta.file_type().is_symlink() {
                    skipped_foreign += 1;
                    continue;
                }
                // SHARED PARSER (Gate 11-B / P0-7, vòng-11): the old loose
                // heuristic (`has '-', ≥4 parts, digits tail`) matched
                // foreign dashed names like `customer-important-backup-123`
                // and DELETED them. Now the doctor uses the SAME strict
                // parser as `unique_tmp_path` (prefix allowlist + exact
                // `<pid>-<tid_hex>-<nanos>-<counter>` arity) and additionally
                // requires the name be sweepable in THIS directory
                // (`import-file`/`write-bytes` — the only prefixes that
                // legally stage under CAS tmp). Everything else is foreign
                // and untouched.
                // (PARSER DÙNG CHUNG (P0-7): heuristic lỏng cũ (`có '-',
                // ≥4 phần, đuôi số) khớp cả tên ngoài như
                // `customer-important-backup-123` và XÓA nó. Giờ doctor
                // dùng parser NGHIÊM NGẶT giống `unique_tmp_path` (allowlist
                // prefix + đúng arity `<pid>-<tid_hex>-<nanos>-<counter>`)
                // và thêm điều kiện tên được quét ở THƯ MỤC NÀY
                // (`import-file`/`write-bytes` — prefix duy nhất hợp pháp
                // staging dưới tmp CAS). Mọi tên khác là của ngoài, không đụng.)
                let file_name = entry.file_name();
                let Some(name) = file_name.to_str() else {
                    skipped_foreign += 1;
                    continue;
                };
                let sweepable = mgc_store::cas::TempFileName::parse(name)
                    .is_some_and(|parsed| parsed.cas_tmp_sweepable());
                if !sweepable {
                    skipped_foreign += 1;
                    continue;
                }
                let stale = meta
                    .modified()
                    .ok()
                    .and_then(|m| m.elapsed().ok())
                    .is_some_and(|age| age > day);
                if stale {
                    let p = entry.path();
                    let removed_ok = if meta.is_dir() {
                        std::fs::remove_dir_all(&p)
                    } else {
                        std::fs::remove_file(&p)
                    };
                    if removed_ok.is_ok() {
                        removed += 1;
                    }
                }
            }
            report.repaired = removed > 0;
            if !json {
                println!("  [ok] orphan-temp GC removed {removed} stale file(s)");
                if skipped_foreign > 0 {
                    println!(
                        "  [ok] skipped {skipped_foreign} non-MGC-named or \
                         symlinked entr(ies) — never swept"
                    );
                }
            }
        }
    }
    Ok(hard_failure)
}
