// Fail-closed bulk advisory response parsing. / Parse response bulk advisory fail-closed.
use mgc_types::{MgError, MgResult, Vulnerability};

use super::super::{build_findings_for_advisory, type_of_json};

/// Parse the bulk advisory response with a fail-closed contract: the
/// payload MUST be an object mapping package names to advisory arrays;
/// every advisory MUST carry the npm Bulk API required fields. Anything
/// else (array, string, null, wrong shapes) is an error — never an
/// implicit zero-finding "clean" report.
/// Parse response bulk advisory theo hợp đồng fail-closed: payload PHẢI
/// là object map tên package → mảng advisory; mọi advisory PHẢI đủ field
/// bắt buộc của npm Bulk API. Cái khác (array, string, null, shape sai)
/// là lỗi — không bao giờ tự biến thành report "sạch" 0 finding.
///
/// Findings are built CLIENT-SIDE: for every advisory, every lockfile
/// entry whose installed version matches `vulnerable_versions` becomes
/// one Vulnerability row (same model as pnpm audit — the registry has
/// no knowledge of our tree).
/// Finding được dựng Ở CLIENT: với mỗi advisory, mọi entry lockfile có
/// version nằm trong `vulnerable_versions` thành một dòng Vulnerability
/// (registry không biết tree của ta — pnpm cũng làm vậy).
pub fn parse_advisory_bulk_response(
    advisories: &serde_json::Value,
    pins: &[(String, String)],
) -> MgResult<Vec<Vulnerability>> {
    let Some(map) = advisories.as_object() else {
        return Err(MgError::Other(format!(
            "advisory response must be a JSON object mapping packages to advisory arrays, got: {}",
            type_of_json(advisories)
        )));
    };

    // Requested package names — the response may only speak about packages
    // we actually asked for; anything else is a contract violation.
    // Tên package đã yêu cầu — response chỉ được nói về package ta hỏi;
    // ngoài tập này là vi phạm hợp đồng.
    let requested: std::collections::HashSet<&str> = pins.iter().map(|(n, _)| n.as_str()).collect();

    let mut vulnerabilities = Vec::new();
    let mut rejected: Vec<String> = Vec::new();

    for (pkg_name, advisory_list) in map {
        if !requested.contains(pkg_name.as_str()) {
            return Err(MgError::Other(format!(
                "advisory response mentions package '{pkg_name}' which was not requested — \
                possible malformed or hostile response"
            )));
        }
        let Some(advisories_arr) = advisory_list.as_array() else {
            return Err(MgError::Other(format!(
                "advisory response for '{pkg_name}' must be an array, got: {}",
                type_of_json(advisory_list)
            )));
        };
        for advisory_value in advisories_arr {
            match build_findings_for_advisory(pkg_name, advisory_value, pins) {
                Ok(mut found) => vulnerabilities.append(&mut found),
                Err(reason) => rejected.push(format!("{pkg_name}: {reason}")),
            }
        }
    }

    // No advisory entry may be dropped silently — a malformed entry is a
    // schema error, not a "skip gracefully" moment.
    // Không được bỏ âm thầm entry advisory nào — entry malformed là lỗi
    // schema, không phải lúc "skip gracefully".
    if !rejected.is_empty() {
        return Err(MgError::Other(format!(
            "advisory response contained {} malformed entr{}: {}",
            rejected.len(),
            if rejected.len() == 1 { "y" } else { "ies" },
            rejected.join("; ")
        )));
    }

    Ok(vulnerabilities)
}
