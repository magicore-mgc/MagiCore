//! Checks database readability, generation invariants and referenced-blob digests.
//! Kiểm tra database, invariant generation và digest blob được tham chiếu.

use super::super::staging_lease::classify_staging_leases;
use anyhow::Result;
use std::path::Path;

pub(in crate::commands::store) struct DatabaseIntegrityResult {
    pub(in crate::commands::store) hard_failure: bool,
    pub(in crate::commands::store) multi_promoted: i64,
}
pub(in crate::commands::store) fn inspect_database_integrity(
    db_path: &Path,
    cas_root: &Path,
    locks_root: &Path,
    grace_secs: i64,
    json: bool,
    report: &mut super::super::doctor_report::DoctorReport,
) -> Result<DatabaseIntegrityResult> {
    let mut hard_failure = false;
    let mut multi_promoted: i64 = 0;
    // Pillar 1 — DB readable + refset loadable + GENERATION INVARIANTS
    // (Gate 11-B, vòng-11): the old doctor proved only "SQLite opens".
    // Store integrity needs the token invariants too:
    //   (a) no claim without a generation marker (orphan);
    //   (b) no stale STAGING generation older than a grace window
    //       (crashed installs — over-retention is safe but must be
    //       VISIBLE, and --repair may retire them);
    //   (c) at most one PROMOTED generation per project (uniqueness);
    //   (d) every referenced blob EXISTS on disk and rehashes to its
    //       digest (missing/corrupt).
    // (Trụ 1 — DB đọc được + refset load được + INVARIANT GENERATION:
    // doctor cũ chỉ chứng minh "SQLite mở được". Sức khỏe store cần cả
    // invariant token: (a) không claim thiếu marker (mồ côi); (b) không
    // staging generation chết quá cửa sổ xử lý (install đứt — giữ thừa
    // an toàn nhưng phải HIỆN, --repair được nghỉ hưu); (c) tối đa MỘT
    // generation promoted mỗi project; (d) mọi blob được tham chiếu
    // TỒN TẠI và rehash đúng digest (mất/hỏng).)
    if db_path.exists() {
        match mgc_store::Database::open(db_path)
            .and_then(|db| db.list_cas_live_refs().map(|live| (db, live)))
        {
            Ok((db, live)) => {
                report.db_ok = true;
                if !json {
                    println!("  [ok] store.db readable — {} live CAS ref(s)", live.len());
                }
                // (c) promoted uniqueness via SQL + (b) stale/retained via
                // the shared lease classifier (Gate 11-B P0-1). The old SQL
                // counted "stale" as claim-less staging ONLY — it ignored
                // lease/pid/lock entirely and never flagged a claim-ful
                // crashed staging whose hashes are all promoted-covered
                // (the exact leak the P2-2 GC retires). The classifier now
                // splits LIVE / STALE / RETAINED with one policy.
                // ((c) duy nhất promoted qua SQL + (b) stale/retained qua
                // classifier lease dùng chung (P0-1). SQL cũ tính "stale"
                // chỉ theo staging KHÔNG claim — bỏ qua lease/pid/lock và
                // không bao giờ cờ staging đứt CÓ claim mà hash đều được
                // promoted giữ (đúng chỗ rò mà GC P2-2 nghỉ hưu). Classifier
                // giờ tách LIVE / STALE / RETAINED bằng một chính sách.)
                let integrity_counts = db.cas_integrity_counts()?;
                multi_promoted = integrity_counts.multi_promoted_generations;
                let classification = classify_staging_leases(&db, locks_root, grace_secs)?;
                report.stale_staging_generations = classification.stale.len();
                report.retained_staging_generations = classification.retained;
                // (a) orphan claims: rows whose (project, generation) has
                // no marker — impossible with FK on, a violation means
                // hand-edited DB.
                // ((a) claim mồ côi: row mà (project, generation) thiếu
                // marker — bất khả khi FK bật, vi phạm nghĩa là DB bị sửa tay.)
                report.orphan_claims = usize::try_from(integrity_counts.orphan_claims)
                    .map_err(|_| anyhow::anyhow!("orphan claim count exceeds platform capacity"))?;
                // (d) referenced blobs exist + rehash. Claims store the bare
                // hex; the blob lives at cas/files/blake3/<xx>/<hash> (+
                // `.exec` variant — either counts as present).
                // ((d) blob được tham chiếu tồn tại + rehash. Claim lưu hex
                // thuần; blob nằm ở cas/files/blake3/<xx>/<hash> (biến thể
                // `.exec` — một trong hai là có).)
                let mut missing = 0usize;
                let mut corrupt = 0usize;
                if !live.is_empty() && cas_root.exists() {
                    for hash in &live {
                        let blob = cas_root
                            .join("files")
                            .join("blake3")
                            .join(&hash[..2])
                            .join(hash);
                        let exec_blob = {
                            let mut p = blob.clone().into_os_string();
                            p.push(".exec");
                            std::path::PathBuf::from(p)
                        };
                        let path = if blob.exists() {
                            blob
                        } else if exec_blob.exists() {
                            exec_blob
                        } else {
                            missing += 1;
                            continue;
                        };
                        // Rehash verify — doctor's whole point (trust the
                        // refset, so verify the blob it vouches for).
                        // P3 (fresh-context review 2026-09-15): STREAMING
                        // hash — `fs::read` loaded the whole blob into
                        // RAM; a store with AI-model/game-asset blobs
                        // made the doctor's RSS balloon. hash_file
                        // streams an 8-KiB buffer; same BLAKE3 digest.
                        // (Rehash verify — điểm cốt lõi của doctor (tin
                        // refset thì phải verify blob nó bảo chứng).
                        // P3: hash STREAMING — `fs::read` load cả blob
                        // vào RAM; store có blob cỡ AI-model/game-asset
                        // làm RSS của doctor phồng. hash_file stream
                        // buffer 8-KiB; cùng digest BLAKE3.)
                        match mgc_crypto::Blake3Hasher::hash_file(&path) {
                            Ok(digest) => {
                                if digest.to_hex() != *hash {
                                    corrupt += 1;
                                }
                            }
                            Err(_) => missing += 1,
                        }
                    }
                }
                report.missing_referenced_blobs = missing;
                report.corrupt_referenced_blobs = corrupt;
                // Pre-repair verdict (Gate 11-B P0-1): stale staging now
                // FAILS health — a leaked begin / redundant claim is garbage
                // that must be retired, not silently tolerated. Retained is
                // SAFE over-retention and stays out of the verdict.
                // (Verdict trước-repair (P0-1): staging stale giờ LÀM HỎNG
                // health — begin rò / claim thừa là rác phải nghỉ hưu, không
                // dung thứ im lặng. Retained là giữ thừa AN TOÀN, nằm ngoài
                // verdict.)
                let generation_ok = report.orphan_claims == 0
                    && multi_promoted == 0
                    && missing == 0
                    && corrupt == 0
                    && report.stale_staging_generations == 0;
                report.generation_state_ok = generation_ok;
                if multi_promoted != 0 || report.orphan_claims != 0 || missing != 0 || corrupt != 0
                {
                    hard_failure = true;
                }
                if !json {
                    println!(
                        "  [{}] generation invariants: {} orphan claim(s), {} stale staging gen(s), {} missing blob(s), {} corrupt blob(s)",
                        if generation_ok { "ok" } else { "FAIL" },
                        report.orphan_claims,
                        report.stale_staging_generations,
                        report.missing_referenced_blobs,
                        report.corrupt_referenced_blobs
                    );
                    if report.retained_staging_generations > 0 {
                        println!(
                            "  [ok] {} retained staging gen(s) (safe over-retention — not corruption)",
                            report.retained_staging_generations
                        );
                    }
                    if report.stale_staging_generations > 0 {
                        println!(
                            "  repair hint: run mgc store doctor --repair to retire leaked staging generation(s)"
                        );
                    }
                }
            }
            Err(e) => {
                hard_failure = true;
                if !json {
                    println!("  [FAIL] store.db unreadable: {e}");
                    println!(
                        "  repair hint: restore store.db from a backup \
                         (mgc store backup/restore), or move the corrupted db \
                         aside and re-install projects to rebuild claims"
                    );
                }
            }
        }
    } else {
        hard_failure = true;
        if !json {
            println!("  [FAIL] store.db missing at {}", db_path.display());
            println!(
                "  repair hint: re-run mgc install in the project to rebuild \
                 the refset (prune stays disabled until then)"
            );
        }
    }
    Ok(DatabaseIntegrityResult {
        hard_failure,
        multi_promoted,
    })
}
