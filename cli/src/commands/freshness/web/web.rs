//! Owns Web framework identity and lock-version resolution. — Sở hữu identity framework Web và resolve version từ lock.

use anyhow::Context;
use std::path::Path;

use anyhow::Result;

use super::super::lock::locked_packages;

/// Known framework package → profile framework id.
/// Package framework đã biết → id framework cho profile.
fn framework_id_for_package(name: &str) -> Option<&'static str> {
    match name {
        "next" => Some("next"),
        "nuxt" => Some("nuxt"),
        "@angular/core" => Some("angular"),
        "@remix-run/react" | "@remix-run/node" | "@react-router/dev" => Some("remix"),
        "astro" => Some("astro"),
        "@sveltejs/kit" => Some("sveltekit"),
        "vite" => Some("vite"),
        "react-scripts" => Some("react-scripts"),
        _ => None,
    }
}
/// Resolve the web framework and its locked version for profile lookup
/// (Spec rev4 §6/§11): framework id from manifest dependencies, version
/// from the locked entry. `None` when no known framework or no lock entry.
/// (Xác định framework web và version đã lock để tra profile (Spec rev4
/// §6/§11): id framework từ dependency manifest, version từ entry lock.
/// `None` khi không có framework đã biết hoặc không có entry lock.)
pub fn resolve_web_framework(project_root: &Path) -> Result<Option<(String, String)>> {
    let manifest_path = project_root.join("package.json");
    if !manifest_path.exists() {
        return Ok(None);
    }
    let manifest: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path)
            .with_context(|| format!("cannot read {}", manifest_path.display()))?,
    )
    .with_context(|| format!("{} is not valid JSON", manifest_path.display()))?;
    let mut frameworks = Vec::new();
    if let Some(obj) = manifest.as_object() {
        for section in ["dependencies", "devDependencies"] {
            if let Some(deps) = obj.get(section).and_then(|v| v.as_object()) {
                for name in deps.keys() {
                    if let Some(id) = framework_id_for_package(name) {
                        frameworks.push(id.to_string());
                    }
                }
            }
        }
    }
    frameworks.sort();
    frameworks.dedup();
    if frameworks.len() != 1 {
        return Ok(None);
    }
    let Some(framework) = frameworks.into_iter().next() else {
        return Ok(None);
    };
    // Locked version: exact installed version binds the profile (Spec rev4 §6).
    // (Version đã lock: version cài chính xác ràng buộc profile (Spec rev4 §6).)
    let package_name = match framework.as_str() {
        "angular" => "@angular/core",
        "remix" => "@remix-run/react",
        _ => framework.as_str(),
    };
    let version = locked_packages(project_root)?
        .iter()
        .find(|(name, _)| name == package_name || name == &framework)
        .map(|(_, version)| version.clone());
    Ok(version.map(|v| (framework, v)))
}
