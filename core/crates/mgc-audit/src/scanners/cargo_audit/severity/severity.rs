use mgc_types::adapter::VulnerabilitySeverity;

/// Extract base severity from a CVSS v3 vector string — implement the CVSS
/// v3.1 base-score formula (ISS → ISC → Exploitability → roundup bands)
/// exactly as the official calculator does. No external crate: keeps the
/// lockfile stable and the mapping auditable in one place.
/// Trích severity từ chuỗi CVSS v3 — thực thi công thức base-score CVSS
/// v3.1 (ISS → ISC → Exploitability → roundup) đúng như calculator chính
/// thức. Không dùng crate ngoài: lockfile ổn định, mapping kiểm tra được.
pub(in crate::scanners) fn cvss_base_severity(vector: &str) -> Option<VulnerabilitySeverity> {
    // "CVSS:3.1/AV:N/..." → drop "CVSS:" prefix, then the version segment
    // up to the FIRST '/' (the version "3.1" contains no ':' — splitting on
    // ':' would swallow the AV metric).
    // "CVSS:3.1/AV:N/..." → bỏ prefix "CVSS:", rồi đoạn version tới '/'
    // ĐẦU TIÊN (version "3.1" không chứa ':' — tách theo ':' sẽ nuốt
    // metric AV).
    let metrics: std::collections::HashMap<&str, &str> = vector
        .trim()
        .strip_prefix("CVSS:")
        .and_then(|rest| rest.split_once('/'))
        .map(|(_, body)| body.split('/').filter_map(|p| p.split_once(':')).collect())?;

    let metric = |key: &str| metrics.get(key).copied();
    let weight = |m: Option<&str>, table: &[(&str, f64)]| {
        m.and_then(|m| table.iter().find(|(k, _)| *k == m).map(|(_, w)| *w))
    };

    // CVSS v3.1 metric weights.
    let av = weight(
        metric("AV"),
        &[("N", 0.85), ("A", 0.62), ("L", 0.55), ("P", 0.2)],
    )?;
    let ac = weight(metric("AC"), &[("L", 0.77), ("H", 0.44)])?;
    let ui = weight(metric("UI"), &[("N", 0.85), ("R", 0.62)])?;
    let scope_changed = metric("S") == Some("C");
    let pr = match (metric("PR"), scope_changed) {
        (Some("N"), _) => 0.85,
        (Some("L"), false) => 0.62,
        (Some("L"), true) => 0.68,
        (Some("H"), false) => 0.27,
        (Some("H"), true) => 0.5,
        _ => return None,
    };
    let c = weight(metric("C"), &[("H", 0.56), ("L", 0.22), ("N", 0.0)])?;
    let i = weight(metric("I"), &[("H", 0.56), ("L", 0.22), ("N", 0.0)])?;
    let a = weight(metric("A"), &[("H", 0.56), ("L", 0.22), ("N", 0.0)])?;

    // Impact Sub-Score, Impact, Exploitability per CVSS v3.1 spec §7.1.
    let iss = 1.0 - (1.0 - c) * (1.0 - i) * (1.0 - a);
    let (isc, multiplier) = if scope_changed {
        (7.52 * (iss - 0.029) - 3.25 * (iss - 0.02).powi(15), 1.08)
    } else {
        (6.42 * iss, 1.0)
    };
    let exploitability = 8.22 * av * ac * pr * ui;
    if isc <= 0.0 {
        return Some(VulnerabilitySeverity::Info);
    }
    let score = (multiplier * (isc + exploitability)).min(10.0);
    Some(cvss_rating((score * 10.0).round() / 10.0))
}

/// CVSS v3.1 qualitative rating bands.
/// Bảng xếp loại định tính CVSS v3.1.
fn cvss_rating(score: f64) -> VulnerabilitySeverity {
    match score {
        s if s >= 9.0 => VulnerabilitySeverity::Critical,
        s if s >= 7.0 => VulnerabilitySeverity::High,
        s if s >= 4.0 => VulnerabilitySeverity::Medium,
        s if s > 0.0 => VulnerabilitySeverity::Low,
        _ => VulnerabilitySeverity::Info,
    }
}

/// Public CVSS bridge for sibling scanner modules (govulncheck) — ONE
/// CVSS implementation for every ecosystem (RULE: no copy-paste logic).
/// Cầu CVSS public cho các module scanner anh em (govulncheck) — MỘT bản
/// implement CVSS cho mọi ecosystem (RULE: không copy-paste logic).
pub(crate) fn cvss_base_severity_for_go(vector: &str) -> Option<VulnerabilitySeverity> {
    cvss_base_severity(vector)
}
