//! bounded concurrent pin requests and deterministic fail-closed reporting.
//! Truy vấn đồng thời có giới hạn và tổng hợp lỗi theo thứ tự đầu vào.

use super::{OsvPin, query_pin};
use futures_util::stream::StreamExt;
use mgc_types::adapter::{AuditReport, ScannerStatus, Vulnerability};
use mgc_types::{MgError, MgResult};

/// Max in-flight OSV queries (RULE §12: named const, not an inline
/// literal) — bounded so a 500-pin solution cannot open 500 sockets at
/// once, large enough that big projects stop paying per-pin latency.
/// (Số query OSV đồng thời tối đa.)
const OSV_QUERY_CONCURRENCY: usize = 8;

/// One pin's query outcome: index-preserving success or the pin name +
/// detail of the first failure (by pin order).
/// Kết quả query một ghim (giữ index để sắp xếp lại).
type OsvPinOutcome = Result<(usize, Vec<Vulnerability>), (usize, String, String)>;

/// Query OSV for a set of pins; a network failure of ANY pin is a
/// SCANNER FAILURE (fail-closed through the report status, never a
/// hard error and never a partial-clean answer). The CLI finisher
/// turns Failed into the honest UNVERIFIED lane (exit 2 strict).
/// Truy vấn OSV cho một tập ghim; MỘT pin lỗi mạng là SCANNER FAILED
/// (fail-closed qua status report, không phải hard error, không phải
/// trả lời sạch từng phần). Finisher CLI chuyển Failed thành lane
/// UNVERIFIED trung thực (strict exit 2).
///
/// Queries run CONCURRENTLY (bounded): rows keep pin order (sorted by
/// pin index after join), and the reported failure names the
/// lowest-index failed pin — deterministic output for the same input.
/// (Query đồng thời có giới hạn: dòng giữ đúng thứ tự ghim, lỗi báo
/// theo ghim đầu tiên — all định với cùng input.)
pub async fn audit_osv_pins(pins: &[OsvPin]) -> MgResult<AuditReport> {
    if pins.is_empty() {
        return Ok(AuditReport::clean(0));
    }
    // L1-hardcode (RULE §8): core crates never touch reqwest directly —
    // the shared mgc_http::HttpClient owns transport. Preconfigured with
    // the JSON content type for OSV query posts and NO retry: audit is a
    // fail-closed read — a dead advisory endpoint must surface the
    // error immediately (the old direct-reqwest behavior), not hang in
    // backoff loops.
    // L1-hardcode (RULE §8): crate core không chạm reqwest trực tiếp —
    // mgc_http::HttpClient dùng chung giữ transport; cấu hình sẵn
    // content-type JSON cho POST query OSV và KHÔNG retry: audit là
    // phép đọc fail-closed — advisory endpoint chết phải lộ lỗi NGAY
    // (đúng hành vi reqwest trực tiếp cũ), không treo trong vòng backoff.
    let client = mgc_http::HttpClient::new()
        .map_err(|e| MgError::Network(format!("osv client error: {e}")))?
        .with_auth("Content-Type", "application/json")
        .with_retry(mgc_http::retry::RetryStrategy::Fixed(
            std::time::Duration::ZERO,
        ));

    let mut all = Vec::new();
    // One client clone per in-flight query (HttpClient clones cheaply —
    // shared transport underneath). Items are OWNED (cloned pins), never
    // borrowed from the stream, so every future is self-contained and
    // output order stays deterministic after the index sort.
    // (Mỗi query một bản clone client; item owned để future tự chứa.)
    let owned: Vec<(usize, OsvPin)> = pins.iter().cloned().enumerate().collect();
    let mut outcomes: Vec<OsvPinOutcome> = futures_util::stream::iter(owned)
        .map(|(index, pin)| {
            let client = client.clone();
            async move {
                match query_pin(&client, &pin).await {
                    Ok(findings) => Ok((index, findings)),
                    Err(e) => Err((index, pin.name.clone(), e.to_string())),
                }
            }
        })
        .buffer_unordered(OSV_QUERY_CONCURRENCY)
        .collect()
        .await;
    outcomes.sort_by_key(|r| match r {
        Ok((index, _)) => *index,
        Err((index, _, _)) => *index,
    });
    for outcome in outcomes {
        match outcome {
            Ok((_, findings)) => all.extend(findings),
            Err((_, name, detail)) => {
                return Ok(AuditReport::scanner_failed(
                    "osv-dev-api",
                    format!("query for '{name}' failed: {detail}"),
                ));
            }
        }
    }
    Ok(AuditReport {
        packages_audited: pins.len(),
        vulnerability_count: all.len(),
        vulnerabilities: all,
        scanner_status: ScannerStatus::Available,
    })
}

// ---------------------------------------------------------------------------
// Swift Package Manager — Package.resolved (state v1 JSON / v2 JSON).
