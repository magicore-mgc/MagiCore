//! Parse npm package-lock JSON — parse JSON package-lock của npm.

use anyhow::Result;
use std::collections::BTreeMap;

use crate::schema::Package;

// Tested format versions are advisory only; shape-compatible input remains importable.
// Version đã kiểm chứng chỉ dùng để cảnh báo; dữ liệu đúng cấu trúc vẫn được nhập.
const NPM_TESTED_VERSIONS: &[i64] = &[2, 3];

// ---------------------------------------------------------------------------
// npm — package-lock.json lockfileVersion 2|3 (packages map)
// npm — parse map packages từ package-lock.json v2/v3.
// ---------------------------------------------------------------------------

pub(in crate::import) fn parse_npm(
    content: &str,
    warnings: &mut Vec<String>,
    root_deps: &mut Vec<String>,
) -> Result<Vec<Package>> {
    let json: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| anyhow::anyhow!("package-lock.json is not valid JSON: {e}"))?;

    // Cổng theo CẤU TRÚC (packages map) chứ không theo số version — npm bump
    // version thường xuyên; shape giữ nguyên thì dữ liệu vẫn import đúng được.
    // (Gate on STRUCTURE, not the version number — PMs bump versions often;
    // unchanged shape means the data still imports correctly.)
    let entries = json
        .get("packages")
        .and_then(|v| v.as_object())
        .ok_or_else(|| anyhow::anyhow!("package-lock.json has no 'packages' map"))?;

    match json.get("lockfileVersion").and_then(|v| v.as_i64()) {
        Some(v) if NPM_TESTED_VERSIONS.contains(&v) => {}
        Some(other) => warnings.push(format!(
            "package-lock.json lockfileVersion {other} is newer than tested ({}) — importing by structure, review the result",
            NPM_TESTED_VERSIONS.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("/")
        )),
        None => warnings.push(
            "package-lock.json has no lockfileVersion field — importing by structure".to_string(),
        ),
    }

    // Exact lookup keys build dependency edges: "node_modules/<name>" → name@version.
    // Lookup key chính xác để dựng cạnh dependency: "node_modules/<name>" → name@version.
    let lookup: BTreeMap<String, (String, String)> = entries
        .iter()
        .filter_map(|(key, entry)| {
            let ver = entry.get("version")?.as_str()?;
            let segment = key.rsplit("node_modules/").next()?;
            let name = segment
                .strip_prefix("node_modules/")
                .filter(|n| !n.is_empty())
                .unwrap_or(segment);
            (!name.is_empty()).then_some((key.clone(), (name.to_string(), ver.to_string())))
        })
        .collect();

    // The root entry (`""`) lists the project's direct dependencies. Recording
    // them is what makes the imported lock answer "which packages did the user
    // actually declare?" — without it every dependency looks transitive, and the
    // freshness gate cannot tell a declared dependency from a hoisted one.
    // (Entry gốc `""` liệt kê dependency trực tiếp. Ghi lại để lock import trả lời
    // được "người dùng khai package nào" — thiếu nó mọi dependency trông như bắc
    // cầu và freshness không phân biệt được.)
    if let Some(root_entry) = entries.get("") {
        for section in ["dependencies", "devDependencies", "optionalDependencies"] {
            let Some(deps) = root_entry.get(section).and_then(|value| value.as_object()) else {
                continue;
            };
            for name in deps.keys() {
                let key = format!("node_modules/{name}");
                if let Some((resolved_name, version)) = lookup.get(&key) {
                    root_deps.push(format!("{resolved_name}@{version}"));
                }
            }
        }
    }

    let mut out = Vec::new();
    for (key, entry) in entries {
        // Bỏ root "" và entry không có version (link/workspace)
        // (Skip the root entry and link/workspace entries without a version)
        let Some(version) = entry.get("version").and_then(|v| v.as_str()) else {
            continue;
        };
        // Package name is the final segment after "node_modules/" in the key.
        // Tên package là đoạn cuối sau "node_modules/" trong key.
        let Some(name) = key.rsplit("node_modules/").next().and_then(|seg| {
            seg.strip_prefix("node_modules/")
                .or_else(|| (!seg.is_empty()).then_some(seg))
        }) else {
            continue;
        };
        if name.is_empty() {
            continue;
        }

        let resolved = entry
            .get("resolved")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let integrity = entry
            .get("integrity")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        // Keep an edge only when exact lookup resolves "node_modules/<dep>".
        // Chỉ giữ cạnh khi lookup tìm thấy đúng "node_modules/<dep>" (exact version).
        let dependencies: Vec<String> = entry
            .get("dependencies")
            .and_then(|v| v.as_object())
            .map(|deps| {
                deps.keys()
                    .filter_map(|dep| {
                        lookup
                            .get(&format!("node_modules/{dep}"))
                            .map(|(dn, dv)| format!("{dn}@{dv}"))
                    })
                    .collect()
            })
            .unwrap_or_default();

        out.push(Package {
            name: name.to_string(),
            version: version.to_string(),
            resolved,
            integrity,
            dependencies,
            ..Default::default()
        });
    }
    Ok(out)
}
