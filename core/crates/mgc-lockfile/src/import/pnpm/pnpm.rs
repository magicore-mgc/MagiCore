//! Parse pnpm lockfile YAML — parse YAML lockfile của pnpm.

use crate::schema::Package;
use anyhow::Result;

use super::super::{split_name_version, strip_paren_suffix};

// Tested format versions are advisory only; shape-compatible input remains importable.
// Version đã kiểm chứng chỉ dùng để cảnh báo; dữ liệu đúng cấu trúc vẫn được nhập.
const PNPM_TESTED_VERSIONS: &[&str] = &["9.0"];

// ---------------------------------------------------------------------------
// pnpm — pnpm-lock.yaml lockfileVersion '9.0'
// pnpm — parser cho định dạng pnpm-lock.yaml.
// ---------------------------------------------------------------------------

pub(in crate::import) fn parse_pnpm(
    content: &str,
    warnings: &mut Vec<String>,
    root_deps: &mut Vec<String>,
) -> Result<Vec<Package>> {
    // Multi-document files (pnpm ≥10 env document + project document):
    // a single-doc parse silently returns the FIRST (env) document — the
    // exact trap Deno documents for pnpm consumers. Always parse every
    // document and take the LAST (project graph); the env document is
    // reported, never silently merged (different universe: .pnpm-config).
    // (File nhiều document: parse đơn-doc âm thầm trả document ĐẦU (env)
    // — đúng bẫy Deno đã ghi. Luôn parse mọi document và lấy CUỐI
    // (graph project); document env được báo cáo, không bao giờ gộp âm
    // thầm.)
    use serde::Deserialize;
    let documents: Vec<serde_yaml::Value> = serde_yaml::Deserializer::from_str(content)
        .map(serde_yaml::Value::deserialize)
        .collect::<Result<_, _>>()
        .map_err(|e| anyhow::anyhow!("pnpm-lock.yaml is not valid YAML: {e}"))?;
    if documents.len() > 1 {
        let env_packages = documents[..documents.len() - 1]
            .iter()
            .filter_map(|doc| doc.get("packages"))
            .filter_map(|packages| packages.as_mapping())
            .map(|mapping| mapping.len())
            .sum::<usize>();
        warnings.push(format!(
            "pnpm-lock.yaml has {} documents: using the LAST (project graph); {} env-document package(s) recorded but not imported (configDependencies live outside the project graph)",
            documents.len(),
            env_packages
        ));
    }
    let yaml = documents
        .last()
        .ok_or_else(|| anyhow::anyhow!("pnpm-lock.yaml is empty"))?;

    // Shape-first như npm — pnpm đổi '9.0' → '10.0'... không cần sửa code.
    // (Shape-first like npm — pnpm bumping '9.0' → '10.0' needs no code change.)
    let entries = yaml
        .get("packages")
        .and_then(|v| v.as_mapping())
        .ok_or_else(|| anyhow::anyhow!("pnpm-lock.yaml has no 'packages' map"))?;

    match yaml.get("lockfileVersion").and_then(|v| v.as_str()) {
        Some(v) if PNPM_TESTED_VERSIONS.contains(&v) => {}
        Some(other) => warnings.push(format!(
            "pnpm-lock.yaml lockfileVersion {other:?} is newer than tested ({}) — importing by structure, review the result",
            PNPM_TESTED_VERSIONS.iter().map(|v| (*v).to_string()).collect::<Vec<_>>().join("/")
        )),
        None => warnings.push(
            "pnpm-lock.yaml has no lockfileVersion field — importing by structure".to_string(),
        ),
    }

    // Known (name, version) pairs constrain edges to exact dependencies.
    // Tập (name,version) đã khóa để chỉ dựng cạnh tới dependency exact.
    let known: std::collections::HashSet<(String, String)> = entries
        .keys()
        .filter_map(|k| k.as_str())
        // Keys with peer suffixes are trimmed before splitting the package name and version.
        // Key có hậu tố peer "/name@1.2.3(peer@x)" được cắt trước khi tách.
        .map(|raw| strip_paren_suffix(raw.trim_start_matches('/')).to_string())
        .filter_map(|cleaned| split_name_version(&cleaned))
        .collect();

    // pnpm lists direct dependencies under `importers."."` (and per workspace
    // package). Recording them gives the imported lock a root graph; without it
    // every dependency looks transitive.
    // (pnpm liệt kê dependency trực tiếp ở `importers."."` — ghi lại để lock
    // import có graph root.)
    if let Some(root_importer) = yaml
        .get("importers")
        .and_then(|importers| importers.get("."))
    {
        for section in ["dependencies", "devDependencies", "optionalDependencies"] {
            let Some(deps) = root_importer.get(section).and_then(|v| v.as_mapping()) else {
                continue;
            };
            for (name_value, spec) in deps {
                let Some(name) = name_value.as_str() else {
                    continue;
                };
                // `version` holds the resolved version; `specifier` holds the range.
                // (`version` là version đã resolve; `specifier` là range.)
                if let Some(version) = spec.get("version").and_then(|v| v.as_str()) {
                    let version = strip_paren_suffix(version);
                    root_deps.push(format!("{name}@{version}"));
                }
            }
        }
    }

    let mut out = Vec::new();
    for (key, entry) in entries {
        let Some(raw) = key.as_str() else { continue };
        // Keys have the form "/name@1.2.3(peer@x)" or "name@1.2.3".
        // Key có dạng "/name@1.2.3(peer@x)" hoặc "name@1.2.3".
        let Some((name, version)) = split_name_version(strip_paren_suffix(raw)) else {
            continue;
        };
        let integrity = entry
            .get("resolution")
            .and_then(|r| r.get("integrity"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let resolved = entry
            .get("resolution")
            .and_then(|r| r.get("tarball"))
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();

        // Keep an edge only when the dependency resolves to a known exact version.
        // Chỉ giữ cạnh khi dependency trỏ tới version exact đã có trong tập.
        // The map key supplies the name for plain versions or name@version specs.
        // Tên lấy từ key map khi spec chỉ chứa version hoặc chứa name@version.
        let dependencies: Vec<String> = entry
            .get("dependencies")
            .and_then(|v| v.as_mapping())
            .map(|deps| {
                deps.iter()
                    .filter_map(|(dep_key, spec)| {
                        let dep_name = dep_key.as_str()?;
                        let spec = spec.as_str()?;
                        let cleaned = strip_paren_suffix(spec.trim_start_matches('/'));
                        // A spec can be "name@version" or a plain version such as "1.2.3".
                        // Spec có thể là "name@version" hoặc version trơn như "1.2.3".
                        let candidate = match split_name_version(cleaned) {
                            Some(pair) => Some(pair),
                            None => {
                                let looks_exact = !cleaned.is_empty()
                                    && cleaned.chars().next().is_some_and(|c| c.is_ascii_digit());
                                looks_exact.then(|| (dep_name.to_string(), cleaned.to_string()))
                            }
                        }?;
                        known
                            .contains(&candidate)
                            .then(|| format!("{}@{}", candidate.0, candidate.1))
                    })
                    .collect()
            })
            .unwrap_or_default();

        out.push(Package {
            name,
            version,
            resolved,
            integrity,
            dependencies,
            ..Default::default()
        });
    }
    Ok(out)
}
