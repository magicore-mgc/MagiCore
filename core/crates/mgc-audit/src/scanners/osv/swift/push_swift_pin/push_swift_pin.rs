//! version/location validation and honest skip classification for one Swift pin.
//! Kiểm tra version/location và phân loại skip cho một pin Swift.

use super::super::{SwiftPinState, osv_swifturl_name};
use crate::scanners::osv::OsvPin;

pub(in crate::scanners::osv) fn push_swift_pin(
    identity: &str,
    location: Option<&str>,
    state: Option<&SwiftPinState>,
    pins: &mut Vec<OsvPin>,
    skipped: &mut Vec<String>,
) {
    let Some(state) = state else {
        skipped.push(format!("{identity}: no state recorded"));
        return;
    };
    match state.version.as_deref() {
        Some(version) if !version.is_empty() => {
            // OSV's Swift ecosystem is `SwiftURL` (verified live:
            // {"ecosystem":"swift"} → 400 invalid ecosystem). CRITICAL
            // NAMING FIX (2026-09-10): the OSV package NAME is the repo
            // path WITHOUT scheme and .git suffix — `github.com/owner/
            // repo` (verified live: swift-nio-http2@1.40.0 under
            // `github.com/apple/swift-nio-http2` returns GHSA-q3g2 +
            // GHSA-4px2, while the full git URL form `https://github.
            // com/apple/swift-nio-http2.git` returns EMPTY — a silent
            // fake-clean). A location is still required; unparsable
            // locations are skipped honestly, never queried wrong.
            // Ecosystem Swift của OSV là `SwiftURL` (verify sống:
            // "swift" → 400). SỬA TÊN QUAN TRỌNG: tên package OSV là
            // đường dẫn repo KHÔNG scheme KHÔNG đuôi .git —
            // `github.com/owner/repo` (verify sống: tên đúng trả 2
            // GHSA, URL git đầy đủ trả RỖNG — sạch giả âm thầm).
            // Vẫn cần location; location không tách được thì skip
            // trung thực, không query sai.
            let Some(location) = location.filter(|l| !l.is_empty()) else {
                skipped.push(format!(
                    "{identity}: pin carries no git location — OSV SwiftURL queries need the repo URL"
                ));
                return;
            };
            let Some(osv_name) = osv_swifturl_name(location) else {
                skipped.push(format!(
                    "{identity}: location '{location}' is not a github.com/owner/repo URL — cannot build the OSV SwiftURL name"
                ));
                return;
            };
            pins.push(OsvPin {
                name: osv_name,
                version: version.to_string(),
                ecosystem: "SwiftURL",
            });
        }
        _ => skipped.push(format!(
            "{identity}: versionless checkout (branch={}, revision={})",
            state.branch.as_deref().unwrap_or("?"),
            state.revision.as_deref().unwrap_or("?"),
        )),
    }
}
