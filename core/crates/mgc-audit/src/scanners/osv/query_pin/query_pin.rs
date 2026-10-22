//! one pinned package request and vulnerability detail materialization.
//! Tạo kết quả lỗ hổng từ một package đã ghim.

use super::super::{OsvPin, OsvQueryResponse, OsvVuln, osv_severity};
use mgc_types::adapter::{FindingClass, Vulnerability};
use mgc_types::{MgError, MgResult, PackageId, PackageName, Version};

/// OSV.dev API base (public instance; overridable via MGC_OSV_API_BASE
/// for offline/failure-lane testing — the 10-test/lane tool-failure
/// evidence needs a deterministic dead endpoint, not a flaky network).
/// Cơ sở API OSV.dev (bản công cộng; ghi đè qua MGC_OSV_API_BASE cho
/// test offline/failure-lane — evidence tool-failure cần endpoint chết
/// tất định, không phải mạng chập chờn).
fn osv_api_base() -> String {
    std::env::var("MGC_OSV_API_BASE").unwrap_or_else(|_| "https://api.osv.dev/v1".to_string())
}

/// Query OSV for ONE pin: POST /v1/query → ids, then GET /v1/vulns/{id}
/// per id for the full record. Any non-2xx or unshapeable payload is a
/// hard error (fail-closed).
/// Truy vấn OSV cho MỘT ghim: POST /v1/query → ids, rồi GET
/// /v1/vulns/{id} từng id lấy bản ghi đầy đủ. Non-2xx hoặc payload
/// không đúng shape là lỗi cứng (fail-closed).
pub(crate) async fn query_pin(
    client: &mgc_http::HttpClient,
    pin: &OsvPin,
) -> MgResult<Vec<Vulnerability>> {
    let body = serde_json::json!({
        "package": {"ecosystem": pin.ecosystem, "name": pin.name},
        "version": pin.version,
    });
    // The shared HttpClient owns transport (RULE §8 no direct reqwest):
    // serialize the JSON body to bytes; the client is preconfigured
    // with the JSON content type by the caller.
    // HttpClient dùng chung giữ transport (RULE §8 không reqwest trực
    // tiếp): serialize body JSON ra bytes; client đã được caller cấu
    // hình sẵn content-type JSON.
    let body_bytes = serde_json::to_vec(&body).map_err(|e| {
        MgError::Other(format!(
            "osv query body for '{}' unserializable: {e}",
            pin.name
        ))
    })?;
    let response = client
        .post(format!("{}/query", osv_api_base()).as_str(), body_bytes)
        .await
        .map_err(|e| MgError::Network(format!("osv query '{}' failed: {e}", pin.name)))?;
    if !response.status().is_success() {
        return Err(MgError::Network(format!(
            "osv query '{}' returned {}",
            pin.name,
            response.status()
        )));
    }
    let ids: OsvQueryResponse = response.json().await.map_err(|e| {
        MgError::Other(format!(
            "osv query response for '{}' unparseable: {e}",
            pin.name
        ))
    })?;
    if ids.vulns.is_empty() {
        return Ok(vec![]);
    }

    let mut findings = Vec::new();
    for id in &ids.vulns {
        let detail = client
            .get(format!("{}/vulns/{}", osv_api_base(), id.id).as_str())
            .await
            .map_err(|e| MgError::Network(format!("osv vuln '{}' fetch failed: {e}", id.id)))?;
        if !detail.status().is_success() {
            return Err(MgError::Network(format!(
                "osv vuln '{}' returned {}",
                id.id,
                detail.status()
            )));
        }
        let vuln: OsvVuln = detail
            .json()
            .await
            .map_err(|e| MgError::Other(format!("osv vuln '{}' unparseable: {e}", id.id)))?;

        let (severity, level) = osv_severity(&vuln);
        let cve = vuln
            .aliases
            .iter()
            .find(|a| a.starts_with("CVE-"))
            .cloned()
            .unwrap_or_else(|| vuln.id.clone());
        let summary = if vuln.summary.is_empty() {
            vuln.details.clone()
        } else {
            vuln.summary.clone()
        };
        let Ok(name) = PackageName::new(pin.name.clone()) else {
            return Err(MgError::Other(format!(
                "osv pin name '{}' cannot be represented as a package name",
                pin.name
            )));
        };
        let Ok(version) = Version::parse(&pin.version) else {
            return Err(MgError::Other(format!(
                "osv pin version '{}' unparseable as semver",
                pin.version
            )));
        };
        findings.push(
            Vulnerability {
                package: PackageId::new(name, version),
                title: format!("{}: {}", vuln.id, truncate(&summary, 200)),
                severity,
                cve,
                severity_level: level,
                patched_versions: None,
                url: vuln
                    .references
                    .iter()
                    .find_map(|r| r.url.clone())
                    .or_else(|| Some(format!("{}/vulns/{}", osv_api_base(), vuln.id))),
                scanner: None,
                ecosystem: None,
                evidence_at: None,
                finding_class: FindingClass::Vulnerability,
            }
            .with_evidence("osv-dev-api", pin.ecosystem),
        );
    }
    Ok(findings)
}

fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        text.chars().take(max_chars).collect()
    }
}
