// Project one advisory onto locked versions. / Chiếu một advisory lên các version đã lock.
use mgc_types::adapter::{FindingClass, Vulnerability, VulnerabilitySeverity};
use mgc_types::{PackageId, PackageName, Version, VersionRange};

use super::super::{AdvisoryRecord, validate_semver_range};

/// Build findings for ONE advisory: parse the npm Bulk API record, then
/// match every installed lockfile version of that package against
/// `vulnerable_versions`. Zero matching versions → the advisory does not
/// apply to this tree → no finding (legitimate, unlike a parse error).
/// Dựng finding cho MỘT advisory: parse record npm Bulk API, rồi đối
/// chiếu mọi version lockfile của package đó với `vulnerable_versions`.
/// Không version nào khớp → advisory không áp dụng cho tree này → không
/// finding (hợp lệ, khác với lỗi parse).
pub(in crate::audit) fn build_findings_for_advisory(
    pkg_name: &str,
    advisory: &serde_json::Value,
    pins: &[(String, String)],
) -> Result<Vec<Vulnerability>, String> {
    let record: AdvisoryRecord =
        serde::Deserialize::deserialize(advisory).map_err(|e| format!("schema mismatch: {e}"))?;

    let range = VersionRange::parse(&record.vulnerable_versions).map_err(|e| {
        format!(
            "invalid vulnerable_versions '{}': {e}",
            record.vulnerable_versions
        )
    })?;
    // Strict semver validation for audit (unlike resolver ranges, an
    // advisory range we cannot interpret must NOT silently match
    // nothing — that would hide real findings behind garbage ranges).
    // Kiểm tra semver chặt cho audit (khác range resolver): range
    // advisory không diễn giải được KHÔNG được phép khớp rỗng một cách
    // âm thầm — sẽ giấu finding thật sau range rác.
    validate_semver_range(&record.vulnerable_versions)?;

    let mut findings = Vec::new();
    for (_pin_name, pin_version) in pins.iter().filter(|(n, _)| n == pkg_name) {
        let installed = Version::parse(pin_version)
            .map_err(|e| format!("invalid installed version '{pin_version}': {e}"))?;
        if !range.matches(&installed) {
            continue;
        }
        findings.push(
            Vulnerability {
                package: PackageId::new(
                    PackageName::new(pkg_name.to_string())
                        .map_err(|e| format!("invalid package name: {e}"))?,
                    installed,
                ),
                title: record.title.clone(),
                severity: record.severity.clone(),
                // The registry numeric `id` is the advisory identifier (same
                // thing npm audit prints); there is no CVE array on this API.
                // `id` số của registry chính là định danh advisory (npm audit
                // cũng in vậy); API này không có mảng CVE.
                cve: record.id.to_string(),
                severity_level: VulnerabilitySeverity::from_str(&record.severity),
                patched_versions: Some(record.vulnerable_versions.clone()),
                url: Some(record.url.clone()),
                scanner: None,
                ecosystem: None,
                evidence_at: None,
                finding_class: FindingClass::Vulnerability,
            }
            .with_evidence("npm-bulk-advisory", "web/javascript"),
        );
    }
    Ok(findings)
}
