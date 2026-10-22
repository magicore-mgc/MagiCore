//! Kotlin native OSV audit entry point.
//! Entry point audit OSV native cho Kotlin.

use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Kotlin security audit uses the native lock/manifest OSV reader and never
/// launches Gradle. Incomplete dependency metadata remains partial/unsupported.
/// Audit Kotlin dùng reader OSV native; không khởi chạy Gradle.
pub async fn audit_kotlin(project_root: &Path) -> MgResult<AuditReport> {
    // Kotlin projects use the native lock/manifest OSV reader. This path
    // never starts Gradle; unresolved Gradle models remain Partial or
    // Unsupported instead of being delegated to a build task.
    // Kotlin dùng reader native lock/manifest + OSV; không khởi chạy Gradle.
    mgc_audit::scanners::audit_java(project_root).await
}
