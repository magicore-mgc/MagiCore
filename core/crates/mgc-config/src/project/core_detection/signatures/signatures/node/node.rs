//! node manifest recognition.
//! Nhận diện manifest node.
use super::super::shared::signature_file_exists;
use crate::project::{ProjectConfig, read_regular_project_text};
use std::path::Path;

/// Known Node framework markers used for conservative project-core inference.
/// Dấu framework Node dùng để suy luận core dự án theo hướng bảo thủ.
const NODE_CORE_MARKERS: &[(&[&str], &str)] = &[
    (
        &["react-native", "expo", "@capacitor/core", "electron"],
        "app",
    ),
    (
        &["phaser", "pixi.js", "@babylonjs/core", "playcanvas"],
        "game",
    ),
    (&["johnny-five", "firmata", "espruino"], "iot"),
    (
        &[
            "@anthropic-ai/sdk",
            "@google/generative-ai",
            "@langchain/core",
            "langchain",
            "openai",
            "@tensorflow/tfjs",
        ],
        "ai",
    ),
    (
        &[
            "@angular/core",
            "@remix-run/react",
            "@sveltejs/kit",
            "astro",
            "express",
            "fastify",
            "hono",
            "koa",
            "next",
            "nuxt",
            "react",
            "react-dom",
            "svelte",
            "vue",
        ],
        "web",
    ),
    (
        &["@pulumi/pulumi", "cdk", "serverless", "sst", "wrangler"],
        "clo",
    ),
    (&["@actions/core", "@actions/github", "probot"], "cicd"),
];

/// Infer the core from dependency metadata and explicit CLI package metadata.
/// Suy luận core từ dependency và metadata CLI khai báo tường minh.
pub(in crate::project::project::core_detection) fn node_manifest_core_hint(
    manifest: &serde_json::Value,
) -> anyhow::Result<Option<String>> {
    let dependencies = [
        "dependencies",
        "devDependencies",
        "optionalDependencies",
        "peerDependencies",
    ]
    .into_iter()
    .filter_map(|section| manifest.get(section).and_then(serde_json::Value::as_object));
    let names = dependencies
        .flat_map(|section| section.keys().map(|name| name.to_ascii_lowercase()))
        .collect::<std::collections::BTreeSet<_>>();
    let mut detected = NODE_CORE_MARKERS
        .iter()
        .filter(|(_, core)| *core != "web")
        .filter(|(markers, _)| markers.iter().any(|name| names.contains(*name)))
        .map(|(_, core)| *core)
        .collect::<std::collections::BTreeSet<_>>();
    let explicit_web_marker = NODE_CORE_MARKERS
        .iter()
        .find(|(_, core)| *core == "web")
        .is_some_and(|(markers, _)| {
            markers
                .iter()
                .filter(|name| **name != "react" && **name != "react-dom")
                .any(|name| names.contains(*name))
        });
    if explicit_web_marker || (detected.is_empty() && names.contains("react")) {
        detected.insert("web");
    }
    if detected.len() > 1 {
        anyhow::bail!(
            "ambiguous package.json framework dependencies identify multiple MagiCore cores ({}); declare one core in package.json under 'magicore.core' or in '{}'",
            detected.into_iter().collect::<Vec<_>>().join(", "),
            ProjectConfig::CORE_MARKER_FILE,
        );
    }
    if let Some(core) = detected.into_iter().next() {
        return Ok(Some(core.to_string()));
    }
    if manifest.get("bin").is_some_and(|value| !value.is_null()) {
        return Ok(Some("lib".to_string()));
    }
    Ok(None)
}

/// Read and classify package.json while preserving signature validation order.
/// Đọc và phân loại package.json, giữ nguyên thứ tự xác minh signature.
pub(in crate::project::project::core_detection) fn detect_node_core(
    project_root: &Path,
    has_cloud_marker: bool,
) -> anyhow::Result<Option<String>> {
    let path = project_root.join("package.json");
    let has_package_manifest = signature_file_exists(&path)?;
    if !has_package_manifest || has_cloud_marker {
        return Ok(None);
    }
    let content = read_regular_project_text(&path, "package manifest")?
        .ok_or_else(|| anyhow::anyhow!("package manifest disappeared during core detection"))?;
    let manifest: serde_json::Value = serde_json::from_str(&content)?;
    Ok(Some(
        node_manifest_core_hint(&manifest)?.unwrap_or_else(|| "web".to_string()),
    ))
}
