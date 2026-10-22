//! Render audit reports as CycloneDX Vulnerable BOM documents.
//! Render audit report thành tài liệu CycloneDX Vulnerable BOM.

use super::purl::{component_bom_ref, purl_name, purl_type_for};
use super::serial::stable_serial_hash;
use super::severity::cyclonedx_severity;
use super::types::*;
use mgc_types::adapter::{AuditReport, ScannerStatus};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

/// Build a CycloneDX Vulnerable BOM from an audit report. Scan results
/// are emitted for Available AND Partial reports (P0/F3: incomplete
/// coverage must not delete rows — the Go OSV fallback and java/dotnet
/// skipped lanes report Partial WITH real findings). ToolMissing,
/// UnsupportedEcosystem, and Failed never ran a scan, so their rows (if
/// any were ever attached) are not evidence and stay suppressed. The
/// UNVERIFIED tool property still marks every non-Available render.
/// Dựng BOM từ report. Kết quả scan được emit cho Available VÀ Partial;
/// các trạng thái chưa từng chạy scan thì không emit.
pub fn to_cyclonedx(report: &AuditReport) -> CycloneDxBom {
    let verified = matches!(report.scanner_status, ScannerStatus::Available);
    let ran_scan = verified || matches!(report.scanner_status, ScannerStatus::Partial { .. });
    let emit = ran_scan && !report.vulnerabilities.is_empty();

    // Deterministic serial from the report CONTENTS — the same report
    // renders the same serial (reproducible output for CI diffing). It
    // identifies this render, not a security boundary.
    // Serial all định từ NỘI DUNG report — cùng report cho cùng serial
    // (output tái lập được để CI diff). Nó định danh bản render, không
    // phải ranh giới bảo mật.
    let mut serial_input = format!(
        "mgc-audit|{}|{}|{}",
        report.packages_audited, report.vulnerability_count, verified
    );
    for v in &report.vulnerabilities {
        serial_input.push_str(&format!("|{}:{}", v.cve, v.package.name().as_str()));
    }
    let serial_hash = stable_serial_hash(&serial_input);

    let mut components: Vec<CycloneDxComponent> = Vec::new();
    let mut vulnerabilities: Vec<CycloneDxVulnerability> = Vec::new();

    if emit {
        for v in &report.vulnerabilities {
            let bom_ref = component_bom_ref(v);
            let purl_type = purl_type_for(v.ecosystem.as_deref());
            let name = v.package.name().as_str();
            let version = v.package.version().to_string();
            let purl_name = purl_name(v);

            // One component per affected package — dedupe on bom-ref.
            // Một component cho mỗi package dính — khử trùng theo bom-ref.
            if !components.iter().any(|c| c.bom_ref == bom_ref) {
                components.push(CycloneDxComponent {
                    component_type: "library",
                    bom_ref: bom_ref.clone(),
                    name: name.to_string(),
                    version: version.clone(),
                    purl: format!("pkg:{purl_type}/{purl_name}@{version}"),
                });
            }

            let scanner_name = v.scanner.as_deref().unwrap_or("unknown");
            vulnerabilities.push(CycloneDxVulnerability {
                bom_ref: format!("vuln:{}:{}", v.cve, bom_ref),
                id: v.cve.clone(),
                source: CycloneDxSource {
                    name: scanner_name.to_string(),
                    url: v.url.clone(),
                },
                ratings: vec![CycloneDxRating {
                    severity: cyclonedx_severity(&v.severity_level).to_string(),
                }],
                description: v.title.clone(),
                affects: vec![CycloneDxAffect { r#ref: bom_ref }],
                properties: vec![
                    CycloneDxProperty {
                        name: "magicore:scanner".to_string(),
                        value: scanner_name.to_string(),
                    },
                    CycloneDxProperty {
                        name: "magicore:evidence_at".to_string(),
                        value: v.evidence_at.clone().unwrap_or_default(),
                    },
                    CycloneDxProperty {
                        name: "magicore:finding_class".to_string(),
                        value: v.finding_class.as_str().to_string(),
                    },
                ],
            });
        }
    }

    let tool_properties = (!verified).then(|| {
        vec![CycloneDxProperty {
            name: "magicore:status".to_string(),
            value: "UNVERIFIED — scanner did not complete".to_string(),
        }]
    });

    CycloneDxBom {
        bom_format: CYCLONEDX_BOM_FORMAT.to_string(),
        spec_version: CYCLONEDX_SPEC_VERSION.to_string(),
        serial_number: format!("urn:uuid:{serial_hash}"),
        version: 1,
        metadata: CycloneDxMetadata {
            timestamp: mgc_types::adapter::now_rfc3339_public(),
            tools: CycloneDxTools {
                components: vec![CycloneDxTool {
                    tool_type: "application",
                    vendor: "MagiCore",
                    name: "mgc-audit",
                    version: env!("CARGO_PKG_VERSION").to_string(),
                    properties: tool_properties.unwrap_or_default(),
                }],
            },
        },
        components,
        vulnerabilities,
    }
}
