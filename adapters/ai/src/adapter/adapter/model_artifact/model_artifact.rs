//! Converts model scanner findings and redacts host-path identity.
//! Chuyển finding model và che danh tính đường dẫn máy.

use mgc_types::adapter::AuditReport;
use mgc_types::{MgResult, PackageId, PackageName, Version};
use std::path::Path;

use super::error::ModelArtifactError;

/// Convert the model-artifact audit (internal Finding format) into the
/// unified AuditReport — every High/Critical model finding becomes a
/// Vulnerability row so the aggregate and exit contract stay uniform.
/// Chuyển audit model-artifact (Finding nội bộ) sang AuditReport thống
/// nhất — mọi finding High/Critical của model thành dòng Vulnerability
/// để aggregate và exit contract giữ một chuẩn.
pub(in crate::adapter::implementation) fn model_artifact_report(
    dir: &Path,
) -> MgResult<AuditReport> {
    use mgc_types::adapter::{FindingClass, Vulnerability, VulnerabilitySeverity};

    let model = futures_util_replay(dir)?;
    let mut vulnerabilities = Vec::new();
    for finding in &model.findings {
        let severity_level = match finding.severity {
            crate::audit::Severity::Critical => VulnerabilitySeverity::Critical,
            crate::audit::Severity::High => VulnerabilitySeverity::High,
            crate::audit::Severity::Medium => VulnerabilitySeverity::Medium,
            crate::audit::Severity::Low => VulnerabilitySeverity::Low,
            crate::audit::Severity::Info => VulnerabilitySeverity::Info,
        };
        let file = finding
            .file_path
            .clone()
            .unwrap_or_else(|| dir.display().to_string());
        vulnerabilities.push(
            Vulnerability {
                package: PackageId::new(model_artifact_package_name(&file)?, Version::new(0, 0, 0)),
                title: format!(
                    "{}: {} ({})",
                    finding.category,
                    finding.message,
                    model_artifact_file_label(&file)
                ),
                severity: format!("{:?}", finding.severity).to_lowercase(),
                cve: format!("model-{}", finding.category),
                severity_level,
                patched_versions: None,
                url: None,
                scanner: None,
                ecosystem: None,
                evidence_at: None,
                finding_class: FindingClass::Artifact,
            }
            .with_evidence("mgc-model-scanner", "model-artifact"),
        );
    }

    Ok(AuditReport {
        packages_audited: model.scanned_files,
        vulnerability_count: vulnerabilities.len(),
        vulnerabilities,
        scanner_status: mgc_types::adapter::ScannerStatus::Available,
    })
}

/// Build a package-safe identity for a model file without putting host paths in package names.
/// Tạo định danh package an toàn cho model mà không đưa đường dẫn máy vào tên package.
pub(in crate::adapter::implementation) fn model_artifact_package_name(
    file: &str,
) -> MgResult<PackageName> {
    use sha2::{Digest, Sha256};

    let path_digest = Sha256::digest(file.as_bytes());
    let package_label = format!("model-artifact-{}", hex::encode(&path_digest[..8]));
    let package_name = PackageName::new(package_label)
        .map_err(|error| ModelArtifactError::InvalidLabel(error.to_string()))?;
    Ok(package_name)
}

/// Keep audit output useful without exposing the host's absolute model path.
/// Giữ output audit hữu ích mà không để lộ đường dẫn model tuyệt đối trên máy.
pub(in crate::adapter::implementation) fn model_artifact_file_label(file: &str) -> &str {
    file.rsplit(['/', '\\'])
        .find(|component| !component.is_empty())
        .unwrap_or("model artifact")
}

/// Poll the async model audit once — same first-poll-complete contract
/// as the dependency scanners.
/// Poll audit model async đúng một lần — cùng hợp đồng poll-đầu-hoàn-
/// tất như các scanner dependency.
fn futures_util_replay(dir: &Path) -> MgResult<crate::audit::AuditReport> {
    use futures_util::future::FutureExt;
    match Box::pin(crate::audit::audit_model(dir)).now_or_never() {
        Some(result) => result,
        None => Err(ModelArtifactError::AsyncExecutionRequired.into()),
    }
}
