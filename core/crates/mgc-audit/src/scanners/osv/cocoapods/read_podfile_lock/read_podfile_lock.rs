//! PODS section parsing into pins and explicit skipped entries.
//! Parse mục PODS thành pin và ghi rõ entry bị bỏ qua.

use crate::scanners::osv::OsvPin;
use mgc_types::MgResult;

/// Read Podfile.lock into OSV pins (`pods` ecosystem = OSV name for
/// CocoaPods). DEPENDENCIES/pods with local :path or git sources are
/// skipped honestly (no registry advisory mapping).
/// Đọc Podfile.lock thành ghim OSV (ecosystem `pods` — tên OSV cho
/// CocoaPods). Pod nguồn local :path hoặc git được bỏ trung thực
/// (không mapping advisory registry).
pub fn read_podfile_lock(raw: &str) -> MgResult<(Vec<OsvPin>, Vec<String>)> {
    let mut pins = Vec::new();
    let mut skipped = Vec::new();
    let mut in_pods = false;
    for line in raw.lines() {
        let trimmed = line.trim_end();
        if trimmed == "PODS:" {
            in_pods = true;
            continue;
        }
        if in_pods {
            if trimmed.is_empty() || !trimmed.starts_with(' ') {
                // Section ended (blank line or non-indented key like
                // DEPENDENCIES:).
                // Section hết (dòng trống hoặc key không thụt lề như
                // DEPENDENCIES:).
                break;
            }
            let entry = trimmed.trim();
            // Top-level pods are "- Name (version)" at the first indent
            // level; sub-dependencies nest deeper (more leading spaces
            // relative to the pod line). Compare indentation: top-level
            // pod lines carry exactly the "- " bullet.
            // Pod cấp gốc là "- Name (version)" thụt lề đầu tiên; dep
            // con thụt sâu hơn. So sánh thụt lề: dòng pod gốc mang
            // đúng bullet "- ".
            let indent = line.len() - line.trim_start().len();
            if indent > 2 {
                // Deeper-indented lines are sub-deps of the enclosing
                // pod — their versions belong to the parent listing,
                // not separate pins.
                // Dòng thụt sâu hơn là dep con của pod bao quanh —
                // version của chúng thuộc listing cha, không phải ghim
                // riêng.
                continue;
            }
            let Some(entry) = entry.strip_prefix("- ") else {
                skipped.push(format!("{entry}: PODS entry must start with '- '"));
                continue;
            };
            // Top-level pod: "Name (1.2.3)" or "Name/Sub (1.2.3)" or
            // "Name/Variant (1.2.3):" with a trailing dep list colon.
            // Pod cấp gốc: "Name (1.2.3)" / "Name/Sub (1.2.3)" /
            // "Name/Variant (1.2.3):" kèm dấu hai chấm danh sách dep.
            let entry = entry.trim_end_matches(':');
            let Some(open) = entry.rfind('(') else {
                skipped.push(format!("{entry}: no version in Podfile.lock entry"));
                continue;
            };
            let Some(close) = entry[open..].find(')').map(|i| open + i) else {
                skipped.push(format!("{entry}: malformed version parens"));
                continue;
            };
            let name = entry[..open].trim().to_string();
            let version = entry[open + 1..close].trim().to_string();
            if version.contains(':') || version.contains("git") {
                skipped.push(format!("{name}: local/git pod has no advisory mapping"));
                continue;
            }
            if name.is_empty() || version.is_empty() {
                skipped.push(format!("{entry}: empty name or version"));
                continue;
            }
            pins.push(OsvPin {
                name,
                version,
                ecosystem: "pods",
            });
        }
    }
    Ok((pins, skipped))
}
