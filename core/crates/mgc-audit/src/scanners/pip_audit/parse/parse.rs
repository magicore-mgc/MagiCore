use mgc_types::adapter::{FindingClass, Vulnerability, VulnerabilitySeverity};
use mgc_types::{MgError, MgResult};
use serde::Deserialize;

use super::super::types::PipAuditParse;
use crate::scanners::truncate_utf8;

/// Typed pip-audit dependency entry — official JSON format is a TOP-LEVEL
/// ARRAY of dependencies, each carrying its own `vulns` list.
/// Entry dependency của pip-audit — JSON chính thức là MẢNG TOP-LEVEL các
/// dependency, mỗi dependency có list `vulns` riêng.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PipAuditDependency {
    name: String,
    version: String,
    #[serde(default)]
    vulns: Vec<PipAuditVuln>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PipAuditVuln {
    id: String,
    #[serde(default)]
    fix_versions: Vec<String>,
    description: Option<String>,
    aliases: Option<Vec<String>>,
}

/// Parse pip-audit JSON output (official schema: top-level array of
/// dependencies with nested `vulns`, fail-closed).
/// Parse JSON pip-audit (schema chính thức: mảng top-level dependency với
/// `vulns` lồng nhau, fail-closed).
///
/// Format: https://pypi.org/project/pip-audit/ (`--format json`)
pub fn parse_pip_audit_json(json: &str) -> MgResult<PipAuditParse> {
    let json_start = json
        .find('[')
        .unwrap_or_else(|| json.find('{').unwrap_or(0));
    // Stream-parse the FIRST complete JSON value from the payload:
    // pip-audit 2.10.1 on some CI runners emits trailing noise after
    // the JSON body (progress/status lines merged into stdout) —
    // serde_json's streaming deserializer consumes exactly one value
    // and ignores anything after, while still failing on a genuinely
    // broken body (fail-closed on corruption, tolerant of suffix junk).
    // Parse-stream GIÁ TRỊ JSON đầu tiên trong payload: pip-audit
    // 2.10.1 trên một số runner CI in thêm rác sau thân JSON (dòng
    // progress/status lẫn vào stdout) — deserializer streaming của
    // serde_json tiêu thụ đúng một giá trị và bỏ qua phần sau, nhưng
    // vẫn fail trên thân JSON thật sự hỏng (fail-closed với hỏng,
    // khoan dung với rác hậu tố).
    let mut stream = serde_json::Deserializer::from_str(json[json_start..].trim())
        .into_iter::<Vec<PipAuditDependency>>();
    let deps: Vec<PipAuditDependency> = stream
        .next()
        .ok_or_else(|| MgError::Other("invalid pip-audit JSON: empty payload".to_string()))?
        .map_err(|e| MgError::Other(format!("invalid pip-audit JSON: {e}")))?;

    let mut vulns = Vec::new();
    let mut rejected = Vec::new();
    for dep in &deps {
        // Skip editable/vcs entries pip-audit reports as `name: null`-style
        // shells — count them as audited, they carry no advisories.
        let Ok(pkg_name) = mgc_types::PackageName::new(&dep.name) else {
            rejected.push(format!("dependency '{}'", dep.name));
            continue;
        };
        let Ok(version) = mgc_types::Version::parse(&dep.version) else {
            rejected.push(format!(
                "dependency '{}' version '{}'",
                dep.name, dep.version
            ));
            continue;
        };
        for vuln in &dep.vulns {
            let description = vuln.description.as_deref().unwrap_or("");
            let title = format!("{}: {}", vuln.id, truncate_utf8(description, 200));
            // pip-audit carries no severity field — record as Info with the
            // advisory id; severity guessing would be dishonest.
            // pip-audit không có trường severity — ghi Info kèm advisory id;
            // đoán bừa severity là không trung thực.
            vulns.push(
                Vulnerability {
                    package: mgc_types::PackageId::new(pkg_name.clone(), version.clone()),
                    title,
                    severity: "info".to_string(),
                    cve: vuln
                        .aliases
                        .as_ref()
                        .and_then(|a| a.iter().find(|x| x.starts_with("CVE-")).cloned())
                        .unwrap_or_else(|| vuln.id.clone()),
                    severity_level: VulnerabilitySeverity::Info,
                    patched_versions: (!vuln.fix_versions.is_empty())
                        .then(|| vuln.fix_versions.join(", ")),
                    url: None,
                    scanner: None,
                    ecosystem: None,
                    evidence_at: None,
                    finding_class: FindingClass::Vulnerability,
                }
                .with_evidence("pip-audit", "python"),
            );
        }
    }

    // Any unparseable dependency is a schema problem, not clean data —
    // surface it instead of dropping findings silently.
    // Dependency không parse được là lỗi schema, không phải dữ liệu sạch —
    // nêu rõ thay vì bỏ finding âm thầm.
    if !rejected.is_empty() {
        return Err(MgError::Other(format!(
            "pip-audit JSON contained {} unparseable entries: {}",
            rejected.len(),
            rejected.join("; ")
        )));
    }

    Ok(PipAuditParse {
        vulnerabilities: vulns,
        packages_audited: deps.len(),
    })
}
