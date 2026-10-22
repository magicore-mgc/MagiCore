//! Owns the reserved Maven coordinate projection only; no freshness gate is enabled. — Sở hữu projection tọa độ Maven đang để dành; chưa bật freshness gate.

/// Extract `group:artifact` coordinates from a Maven `pom.xml`, matching
/// the resolver coordinate shape (`group:artifact` — see
/// `mgc-resolver/src/protocols/maven.rs`). Per-dependency `<groupId>` wins;
/// a missing one falls back to the project `<groupId>`.
/// (Trích tọa độ `group:artifact` từ `pom.xml` Maven, khớp dạng coordinate
/// của resolver. `<groupId>` từng dependency thắng; thiếu thì fallback
/// `<groupId>` project.)
///
/// Reserved: wired only where a native Maven lane writes `mgc.lock`
/// (follow-up per lane).
/// (Dành sẵn: chỉ nối nơi lane Maven native ghi `mgc.lock`
/// (follow-up từng lane).)
#[allow(dead_code)]
pub fn declared_maven_deps(pom_xml: &str) -> Vec<String> {
    fn text_of(block: &str, tag: &str) -> Option<String> {
        let open = format!("<{tag}>");
        let close = format!("</{tag}>");
        let start = block.find(&open)? + open.len();
        let end = block[start..].find(&close)?;
        let value = block[start..start + end].trim();
        (!value.is_empty()).then(|| value.to_string())
    }
    // Project groupId fallback (first <groupId> outside <dependencies>).
    // (Fallback groupId project (<groupId> đầu ngoài <dependencies>).)
    let head = pom_xml.split("<dependencies>").next().unwrap_or("");
    let project_group = text_of(head, "groupId");
    let mut names = Vec::new();
    let Some(start) = pom_xml.find("<dependencies>") else {
        return names;
    };
    let mut rest = &pom_xml[start + "<dependencies>".len()..];
    let end = rest.find("</dependencies>").unwrap_or(rest.len());
    rest = &rest[..end];
    while let Some(dep_start) = rest.find("<dependency>") {
        rest = &rest[dep_start + "<dependency>".len()..];
        let dep_end = rest.find("</dependency>").unwrap_or(rest.len());
        let block = &rest[..dep_end];
        if let Some(artifact) = text_of(block, "artifactId") {
            let group = text_of(block, "groupId")
                .or_else(|| project_group.clone())
                .unwrap_or_default();
            if !group.is_empty() {
                names.push(format!("{group}:{artifact}"));
            }
        }
        rest = &rest[dep_end.min(rest.len())..];
    }
    names.sort();
    names.dedup();
    names
}
