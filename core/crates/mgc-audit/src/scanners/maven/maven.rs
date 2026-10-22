//! Coordinates Java and Maven audit source selection.
//! Điều phối chọn đầu vào audit Java và Maven.

use super::query_maven_pins::query_maven_pins;
use super::{read_gradle_verification_metadata, read_pom_gavs};
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

/// Java audit: gradle verification metadata → OSV `Maven` queries,
/// falling back to Maven pom.xml dependencies (Spring Boot and plain
/// Maven projects). Without any lockfile the honest unsupported state
/// stays.
/// Audit Java: metadata verification Gradle, rơi về dependency pom.xml
/// → truy vấn OSV `Maven`. Thiếu lockfile giữ unsupported trung thực.
pub async fn audit_java(project_root: &Path) -> MgResult<AuditReport> {
    let verification = project_root
        .join("gradle")
        .join("verification-metadata.xml");
    if verification.is_file() {
        let raw =
            super::super::read_bounded_project_input(&verification, "verification-metadata.xml")?;
        return query_maven_pins(&read_gradle_verification_metadata(&raw)).await;
    }
    // Maven/Spring lane: pom.xml dependencies carry exact versions for
    // fixed pins; property/versionless entries skip honestly.
    // Lane Maven/Spring: dependency pom.xml ghim version cố định; entry
    // property/thiếu version skip có ghi nhận.
    let pom = project_root.join("pom.xml");
    if pom.is_file() {
        let raw = super::super::read_bounded_project_input(&pom, "pom.xml")?;
        let mut report = query_maven_pins(&read_pom_gavs(&raw)).await?;
        // P0-4: pom.xml lists DIRECT dependencies only — the transitive
        // graph (parents, BOMs, profiles, dependencyManagement) is NOT
        // resolved by the line reader, so a pom-only scan can never be
        // Available-clean: force Partial with the coverage note, keeping
        // every finding. (The verification-metadata.xml path above IS a
        // resolved lockfile and keeps its own status.)
        // pom.xml chỉ liệt kê direct dependency — scan pom-only không
        // bao giờ Available: ép Partial giữ nguyên findings.
        if matches!(
            report.scanner_status,
            mgc_types::adapter::ScannerStatus::Available
        ) {
            report.scanner_status = mgc_types::adapter::ScannerStatus::Partial {
                scanned: report.packages_audited,
                skipped: 0,
                reasons: vec![
                    "pom.xml covers direct dependencies only — transitive graph (parents/BOMs/profiles) is not resolved by MagiCore; a complete native audit is unavailable for this project shape"
                        .to_string(),
                ],
            };
        }
        return Ok(report);
    }
    let has_gradle = project_root.join("build.gradle").is_file()
        || project_root.join("build.gradle.kts").is_file();
    Ok(AuditReport::unsupported_ecosystem(if has_gradle {
        "lib/java (Gradle dependency graph is not natively resolved by MagiCore; audit is unsupported without a MagiCore-owned resolved lock graph)"
    } else {
        "lib/java (no gradle project or pom.xml detected)"
    }))
}

#[cfg(test)]
#[path = "test/maven.rs"]
mod tests;
