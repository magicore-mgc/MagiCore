//! Classifies leases by process liveness, age, install lock and promoted coverage.
//! Phân loại lease theo PID, tuổi lease, install lock và promoted coverage.

use super::super::process_liveness::pid_alive_on_this_os;
use anyhow::Result;
use std::path::Path;

/// Default staging-lease grace window (seconds): a staging lease older
/// than this whose pid is unverifiable is garbage. Overridable for tests
/// via `MGC_DOCTOR_STAGING_GRACE_SECS` (RULE §12 — no inline magic).
/// (Cửa sổ xử lý lease staging mặc định (giây): lease staging cũ hơn mà
/// pid không kiểm chứng được là rác. Override cho test qua
/// `MGC_DOCTOR_STAGING_GRACE_SECS` (RULE §12 — không literal ẩn).)
pub(in crate::commands::store) const DEFAULT_STAGING_GRACE_SECS: i64 = 24 * 60 * 60;

/// Read the staging grace window from the env override or the default.
/// (Đọc cửa sổ xử lý từ env override hoặc mặc định.)
pub(in crate::commands::store) fn staging_grace_secs() -> i64 {
    std::env::var("MGC_DOCTOR_STAGING_GRACE_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_STAGING_GRACE_SECS)
}

/// Staging-lease disposition under the Gate 11-B policy — the ONE source
/// of truth for both the doctor's health verdict and the --repair GC.
/// (Phân loại lease staging theo chính sách Gate 11-B — nguồn sự thật DUY
/// NHẤT cho cả verdict sức khỏe doctor lẫn GC --repair.)
#[derive(Debug, PartialEq, Eq)]
pub(in crate::commands::store) enum StagingDisposition {
    /// Live: pid alive AND lease fresh, OR the project install lock is
    /// held — an in-flight install must never be counted stale.
    /// (Live: pid sống VÀ lease mới, HOẶC install lock project bị giữ —
    /// install đang bay không bao giờ bị tính stale.)
    Live,
    /// Stale: lease dead (pid gone / lease ancient) with zero claims
    /// (leaked begin) or claims all covered by a promoted generation
    /// (redundant) — retirable garbage that FAILS health until repaired.
    /// (Stale: lease chết (pid mất / lease cổ) không claim (begin rò) hoặc
    /// claim đều được promoted generation giữ (thừa) — rác nghỉ hưu được,
    /// làm hỏng health cho tới khi repair.)
    Stale,
    /// Retained: lease dead but at least one claimed hash has no promoted
    /// generation vouching for it — the claim is that blob's ONLY
    /// protection, so over-retention is SAFE and must NOT fail health.
    /// (Retained: lease chết nhưng có hash chưa được promoted nào giữ —
    /// claim là bảo vệ DUY NHẤT của blob, giữ thừa AN TOÀN, KHÔNG hỏng.)
    Retained,
}

/// Result of classifying every staging lease in a store.
pub(in crate::commands::store) struct StagingClassification {
    pub(in crate::commands::store) live: usize,
    pub(in crate::commands::store) stale: Vec<mgc_store::StagingLease>,
    pub(in crate::commands::store) retained: usize,
}

/// Classify every staging lease of `db` with the SHARED policy: pid
/// liveness (Unix) + lease age + project-install-lock + promoted coverage.
/// Both the doctor's count path and the --repair GC call this — never two
/// diverged copies of the same rules.
/// (Phân loại mọi lease staging của db bằng chính sách DÙNG CHUNG: pid
/// sống (Unix) + tuổi lease + install-lock project + độ phủ promoted. Cả
/// path đếm của doctor lẫn GC --repair đều gọi đây — không 2 bản copy lệch.)
pub(in crate::commands::store) fn classify_staging_leases(
    db: &mgc_store::Database,
    locks_root: &Path,
    grace_secs: i64,
) -> Result<StagingClassification> {
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let leases = db.list_staging_leases()?;
    let covered: std::collections::HashSet<(String, i64)> = db
        .list_staging_leases_covered_by_promoted()?
        .into_iter()
        .map(|l| (l.project_root, l.generation))
        .collect();
    let mut live = 0usize;
    let mut stale = Vec::new();
    let mut retained = 0usize;
    for lease in leases {
        match classify_staging_lease(&lease, &covered, locks_root, now_secs, grace_secs) {
            StagingDisposition::Live => live += 1,
            StagingDisposition::Stale => stale.push(lease),
            StagingDisposition::Retained => retained += 1,
        }
    }
    Ok(StagingClassification {
        live,
        stale,
        retained,
    })
}

/// Classify ONE lease (pure decision logic shared by count + repair).
/// (Phân loại MỘT lease (logic quyết định thuần, dùng chung count + repair).)
pub(in crate::commands::store) fn classify_staging_lease(
    lease: &mgc_store::StagingLease,
    covered: &std::collections::HashSet<(String, i64)>,
    locks_root: &Path,
    now_secs: i64,
    grace_secs: i64,
) -> StagingDisposition {
    let pid_alive = lease
        .lease_pid
        .and_then(|pid| pid_alive_on_this_os(pid as u32))
        .unwrap_or(false);
    let lease_fresh = lease
        .lease_started_at
        .is_some_and(|started| now_secs - started < grace_secs);
    // P2-1: the project install lock is the cross-platform liveness proof
    // (the pid probe is Unix-only). Holding it ⇒ live, regardless of pid.
    // (P2-1: install lock project là bằng chứng sống cross-platform (pid
    // probe chỉ Unix). Giữ nó ⇒ live, bất kể pid.)
    let lock_held =
        mgc_store::ProjectInstallLock::acquire_at(locks_root, &lease.project_root).is_err();
    let covered_by_promoted = covered.contains(&(lease.project_root.clone(), lease.generation));
    classify_staging_lease_pure(
        lease.claim_count,
        pid_alive,
        lease_fresh,
        lock_held,
        covered_by_promoted,
    )
}

/// Pure decision core of the staging-lease classifier (Gate 11-B.2): the
/// same LIVE / STALE / RETAINED rules WITHOUT the pid or lock probes, so the
/// disposition logic is directly unit-testable. `classify_staging_lease`
/// computes the probes and delegates here — never two copies of the rules.
/// (Lõi quyết định thuần của classifier lease staging (Gate 11-B.2): cùng
/// luật LIVE / STALE / RETAINED NHƯNG không có probe pid hay lock, để logic
/// disposition test đơn vị trực tiếp được. `classify_staging_lease` tính
/// probe rồi ủy quyền vào đây — không bao giờ hai bản copy lệch nhau.)
pub(in crate::commands::store) fn classify_staging_lease_pure(
    claim_count: i64,
    pid_alive: bool,
    lease_fresh: bool,
    lock_held: bool,
    covered_by_promoted: bool,
) -> StagingDisposition {
    if (pid_alive && lease_fresh) || lock_held {
        return StagingDisposition::Live;
    }
    if claim_count == 0 || covered_by_promoted {
        StagingDisposition::Stale
    } else {
        StagingDisposition::Retained
    }
}
