//! Parse the official govulncheck streaming JSON without spawning the binary.
//! (Parse stream JSON chính thức mà không khởi chạy binary govulncheck.)
//! A finding without a matching OSV record is rejected; both pretty and
//! one-message-per-line encodings remain accepted.
//! Finding thiếu OSV record bị từ chối; vẫn nhận JSON pretty và mỗi-message-mỗi-dòng.

use super::super::govulncheck::GovulncheckParse;
use super::super::model::{GovulncheckFinding, GovulncheckMessage, GovulncheckOsv};
use mgc_types::MgResult;
use mgc_types::adapter::Vulnerability;

pub fn parse_govulncheck_json(raw: &str) -> MgResult<GovulncheckParse> {
    let mut osv_entries: std::collections::HashMap<String, GovulncheckOsv> =
        std::collections::HashMap::new();
    let mut findings: Vec<GovulncheckFinding> = Vec::new();
    let mut packages_audited = 0usize;

    let stream = serde_json::Deserializer::from_str(raw).into_iter::<GovulncheckMessage>();
    for message in stream {
        let message = message.map_err(|error| {
            super::super::error::GovulncheckError::InvalidJson(error.to_string())
        })?;

        if let Some(sbom) = message.sbom {
            packages_audited = sbom.modules.len();
        }
        if let Some(osv) = message.osv {
            osv_entries.insert(osv.id.clone(), osv);
        }
        if let Some(finding) = message.finding {
            findings.push(finding);
        }
    }

    // Every finding MUST reference a known OSV id — a dangling reference
    // means the stream was truncated or malformed: fail closed.
    // Mọi finding PHẢI trỏ tới id OSV đã thấy — reference lơ lửng nghĩa
    // là stream bị cắt/malformed: fail-closed.
    let mut vulnerabilities: Vec<Vulnerability> = Vec::new();
    let mut seen_osv_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
    for finding in &findings {
        if finding.osv.is_empty() {
            return Err(super::super::error::GovulncheckError::FindingWithoutOsvId.into());
        }
        // Dedupe: one OSV id → one row (govulncheck emits module-, package-
        // AND symbol-level findings for the same vulnerability).
        // Khử trùng: một id OSV → một dòng (govulncheck phát finding ở cả
        // level module/package/symbol cho cùng một lỗ hổng).
        if !seen_osv_ids.insert(finding.osv.clone()) {
            continue;
        }
        let osv = osv_entries.get(&finding.osv).ok_or_else(|| {
            super::super::error::GovulncheckError::UnknownOsvId(finding.osv.clone())
        })?;

        vulnerabilities.push(super::super::finding::vulnerability_from_finding(
            finding, osv,
        )?);
    }

    Ok(GovulncheckParse {
        vulnerabilities,
        packages_audited,
    })
}
