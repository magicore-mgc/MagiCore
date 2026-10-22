//! Stable Dart scanner entry points.
//! Entry point ổn định của scanner Dart.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Audit Flutter packages from `pubspec.lock` through the shared OSV service.
/// Audit package Flutter từ `pubspec.lock` qua dịch vụ OSV dùng chung.
pub async fn audit_flutter_osv(project_root: &Path) -> MgResult<AuditReport> {
    super::audit_flutter_osv::run(project_root).await
}

/// Read the generated Pub lockfile subset and return pins plus skipped sources.
/// Đọc tập con lockfile Pub được sinh và trả về bản ghim cùng nguồn bị bỏ qua.
pub fn read_pubspec_lock(raw: &str) -> MgResult<(Vec<crate::scanners::osv::OsvPin>, Vec<String>)> {
    super::pubspec_lock::read(raw)
}

#[cfg(test)]
#[path = "test/dart.rs"]
mod tests;
