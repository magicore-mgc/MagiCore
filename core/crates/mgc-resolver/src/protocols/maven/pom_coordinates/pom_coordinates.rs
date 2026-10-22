use super::super::pom_xml::{element_blocks, normalize_pom_xml, remove_element_blocks};
use mgc_types::MgResult;

/// Coordinates of the `<parent>` block, if all three are literal.
/// (Tọa độ khối `<parent>`, nếu cả ba đều literal.)
pub fn parse_parent_coords(pom: &str) -> MgResult<Option<(String, String, String)>> {
    let normalized = normalize_pom_xml(pom)?;
    let pom = normalized.as_str();
    let Some(block) = element_blocks(pom, "parent").first().map(|s| s.to_string()) else {
        return Ok(None);
    };
    let field = |tag: &str| {
        element_blocks(&block, tag)
            .first()
            .map(|s| s.trim().to_string())
    };
    let (Some(g), Some(a), Some(v)) = (field("groupId"), field("artifactId"), field("version"))
    else {
        return Ok(None);
    };
    if g.contains("${") || a.contains("${") || v.contains("${") {
        return Ok(None);
    }
    Ok(Some((g, a, v)))
}

/// Substitute `${key}` from props (iterated: values may nest one level).
/// Returns None when any `${…}` remains unresolvable.
/// (Thế `${key}` từ props. Trả None khi còn `${…}` không resolve được.)
pub fn substitute_properties(
    raw: &str,
    props: &std::collections::HashMap<String, String>,
) -> Option<String> {
    let mut current = raw.to_string();
    for _ in 0..5 {
        if !current.contains("${") {
            return Some(current);
        }
        let mut next = current.clone();
        let mut keys: Vec<&String> = props.keys().collect();
        keys.sort_by_key(|k| std::cmp::Reverse(k.len()));
        for key in keys {
            let placeholder = format!("${{{key}}}");
            if next.contains(&placeholder) {
                next = next.replace(&placeholder, &props[key]);
            }
        }
        if next == current {
            return None;
        }
        current = next;
    }
    if current.contains("${") {
        None
    } else {
        Some(current)
    }
}
/// Project coordinates `(groupId, artifactId)` of a pom.xml: dependency
/// blocks are stripped first so nested artifactIds can never shadow the
/// project's own coordinates.
/// Tọa độ project `(groupId, artifactId)` của pom.xml: khối dependency bị
/// cắt trước để artifactId lồng nhau không bao giờ che tọa độ của project.
pub fn pom_project_coordinates(pom: &str) -> MgResult<Option<(String, String)>> {
    let normalized = normalize_pom_xml(pom)?;
    let pom = normalized.as_str();
    let stripped = remove_element_blocks(pom, "dependencyManagement");
    let stripped = remove_element_blocks(&stripped, "dependencies");
    let group = element_blocks(&stripped, "groupId")
        .first()
        .map(|s| s.trim().to_string());
    let artifact = element_blocks(&stripped, "artifactId")
        .first()
        .map(|s| s.trim().to_string());
    let (Some(group), Some(artifact)) = (group, artifact) else {
        return Ok(None);
    };
    Ok(Some((group, artifact)))
}
