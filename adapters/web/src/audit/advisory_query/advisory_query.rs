//! Execute the npm bulk advisory request and parse its response.
//! Gọi request npm bulk advisory và parse response.

use mgc_types::Vulnerability;
use mgc_types::adapter::AuditReport;

use super::{AdvisoryQueryError, AdvisoryQueryOutcome};

/// Query npm advisories for the already-filtered npm lock pins.
/// Tra advisory npm cho các pin npm đã được lọc từ lock.
pub(in crate::audit) async fn query_npm_advisories(
    registry_url: &str,
    pins: &[(String, String)],
) -> Result<AdvisoryQueryOutcome, AdvisoryQueryError> {
    let body = mgc_js_service::audit::advisory_bulk_request_body(pins);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent(format!("magicore/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(AdvisoryQueryError::ClientBuild)?;

    let advisory_endpoint = mgc_js_service::audit::registry_advisory_bulk_endpoint(registry_url)
        .map_err(AdvisoryQueryError::Endpoint)?;
    let response = match client.post(advisory_endpoint).json(&body).send().await {
        Ok(response) => response,
        Err(error) => {
            return Ok(AdvisoryQueryOutcome::ScannerFailed(
                AuditReport::scanner_failed(
                    "npm-bulk-advisory",
                    format!("audit request failed: {error}"),
                ),
            ));
        }
    };

    if !response.status().is_success() {
        return Ok(AdvisoryQueryOutcome::ScannerFailed(
            AuditReport::scanner_failed(
                "npm-bulk-advisory",
                format!("audit API returned {}", response.status()),
            ),
        ));
    }

    let advisories: serde_json::Value = response
        .json()
        .await
        .map_err(AdvisoryQueryError::ResponseJson)?;
    let vulnerabilities: Vec<Vulnerability> =
        mgc_js_service::audit::parse_advisory_bulk_response(&advisories, pins)
            .map_err(AdvisoryQueryError::AdvisoryParse)?;
    Ok(AdvisoryQueryOutcome::Parsed(vulnerabilities))
}
