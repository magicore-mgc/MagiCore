use super::super::pom_xml::{element_blocks, normalize_pom_xml, remove_element_blocks};
use mgc_types::MgResult;

/// Properties of one POM: its `<properties>` entries plus the
/// `project.*` built-ins (groupId/artifactId/version of the POM itself).
/// Later merges never overwrite keys already present (child wins).
/// (Properties của một POM: entry `<properties>` cộng built-in
/// `project.*`. Merge sau không bao giờ ghi đè key đã có (con thắng).)
pub fn collect_pom_properties(pom: &str) -> MgResult<std::collections::HashMap<String, String>> {
    let normalized = normalize_pom_xml(pom)?;
    let pom = normalized.as_str();
    let mut props = std::collections::HashMap::new();
    for block in element_blocks(pom, "properties").iter().take(1) {
        for (key, value) in pom_direct_entries(block) {
            props.entry(key).or_insert(value);
        }
    }
    let stripped = remove_element_blocks(pom, "dependencyManagement");
    let stripped = remove_element_blocks(&stripped, "dependencies");
    let field = |tag: &str| {
        element_blocks(&stripped, tag)
            .first()
            .map(|s| s.trim().to_string())
    };
    if let (Some(group), Some(artifact), Some(version)) =
        (field("groupId"), field("artifactId"), field("version"))
    {
        props.entry("project.groupId".to_string()).or_insert(group);
        props
            .entry("project.artifactId".to_string())
            .or_insert(artifact);
        props
            .entry("project.version".to_string())
            .or_insert(version);
    }
    Ok(props)
}

/// Direct `key → text` children of a block (nested blocks ignored).
/// (Các con trực tiếp `key → text` của một khối.)
fn pom_direct_entries(block: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = block;
    while let Some(start) = rest.find('<') {
        if rest[start..].starts_with("</") {
            break;
        }
        let after = &rest[start + 1..];
        let end = after.find(['>', ' ', '/']).unwrap_or(after.len());
        let tag = &after[..end];
        if tag.is_empty() || tag.starts_with('?') || tag.starts_with('!') {
            rest = &after[end.min(after.len())..];
            continue;
        }
        let inner = element_blocks(rest, tag).first().map(|s| s.to_string());
        let Some(text) = inner else {
            break;
        };
        // Advance past this whole element for the next sibling.
        // (Tiến qua cả phần tử này để tới anh em kế.)
        if let Some(close) = rest.find(&format!("</{tag}>")) {
            rest = &rest[close + tag.len() + 3..];
        } else if let Some(gt) = rest[start..].find('>') {
            rest = &rest[start + gt + 1..];
        } else {
            break;
        }
        if text.contains('<') {
            continue;
        }
        out.push((tag.to_string(), text.trim().to_string()));
    }
    out
}

/// `dependencyManagement` versions with literal (non-`${}`) versions:
/// `(group, artifact) → version`.
/// (Version `dependencyManagement` dạng literal.)
pub fn collect_managed_versions(
    pom: &str,
) -> MgResult<std::collections::HashMap<(String, String), String>> {
    let normalized = normalize_pom_xml(pom)?;
    let pom = normalized.as_str();
    let mut managed = std::collections::HashMap::new();
    for management in element_blocks(pom, "dependencyManagement") {
        for block in element_blocks(management, "dependencies") {
            for dep_xml in element_blocks(block, "dependency") {
                let field = |tag: &str| {
                    element_blocks(dep_xml, tag)
                        .first()
                        .map(|s| s.trim().to_string())
                };
                if let (Some(g), Some(a), Some(v)) =
                    (field("groupId"), field("artifactId"), field("version"))
                    && !v.contains("${")
                {
                    managed.entry((g, a)).or_insert(v);
                }
            }
        }
    }
    Ok(managed)
}
