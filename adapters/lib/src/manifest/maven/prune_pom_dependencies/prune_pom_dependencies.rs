//! Maven dependency pruning.
//! Loại dependency Maven cũ.
use super::super::pom_dependency_is_mgc_managed::pom_dependency_is_mgc_managed;

/// Drop top-level `<dependency>` blocks whose group:artifact is absent
/// from `pins`. Other blocks (dependencyManagement, plugins, profiles)
/// are never touched.
pub(crate) fn prune_pom_dependencies(content: &str, pins: &[(String, String, String)]) -> String {
    // Collect top-level <dependency> spans with the same depth walk as
    // upsert (duplicated walk for clarity over cleverness).
    let mut spans: Vec<(usize, usize, String, String, bool)> = Vec::new();
    let bytes = content.as_bytes();
    let mut depth = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            if content[i..].starts_with("</") {
                let end = content[i..].find('>').map(|p| i + p).unwrap_or(bytes.len());
                depth = depth.saturating_sub(1);
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
            let tag = content[i + 1..end].trim().to_string();
            let name = tag.split_whitespace().next().unwrap_or("").to_string();
            if name == "dependency"
                && depth == 2
                && let Some(fe) = content[end..]
                    .find("</dependency>")
                    .map(|p| end + p + "</dependency>".len())
            {
                let chunk = &content[i..fe];
                let g = chunk
                    .split("<groupId>")
                    .nth(1)
                    .and_then(|s| s.split("</groupId>").next())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let a = chunk
                    .split("<artifactId>")
                    .nth(1)
                    .and_then(|s| s.split("</artifactId>").next())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                spans.push((i, fe, g, a, pom_dependency_is_mgc_managed(chunk)));
                i = fe;
                continue;
            }
            if !tag.ends_with('/') && !name.starts_with('?') && !name.starts_with('!') {
                depth += 1;
            }
            i = end + 1;
            continue;
        }
        i += 1;
    }
    let mut out = content.to_string();
    for (s, e, g, a, managed) in spans.iter().rev() {
        if *managed && !pins.iter().any(|(pg, pa, _)| pg == g && pa == a) {
            out.replace_range(*s..*e, "");
        }
    }
    // Collapse triple blank lines left by removals (cosmetic only).
    while out.contains("\n\n\n") {
        out = out.replace("\n\n\n", "\n\n");
    }
    out
}
