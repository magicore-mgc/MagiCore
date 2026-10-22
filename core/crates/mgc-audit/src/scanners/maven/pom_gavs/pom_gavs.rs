//! Projects fixed Maven POM dependencies into audit pins.
//! Chuyển dependency POM Maven có version cố định thành pin audit.

use crate::scanners::osv::OsvPin;

/// Read Maven pom.xml `<dependency>` blocks into OSV pins (`Maven`
/// ecosystem, `group:artifact` names). The block is accumulated until
/// `</dependency>` so one-tag-per-line and single-line layouts both
/// parse; tags are extracted per-name from the block text. A fixed
/// version becomes a pin; property versions (`${...}`), missing
/// versions, or missing coordinates skip HONESTLY (resolving the full
/// Maven model — parents, BOMs, profiles — is out of scope for the
/// line reader, and guessing would fake coverage).
/// Đọc block `<dependency>` trong pom.xml thành ghim OSV. Tích lũy tới
/// `</dependency>` nên cả hai layout đều parse; version cố định thành
/// ghim; version property/thiếu thì skip có ghi nhận (không đoán mò).
pub fn read_pom_gavs(raw: &str) -> (Vec<OsvPin>, Vec<String>) {
    let mut pins = Vec::new();
    let mut skipped = Vec::new();
    let mut block = String::new();
    let mut in_dep = false;
    // dependencyManagement only DECLARES versions for children — those
    // pins are not necessarily in the build, so they skip honestly
    // instead of producing findings for unshipped code.
    // dependencyManagement chỉ KHAI BÁO version cho module con — ghim đó
    // chưa chắc có trong build nên skip có ghi nhận.
    let mut in_mgmt = false;
    // XML comments are not dependencies — a commented-out block must
    // never become a pin (false positive) nor vanish silently; comment
    // spans are stripped before any tag matching.
    // Comment XML không phải dependency — block bị comment không được
    // thành ghim.
    let mut in_comment = false;
    for line in raw.lines() {
        let mut text = line.trim().to_string();
        if in_comment {
            match text.split_once("-->") {
                Some((_, rest)) => {
                    in_comment = false;
                    text = rest.trim().to_string();
                }
                None => continue,
            }
        }
        while let Some((before, rest)) = text
            .split_once("<!--")
            .map(|(b, r)| (b.to_string(), r.to_string()))
        {
            match rest.split_once("-->") {
                // Single-line comment: drop the span, keep the tails.
                // Comment một dòng: bỏ span, giữ hai đầu.
                Some((_, after)) => {
                    text = format!("{before}{after}");
                }
                // Unclosed opener: keep the head, rest is comment.
                // Opener không đóng: giữ đầu, phần còn lại là comment.
                None => {
                    text = before;
                    in_comment = true;
                    break;
                }
            }
        }
        let trimmed = text.trim();
        if trimmed.contains("<dependencyManagement") {
            in_mgmt = true;
        }
        if trimmed.contains("</dependencyManagement>") {
            in_mgmt = false;
            continue;
        }
        // `<dependency>` may open mid-line (single-line dependency
        // elements); `<dependencies>` and `<dependencyManagement>` do NOT
        // contain the exact `<dependency>`/`<dependency ` openers, so a
        // contains-check is safe.
        // `<dependency>` có thể mở giữa dòng; `<dependencies>` và
        // `<dependencyManagement>` không chứa opener chính xác nên
        // contains-check an toàn.
        if trimmed.contains("<dependency>") || trimmed.contains("<dependency ") {
            in_dep = true;
            block.clear();
        }
        if in_dep {
            block.push_str(trimmed);
            block.push('\n');
        }
        if in_dep && trimmed.contains("</dependency>") {
            in_dep = false;
            if in_mgmt {
                skipped.push(
                    "pom dependencyManagement entry is version-only (not resolved into this build)"
                        .to_string(),
                );
                continue;
            }
            match pom_block_pin(&block) {
                Ok(pin) => pins.push(pin),
                Err(reason) => skipped.push(reason),
            }
        }
    }
    (pins, skipped)
}

/// One accumulated `<dependency>` block → pin or honest skip reason.
/// Một block dependency → ghim hoặc lý do skip.
fn pom_block_pin(block: &str) -> Result<OsvPin, String> {
    let tag = |name: &str| {
        let open = format!("<{name}>");
        let close = format!("</{name}>");
        let start = block.find(open.as_str())? + open.len();
        let end = block[start..].find(close.as_str())? + start;
        Some(block[start..end].trim().to_string())
    };
    let group = tag("groupId").filter(|s| !s.is_empty());
    let name = tag("artifactId").filter(|s| !s.is_empty());
    let version = tag("version").filter(|s| !s.is_empty());
    match (group, name, version) {
        (Some(g), Some(n), Some(v)) if !v.contains('$') => Ok(OsvPin {
            name: format!("{g}:{n}"),
            version: v,
            ecosystem: "Maven",
        }),
        (Some(g), Some(n), Some(v)) => Err(format!(
            "pom dependency {g}:{n} has a property version ({v}) — full Maven model out of scope"
        )),
        _ => Err(format!(
            "pom dependency block missing groupId/artifactId/version: {}",
            block.chars().take(80).collect::<String>()
        )),
    }
}

#[cfg(test)]
#[path = "test/pom_gavs.rs"]
mod tests;
