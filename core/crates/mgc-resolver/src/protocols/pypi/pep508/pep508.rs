/// Detect extras requested on a dependency name before PEP 508 parsing.
/// Dependency extras change that dependency's own transitive graph and cannot
/// be discarded without producing an incomplete resolution.
/// Phát hiện extras được yêu cầu trên tên dependency trước khi parse PEP 508.
/// Extras làm thay đổi graph bắc cầu của dependency đó nên không thể bỏ qua.
pub(in crate::protocols::pypi) fn requires_requested_extras(spec: &str) -> bool {
    let requirement = spec
        .split_once(';')
        .map_or(spec, |(requirement, _)| requirement)
        .trim();
    let name_end = requirement
        .find(|c: char| ['=', '>', '<', '!', '~'].contains(&c))
        .unwrap_or(requirement.len());
    let name = requirement[..name_end].trim();
    name.contains('[') || name.contains(']')
}

/// Parse a PEP 508 requirement into `(name, range, marker)`. Extra deps are
/// surfaced via the marker (caller skips them); `[extras]` are dropped.
/// Parse requirement PEP 508 thành `(name, range, marker)`. Dep extra lộ qua
/// marker (caller bỏ); `[extras]` bị loại.
pub(in crate::protocols::pypi) fn parse_pep508(
    spec: &str,
) -> Option<(String, String, Option<String>)> {
    let spec = spec.trim();
    if spec.is_empty() {
        return None;
    }
    let (req, marker) = match spec.split_once(';') {
        Some((r, m)) => (r.trim(), Some(m.trim().to_string())),
        None => (spec.trim(), None),
    };
    let op_idx = req
        .find(|c: char| ['=', '>', '<', '!', '~'].contains(&c))
        .unwrap_or(req.len());
    let name_part = req[..op_idx].split('[').next().unwrap_or("").trim();
    if name_part.is_empty() {
        return None;
    }
    let range = if op_idx < req.len() {
        req[op_idx..].trim().to_string()
    } else {
        "*".to_string()
    };
    Some((name_part.to_string(), range, marker))
}
