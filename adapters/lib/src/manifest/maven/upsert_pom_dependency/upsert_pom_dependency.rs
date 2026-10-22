//! Maven dependency upsert.
//! Cập nhật dependency Maven.

use super::super::pom_dependency_is_mgc_managed::pom_dependency_is_mgc_managed;
use mgc_types::MgResult;

/// Insert or update one dependency inside the TOP-LEVEL `<dependencies>`
/// block (a direct child of `<project>` — dependencyManagement, profiles
/// and plugin blocks are never touched). Creates the block when absent.
pub(crate) fn upsert_pom_dependency(
    content: &str,
    group: &str,
    artifact: &str,
    version: &str,
) -> MgResult<String> {
    let entry = render_pom_dependency(group, artifact, version);
    // Locate the top-level <dependencies> span by tag-depth tracking
    // (dependencyManagement nests its own <dependencies> deeper).
    let mut depth = 0usize;
    let mut top_start: Option<usize> = None;
    let mut top_end: Option<usize> = None;
    let bytes = content.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if content[i..].starts_with("</") {
                let end = content[i..].find('>').map(|p| i + p).unwrap_or(bytes.len());
                let tag = content[i + 2..end].trim().to_string();
                if tag == "dependencies" && depth == 2 && top_start.is_some() && top_end.is_none() {
                    top_end = Some(end + 1);
                }
                if !tag.starts_with('?') {
                    depth = depth.saturating_sub(1);
                }
                i = end + 1;
                continue;
            }
            if content[i..].starts_with("<!--") {
                let end = content[i..]
                    .find("-->")
                    .map(|p| i + p + 3)
                    .unwrap_or(bytes.len());
                i = end;
                continue;
            }
            let end = content[i..].find('>').map(|p| i + p).unwrap_or(bytes.len());
            let mut tag = content[i + 1..end].trim().to_string();
            let self_close = tag.ends_with('/');
            if self_close {
                tag = tag.trim_end_matches('/').trim().to_string();
            }
            let name = tag.split_whitespace().next().unwrap_or("").to_string();
            if name == "dependencies" && depth == 1 && top_start.is_none() {
                top_start = Some(i);
            }
            if !self_close && !name.starts_with('?') && !name.starts_with('!') {
                depth += 1;
            }
            i = end + 1;
            continue;
        }
        i += 1;
    }
    if let (Some(s), Some(e)) = (top_start, top_end) {
        let mut block = content[s..e].to_string();
        let mut search_from = 0;
        let mut replaced = false;
        while let Some(rel) = block[search_from..].find("<dependency>") {
            let ds = search_from + rel;
            let Some(de) = block[ds..]
                .find("</dependency>")
                .map(|p| ds + p + "</dependency>".len())
            else {
                break;
            };
            let chunk = &block[ds..de];
            if chunk.contains(&format!("<groupId>{group}</groupId>"))
                && chunk.contains(&format!("<artifactId>{artifact}</artifactId>"))
            {
                if !pom_dependency_is_mgc_managed(chunk) {
                    return Err(mgc_types::MgError::Unsupported {
                        core: "lib",
                        capability: "mutate non-runtime or optional Maven dependency",
                        guidance: format!(
                            "MagiCore will not rewrite '{group}:{artifact}' because its Maven scope/optional semantics are outside the native dependency model"
                        ),
                    });
                }
                let updated = update_pom_dependency_version(chunk, version)?;
                block.replace_range(ds..de, &updated);
                replaced = true;
                break;
            }
            search_from = de;
        }
        if !replaced {
            block = block.replacen("</dependencies>", &format!("{entry}</dependencies>"), 1);
        }
        let mut out = content.to_string();
        out.replace_range(s..e, &block);
        return Ok(out);
    }
    if content.contains("</project>") {
        return Ok(content.replacen(
            "</project>",
            &format!("  <dependencies>\n{entry}  </dependencies>\n</project>"),
            1,
        ));
    }
    Err(mgc_types::MgError::Other(
        "pom.xml has no </project> root — refusing to write".to_string(),
    ))
}

/// Update only a dependency's version element, preserving exclusions, scope,
/// optionality, classifier, and other Maven metadata.
/// Chỉ đổi version, giữ exclusions/scope/optional/classifier và metadata khác.
fn update_pom_dependency_version(dependency: &str, version: &str) -> MgResult<String> {
    if let Some(start) = dependency.find("<version>") {
        let value_start = start + "<version>".len();
        let value_end = dependency[value_start..]
            .find("</version>")
            .map(|offset| value_start + offset)
            .ok_or_else(|| {
                mgc_types::MgError::Other(
                    "malformed Maven dependency has an unclosed version element".into(),
                )
            })?;
        let mut updated = dependency.to_string();
        updated.replace_range(value_start..value_end, version);
        return Ok(updated);
    }
    let close = dependency.rfind("</dependency>").ok_or_else(|| {
        mgc_types::MgError::Other("malformed Maven dependency has no closing element".into())
    })?;
    let mut updated = dependency.to_string();
    updated.insert_str(close, &format!("<version>{version}</version>"));
    Ok(updated)
}

fn render_pom_dependency(group: &str, artifact: &str, version: &str) -> String {
    format!(
        "    <dependency>\n      <groupId>{group}</groupId>\n      <artifactId>{artifact}</artifactId>\n      <version>{version}</version>\n    </dependency>\n"
    )
}
