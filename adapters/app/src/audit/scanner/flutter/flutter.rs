//! Flutter audit and dependency-health entry points.
//! Entry point audit bảo mật và dependency health của Flutter.

use mgc_types::adapter::{AuditReport, DependencyHealthReport};
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Flutter freshness remains unsupported until MagiCore owns the registry query.
/// Chưa hỗ trợ freshness Flutter cho tới khi MGC tự truy vấn registry.
pub async fn dependency_health_flutter(project_root: &Path) -> MgResult<DependencyHealthReport> {
    let _ = project_root;
    Err(MgError::Unsupported {
        core: "app",
        capability: "flutter_dependency_health",
        guidance: "MagiCore has no native pub.dev freshness resolver yet; this command will not invoke Flutter or report a guessed result".to_string(),
    })
}

/// Security audit for Flutter delegates to the shared OSV scanner when
/// `pubspec.lock` exists; without the lockfile, the result stays unsupported.
/// Audit bảo mật Flutter gọi scanner OSV dùng chung khi có `pubspec.lock`;
/// nếu thiếu lockfile, kết quả vẫn là unsupported.
pub async fn audit_flutter(project_root: &Path) -> MgResult<AuditReport> {
    // P2 2026-09-10: pubspec.lock pins → OSV.dev `pub` ecosystem — a
    // REAL CVE scan; without the lockfile the honest unsupported state
    // stays (pub outdated is freshness, not security).
    // pubspec.lock ghim → OSV.dev ecosystem `pub` — scan CVE THẬT; thiếu
    // lockfile giữ trạng thái unsupported trung thực (pub outdated chỉ
    // là độ tươi, không phải bảo mật).
    mgc_audit::scanners::audit_flutter_osv(project_root).await
}
