//! Dispatch a Lib audit request to its native or shared scanner lane.
//! Điều phối yêu cầu audit Lib vào lane scanner native hoặc dùng chung.

use mgc_types::adapter::AuditReport;
use mgc_types::{MgError, MgResult};
use std::path::Path;

use crate::audit::aggregate::{
    aggregate_dotnet_rust, aggregate_go_rust, aggregate_java_rust, aggregate_python_rust,
};
use crate::audit::discovery::pylock_variant_exists;
use crate::audit::scanner;
use crate::language::LibLanguage;

/// Run security audit for lib project.
/// Chạy security audit cho lib project.
pub(crate) async fn run_audit(language: LibLanguage, project_root: &Path) -> MgResult<AuditReport> {
    match language {
        LibLanguage::Ts => {
            // TypeScript: delegate to web adapter audit
            // TypeScript: ủy quyền cho web adapter audit
            Err(MgError::Other(
                "TypeScript audit should be delegated to web adapter".to_string(),
            ))
        }
        LibLanguage::Rust => scanner::audit_rust(project_root).await,
        LibLanguage::Python => {
            // A polyglot lib (pyproject + Cargo) must scan BOTH sets —
            // aggregate, never first-match.
            // Lib đa ngôn ngữ (pyproject + Cargo) phải quét CẢ HAI tập —
            // tổng hợp, không first-match.
            if project_root.join("Cargo.toml").is_file()
                && (project_root.join("requirements.txt").is_file()
                    || project_root.join("pylock.toml").is_file()
                    || pylock_variant_exists(project_root)
                    || project_root.join("uv.lock").is_file()
                    || project_root.join("pyproject.toml").is_file())
            {
                aggregate_python_rust(project_root).await
            } else {
                scanner::audit_python(project_root).await
            }
        }
        LibLanguage::Go => {
            // Go lane (P1 matrix row "Lib Go govulncheck"): shared scanner
            // through the engine; a go.mod + Cargo.toml polyglot
            // aggregates BOTH.
            // Lane Go (P1 matrix "Lib Go govulncheck"): scanner chung qua
            // engine; polyglot go.mod + Cargo.toml gộp CẢ HAI.
            if project_root.join("Cargo.toml").is_file() {
                aggregate_go_rust(project_root).await
            } else {
                scanner::audit_go(project_root).await
            }
        }
        // Java/Kotlin + .NET lanes (P2 2026-09-10 matrix rows "Lib
        // Java/Kotlin Gradle/Maven Dependency Check" + "Lib .NET dotnet
        // list package --vulnerable"): OSV-backed shared scanners —
        // verification-metadata.xml / packages.lock.json pins, no heavy
        // CLI needed. Polyglot roots (Cargo.toml kề) aggregate BOTH.
        // Lane Java/Kotlin + .NET: scanner chung chạy OSV — ghim từ
        // lockfile, không cần CLI nặng. Root polyglot (có Cargo.toml
        // kề) gộp CẢ HAI.
        LibLanguage::Java => {
            if project_root.join("Cargo.toml").is_file() {
                aggregate_java_rust(project_root).await
            } else {
                mgc_audit::scanners::audit_java(project_root).await
            }
        }
        LibLanguage::DotNet => {
            if project_root.join("Cargo.toml").is_file() {
                aggregate_dotnet_rust(project_root).await
            } else {
                mgc_audit::scanners::audit_dotnet(project_root).await
            }
        }
    }
}
