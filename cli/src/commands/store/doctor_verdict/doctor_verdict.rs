//! Computes and renders doctor health and exit-code result.
//! Tính và hiển thị sức khỏe doctor cùng kết quả exit-code.

pub(in crate::commands::store) fn finalize_doctor_verdict(
    core: &str,
    framework: Option<&str>,
    json: bool,
    hard_failure: bool,
    multi_promoted: i64,
    report: &mut super::super::doctor_report::DoctorReport,
) -> anyhow::Result<()> {
    // Final verdict (P0-B + Gate 11-B P0-1): healthy iff no hard failure
    // (unreadable DB, orphan/multi-promoted/missing/corrupt, unsafe CAS
    // root/tmp) AND no stale staging remains (repair may have retired it).
    // Retained staging is SAFE over-retention and does not fail health.
    // (Verdict cuối (P0-B + P0-1): khỏe iff không lỗi cứng (DB không đọc
    // được, orphan/multi-promoted/missing/corrupt, CAS root/tmp không an
    // toàn) VÀ không còn staging stale (repair có thể đã nghỉ hưu). Staging
    // retained là giữ thừa AN TOÀN, không làm hỏng health.)
    let generation_ok = report.orphan_claims == 0
        && multi_promoted == 0
        && report.missing_referenced_blobs == 0
        && report.corrupt_referenced_blobs == 0
        && report.stale_staging_generations == 0;
    report.generation_state_ok = generation_ok;
    report.healthy = !hard_failure && generation_ok;

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!(
            "store doctor: {}",
            if report.healthy {
                "HEALTHY"
            } else {
                "NEEDS REPAIR — prune/install refuse until fixed"
            }
        );
    }

    // EXIT CONTRACT (P0-B): healthy → Ok; broken → anyhow error so the CLI
    // exits 1 and `mgc store doctor && <next>` gates correctly. A broken
    // store must never read as success.
    // (Hợp đồng exit (P0-B): khỏe → Ok; hỏng → error anyhow để CLI exit 1
    // và `mgc store doctor && <bước sau>` chặn đúng. Store hỏng không
    // bao giờ được đọc là thành công.)
    if report.healthy {
        Ok(())
    } else {
        Err(crate::error::store_doctor_unhealthy(core, framework))
    }
}
