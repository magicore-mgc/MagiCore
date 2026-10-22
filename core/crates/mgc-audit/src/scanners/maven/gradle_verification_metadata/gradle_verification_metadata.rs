//! Parses the supported Gradle verification-metadata line shape.
//! Phân tích dạng dòng verification-metadata Gradle đang hỗ trợ.

use crate::scanners::osv::OsvPin;

/// Read `gradle/verification-metadata.xml` dependencies into OSV pins.
/// The file is machine-written by Gradle with a stable shape:
/// `<dependency group="..." name="..." version="...">`. Anything
/// without all three attributes is skipped honestly.
/// Đọc dependency của verification-metadata.xml thành ghim OSV. File do
/// Gradle ghi với shape ổn định: group/name/version đủ bộ. Thiếu thuộc
/// tính nào thì skip trung thực.
pub fn read_gradle_verification_metadata(raw: &str) -> (Vec<OsvPin>, Vec<String>) {
    let mut pins = Vec::new();
    let mut skipped = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("<dependency") {
            continue;
        }
        let group = xml_attr(trimmed, "group");
        let name = xml_attr(trimmed, "name");
        let version = xml_attr(trimmed, "version");
        match (group, name, version) {
            (Some(g), Some(n), Some(v)) if !g.is_empty() && !n.is_empty() && !v.is_empty() => {
                pins.push(OsvPin {
                    name: format!("{g}:{n}"),
                    version: v,
                    ecosystem: "Maven",
                });
            }
            _ => skipped.push(format!(
                "gradle dependency line missing group/name/version: {}",
                truncate_line(trimmed, 80)
            )),
        }
    }
    (pins, skipped)
}

/// Extract `key="value"` from a single XML element line (best-effort,
/// bounded to attributes — Gradle writes them on one line).
/// Trích `key="value"` từ một dòng phần tử XML (giới hạn trong thuộc
/// tính — Gradle ghi trên một dòng).
fn xml_attr(line: &str, key: &str) -> Option<String> {
    let needle = format!("{key}=\"");
    let start = line.find(&needle)? + needle.len();
    let end = line[start..].find('"')? + start;
    Some(line[start..end].to_string())
}

fn truncate_line(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

#[cfg(test)]
#[path = "test/gradle_verification_metadata.rs"]
mod tests;
