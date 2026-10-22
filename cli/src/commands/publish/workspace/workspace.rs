use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// Sắp workspace theo topological — package phụ thuộc published trước
pub(in crate::commands::publish) fn topo_sort_workspaces(
    workspaces: Vec<PathBuf>,
) -> Result<Vec<PathBuf>> {
    let names: std::collections::HashMap<String, PathBuf> = workspaces
        .iter()
        .filter_map(|ws| {
            let pkg = ws.join("package.json");
            if !pkg.exists() {
                return None;
            }
            let v: serde_json::Value = serde_json::from_str(&fs::read_to_string(pkg).ok()?).ok()?;
            Some((v.get("name")?.as_str()?.to_string(), ws.clone()))
        })
        .collect();

    let mut visited = std::collections::HashSet::new();
    let mut order = vec![];
    for ws in &workspaces {
        visit_ws(ws, &names, &mut visited, &mut order);
    }
    Ok(order)
}

pub(in crate::commands::publish) fn visit_ws(
    ws: &Path,
    names: &std::collections::HashMap<String, PathBuf>,
    visited: &mut std::collections::HashSet<PathBuf>,
    order: &mut Vec<PathBuf>,
) {
    if !visited.insert(ws.to_path_buf()) {
        return;
    }
    let pkg = ws.join("package.json");
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Ok(raw) = fs::read_to_string(&pkg)
        && let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw)
    {
        let mut deps: Vec<String> = vec![];
        for key in ["dependencies", "devDependencies", "peerDependencies"] {
            if let Some(map) = v.get(key).and_then(|d| d.as_object()) {
                deps.extend(map.keys().cloned());
            }
        }
        for dep in deps {
            if let Some(dep_ws) = names.get(&dep) {
                visit_ws(dep_ws, names, visited, order);
            }
        }
    }
    order.push(ws.to_path_buf());
}
