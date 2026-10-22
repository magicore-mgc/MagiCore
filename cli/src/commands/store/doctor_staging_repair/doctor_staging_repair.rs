//! Retires only staging generations proven stale and safe to abort.
//! Chỉ nghỉ hưu staging generation được chứng minh stale và an toàn để hủy.

use super::super::staging_lease::classify_staging_leases;
use std::path::Path;

pub(in crate::commands::store) fn repair_stale_staging(
    db_path: &Path,
    locks_root: &Path,
    grace_secs: i64,
    repair: bool,
    json: bool,
    report: &mut super::super::doctor_report::DoctorReport,
) -> anyhow::Result<()> {
    // Stale-staging repair (Gate 11-B, vòng-11 — lease-aware): the OLD
    // GC only retired claim-LESS staging markers; the verdict demands
    // lease/pid awareness. Contract:
    //   (a) claim-less staging + lease pid DEAD (or ancient beyond
    //       grace) → garbage → --repair aborts it;
    //   (b) claim-less staging + pid ALIVE → a begin in flight — NEVER
    //       touched (report only);
    //   (c) claim-ful staging → either a live install or a crashed one
    //       still over-protecting blobs — NEVER touched here (its own
    //       abort / a future GC with a full claim-map retires it).
    // pid liveness: kill(pid, 0) semantics via the `sysinfo`-free route —
    // a zero-signal probe. On non-unix the probe is unavailable → the
    // doctor falls back to the grace window alone (still conservative).
    // (Sửa staging chết — có tri lease: GC cũ chỉ nghỉ hưu staging
    // KHÔNG claim; phán quyết đòi nhận thức lease/pid. Hợp đồng:
    // (a) staging không claim + pid lease CHẾT (hoặc quá cũ quá cửa sổ) →
    // rác → --repair hủy; (b) staging không claim + pid CÒN SỐNG →
    // begin đang bay — KHÔNG BAO GIỜ đụng (chỉ báo); (c) staging CÓ
    // claim → hoặc install đang chạy hoặc install đứt vẫn bảo vệ blob
    // — KHÔNG BAO GIỜ đụng ở đây (abort của chính nó / GC tương lai với
    // claim-map đầy đủ mới nghỉ hưu).)
    //
    // P2-1 (fresh-context review 2026-09-15): the GC now takes the
    // project's INSTALL LOCK before aborting a staging generation of
    // that project. Rationale: the pid probe (kill(pid,0)) is Unix-only —
    // on Windows `pid_alive_on_this_os` returns None→false, so a
    // CLAIM-LESS IN-FLIGHT begin (lease fresh, claims not yet filed)
    // could be aborted MID-INSTALL by --repair; the install's next
    // cas_claim would then die with UnknownToken. The install lock is
    // the serialization boundary that closes the hole cross-platform:
    // an install holding the lock OWNS its staging generation; the
    // doctor can only retire staging of installs that are provably NOT
    // running (no lock held).
    // (P2-1: GC giờ lấy INSTALL LOCK của project trước khi hủy staging
    // generation của project đó. Lý do: pid probe chỉ có trên Unix —
    // trên Windows probe trả None→false, nên begin ĐANG CHẠY không claim
    // (lease mới, chưa ghi claim) có thể bị --repair hủy GIỮA CHỪNG;
    // cas_claim kế tiếp của install chết với UnknownToken. Install lock
    // là ranh giới xếp tuần tự đóng lỗ hổng cross-platform: install giữ
    // lock SỞ HỮU staging generation của nó; doctor chỉ nghỉ hưu staging
    // của install chứng minh được là KHÔNG chạy (không giữ lock).)
    //
    // P2-2 (fresh-context review 2026-09-15): claim-ful staging of a
    // DEAD install was NEVER retired before — it over-protected blobs
    // forever (safe, unbounded). The doctor now also retires the
    // evidence-backed subset via list_staging_leases_covered_by_promoted:
    // a claim-ful staging whose EVERY claimed hash is still claimed by a
    // PROMOTED generation of the same project loses nothing by being
    // aborted (the promoted refset subsumes it). Claim-ful staging with
    // hashes NOT covered by any promoted generation stays untouched —
    // those blobs would lose their only protection.
    // (P2-2: staging CÓ claim của install CHẾT chưa bao giờ được nghỉ
    // hưu — nó bảo vệ blob vô hạn (an toàn, không giới hạn). Doctor giờ
    // còn nghỉ hưu tập có bằng chứng qua list_staging_leases_covered_
    // by_promoted: staging có claim mà MỌI hash nó claim vẫn được
    // generation PROMOTED của cùng project claim thì hủy đi không mất
    // gì (refset promoted thâu hẹp nó). Staging có claim mà hash KHÔNG
    // được promoted nào giữ thì không đụng — blob đó mất bảo vệ duy
    // nhất.)
    if repair && report.db_ok {
        let db = mgc_store::Database::open(db_path)?;
        // Shared classifier (single source of truth) — the SAME policy the
        // count path used above. Retire exactly the STALE subset; Live and
        // Retained are left alone (an in-flight install / a blob's only
        // protection must never be aborted).
        // (Classifier dùng chung (nguồn sự thật duy nhất) — CÙNG chính
        // sách path đếm dùng ở trên. Nghỉ hưu đúng tập STALE; Live và
        // Retained để nguyên (install đang bay / bảo vệ duy nhất của blob
        // không bao giờ bị hủy).)
        let classification = classify_staging_leases(&db, locks_root, grace_secs)?;
        let mut retired = 0usize;
        for lease in &classification.stale {
            // TOCTOU: abort_cas_generation is token-gated and atomic on the
            // OLD generation number; a new install always allocates a NEW
            // token, so retiring a STALE generation is safe even if another
            // install started between classify and abort. The lock gate in
            // the classifier already guarantees we never touch a LIVE one.
            // (TOCTOU: abort_cas_generation có cổng token và nguyên tử trên
            // số generation CŨ; install mới luôn cấp token MỚI, nên nghỉ
            // hưu generation STALE an toàn kể cả khi install khác bắt đầu
            // giữa classify và abort. Cổng lock trong classifier đã bảo
            // đảm không bao giờ đụng generation LIVE.)
            if db
                .abort_cas_generation(&lease.project_root, lease.generation)
                .is_ok()
            {
                retired += 1;
            }
        }
        report.repaired = report.repaired || retired > 0;
        // Post-repair stale ledger: the invariants line above printed the
        // PRE-repair evidence (what the doctor FOUND); this field documents
        // what REMAINS, so the --json consumer and the final verdict read
        // the post-repair truth.
        // (Sổ stale sau-repair: dòng invariants phía trên in bằng chứng
        // TRƯỚC-repair (doctor TÌM THẤY gì); field này ghi còn LẠI gì, để
        // consumer --json và verdict cuối đọc sự thật sau-repair.)
        report.stale_staging_generations = report.stale_staging_generations.saturating_sub(retired);
        if !json {
            println!(
                "  [ok] lease GC retired {retired} leaked staging generation(s) \
                 (skipped: {} live, {} retained over-retention)",
                classification.live, classification.retained,
            );
        }
    }
    Ok(())
}
