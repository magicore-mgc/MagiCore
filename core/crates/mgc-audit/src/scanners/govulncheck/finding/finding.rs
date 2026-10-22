//! Map one Go advisory finding into MagiCore's shared vulnerability record.
//! (Ánh xạ một finding Go sang record vulnerability dùng chung của MagiCore.)

use super::super::model::{GovulncheckFinding, GovulncheckOsv};
use mgc_types::adapter::{FindingClass, Vulnerability};
use mgc_types::{MgResult, PackageId, PackageName, Version};

/// Convert a deduplicated stream finding and its OSV metadata.
/// Chuyển finding stream đã khử trùng cùng metadata OSV.
pub(in crate::scanners::govulncheck) fn vulnerability_from_finding(
    finding: &GovulncheckFinding,
    osv: &GovulncheckOsv,
) -> MgResult<Vulnerability> {
    // First trace frame names the affected module + version; Go module
    // versions may be pseudo-versions ("v1.0.0-20200101...") that are
    // NOT plain semver — keep the finding with a 0.0.0 carrier and
    // the real version recorded in the title (never dropped).
    // Frame đầu của trace nêu module + version dính; version module Go
    // có thể là pseudo-version ("v1.0.0-20200101...") KHÔNG phải
    // semver thuần — giữ finding với carrier 0.0.0 và version thật ghi
    // trong title (không bao giờ bỏ).
    let frame = finding.trace.first();
    let module_path = frame.map(|f| f.module.as_str()).unwrap_or("");
    let module_version = frame.map(|f| f.version.as_str()).unwrap_or("");
    let version = Version::parse(module_version.trim_start_matches('v'))
        .unwrap_or_else(|_| Version::new(0, 0, 0));

    // Go module paths contain multiple slashes ("golang.org/x/text")
    // that PackageName (npm-scoped, 1 slash) cannot hold. Keep the
    // FULL path in the title + use the LAST NON-EMPTY path segment as
    // the display name — the finding survives with all its
    // information, an empty/missing path degrades to an honest
    // "unknown-module" carrier.
    // Đường dẫn module Go chứa nhiều slash ("golang.org/x/text") mà
    // PackageName (npm-scoped, 1 slash) không giữ được. Giữ path
    // ĐẦY ĐỦ trong title + dùng đoạn CUỐI KHÁC RỖNG làm tên hiển
    // thị — finding sống sót với trọn thông tin; path rỗng/thiếu thì
    // rơi về carrier "unknown-module" trung thực.
    let display_name = module_path
        .rsplit('/')
        .find(|s| !s.is_empty())
        .unwrap_or("unknown-module");
    let Ok(pkg_name) = PackageName::new(display_name.to_string()) else {
        return Err(super::super::error::GovulncheckError::InvalidPackageName(
            module_path.to_string(),
        )
        .into());
    };

    let summary = if osv.summary.is_empty() {
        osv.details.clone()
    } else {
        osv.summary.clone()
    };
    // Real module version rides the title when it is not plain semver.
    // Version module thật ghi trong title khi nó không phải semver thuần.
    let title_suffix = if module_path.is_empty() {
        String::new()
    } else if module_version.is_empty() {
        format!(" (module {module_path})")
    } else {
        format!(" (module {module_path}@{module_version})")
    };
    let (severity_str, severity_level) = super::super::severity::govulncheck_severity(osv);

    Ok(Vulnerability {
        package: PackageId::new(pkg_name, version),
        title: format!(
            "{}: {}{}",
            osv.id,
            truncate_chars(&summary, 200),
            title_suffix
        ),
        severity: severity_str,
        cve: osv
            .aliases
            .iter()
            .find(|a| a.starts_with("CVE-"))
            .cloned()
            .unwrap_or_else(|| osv.id.clone()),
        severity_level,
        patched_versions: finding
            .fixed_version
            .as_deref()
            .filter(|v| !v.is_empty())
            .map(str::to_string),
        url: osv
            .references
            .iter()
            .find_map(|r| r.url.clone())
            .or_else(|| Some(format!("https://pkg.go.dev/vuln/{}", osv.id))),
        scanner: None,
        ecosystem: None,
        evidence_at: None,
        finding_class: FindingClass::Vulnerability,
    }
    .with_evidence("govulncheck", "go"))
}

pub(super) fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    text.chars().take(max_chars).collect()
}
