//! Root-pin splitting and V4 lane-base filtering for lane merges.
//! Tách root pin và lọc lane base V4 cho merge theo lane.

use std::collections::HashSet;

/// split on the LAST `@`.
/// (Tách pin root thành `(name, version)`. Tên scope tách ở `@` cuối.)
pub(crate) fn split_root_name_version(pin: &str) -> Option<(String, String)> {
    let id = if let Some(encoded) = pin.strip_prefix("mgc-root-v1:") {
        encoded.split_once(':').map(|(_, id)| id).unwrap_or(encoded)
    } else {
        pin
    };
    let (name, version) = id.rsplit_once('@')?;
    if name.is_empty() || version.is_empty() {
        return None;
    }
    Some((name.to_string(), version.to_string()))
}
/// Keep rows that are NOT this lane's own (caller-filtered by identity);
/// `None` when the base document is unusable as a merge base.
/// (Giữ row KHÔNG phải của lane này; `None` khi base không dùng merge được.)
pub(crate) fn base_has_v4(
    doc: &crate::canonical::LockfileV4,
    lane_ids: &HashSet<(String, String, String)>,
) -> Option<Vec<crate::canonical::PackageV4>> {
    if !crate::v4::is_v4_schema_version(&doc.version) {
        return None;
    }
    Some(
        doc.packages
            .iter()
            .filter(|p| {
                !lane_ids.contains(&(
                    p.key.name.clone(),
                    p.key.version.clone(),
                    p.key.source_id.clone(),
                ))
            })
            .cloned()
            .collect(),
    )
}
