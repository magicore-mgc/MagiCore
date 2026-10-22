//! Coordinates database checks, root safety, bounded repair and final Web store verdict.
//! Điều phối database, an toàn root, repair giới hạn và verdict Web store.

use crate::commands::store::doctor_report::DoctorReport;
use std::path::Path;

/// Store health check + remediation (P1-6 + P0-C, Tech Lead vòng-7):
/// every fail-closed refusal (prune, install) points HERE. The doctor
/// verifies the two pillars the refcount system stands on:
///
/// 1. store.db opens, its schema parses, and live CAS refs are readable;
/// 2. the CAS root itself passes the same safety validation the store
///    applies at open (real dir, no symlink ancestry — P0-A/P0-B).
///
/// `--repair` additionally clears stale orphan temps under the CAS tmp/
/// (crash leftovers — safe to delete by age) and reports anything it
/// cannot fix. The doctor NEVER deletes blobs: without a readable
/// refcount, deletion is exactly what P0-C forbids.
///
/// Kiểm tra sức khỏe store + hướng sửa (P1-6 + P0-C): mọi lời từ chối
/// fail-closed (prune, install) trỏ VÀO ĐÂY. Doctor verify hai trụ cột
/// mà hệ refcount đứng trên:
///
/// 1. store.db mở được, schema parse được, CAS ref sống đọc được;
/// 2. root CAS tự nó qua cùng bước validate an toàn mà store áp lúc mở
///    (dir thật, không symlink ancestor — P0-A/P0-B).
///
/// `--repair` dọn thêm temp mồ côi stale dưới CAS tmp/ (tàn dư crash —
/// xóa theo tuổi là an toàn) và báo mọi thứ không sửa được. Doctor
/// KHÔNG BAO GIỜ xóa blob: không đọc được refcount thì xóa đúng là điều
/// P0-C cấm.
pub(in crate::commands::store) fn doctor(
    project_root: &Path,
    repair: bool,
    core: &str,
    framework: Option<&str>,
    json: bool,
) -> anyhow::Result<()> {
    let web_cache = project_root.join(".magicore").join("cache").join("web");
    let db_path = super::super::store_paths::store_db_for(project_root)?;
    let cas_root = web_cache.join("cas");
    let tmp_dir = web_cache.join("tmp");
    let locks_root = super::super::store_paths::locks_dir_for(project_root);
    let grace_secs = super::super::staging_lease::staging_grace_secs();
    let mut report = DoctorReport {
        core: core.to_string(),
        framework: framework.map(str::to_string),
        scope: "web-cas-and-generation-integrity",
        integrity_checked: true,
        store_root: web_cache.display().to_string(),
        db_ok: false,
        cas_root_ok: false,
        tmp_dir_ok: true,
        repaired: false,
        healthy: false,
        generation_state_ok: false,
        orphan_claims: 0,
        stale_staging_generations: 0,
        retained_staging_generations: 0,
        missing_referenced_blobs: 0,
        corrupt_referenced_blobs: 0,
    };
    if !json {
        println!("store doctor: {}", web_cache.display());
    }
    let db = super::super::doctor_database_integrity::inspect_database_integrity(
        &db_path,
        &cas_root,
        &locks_root,
        grace_secs,
        json,
        &mut report,
    )?;
    let mut hard_failure = db.hard_failure;
    let multi_promoted = db.multi_promoted;
    hard_failure |=
        super::super::doctor_root_safety::inspect_cas_root_safety(&cas_root, json, &mut report);
    hard_failure |=
        super::super::doctor_temp_repair::repair_orphan_temps(&tmp_dir, repair, json, &mut report)?;
    super::super::doctor_staging_repair::repair_stale_staging(
        &db_path,
        &locks_root,
        grace_secs,
        repair,
        json,
        &mut report,
    )?;
    super::super::doctor_verdict::finalize_doctor_verdict(
        core,
        framework,
        json,
        hard_failure,
        multi_promoted,
        &mut report,
    )
}
