use super::error;
use crate::commands::template::{TemplatePublishArgs, publish, registry_package_name};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Publish toàn bộ layer có đủ template.toml+sources, mọi core (web/lib/cicd/...).
/// Nguồn đọc qua TemplateRoot::resolve (MAGICORE_TEMPLATE_DIR → registry cache).
pub(in crate::commands::template) async fn publish_all() -> Result<()> {
    let root = crate::scaffold::template_root::TemplateRoot::resolve("");
    let root_path = root.path();
    if !root_path.is_dir() {
        return Err(error::web_templates_missing(root_path));
    }
    let mut layers: Vec<PathBuf> = Vec::new();
    collect_layers_under(root_path, &mut layers);
    if layers.is_empty() {
        return Err(error::no_web_layer_found(root_path));
    }
    layers.sort();
    let cores = published_template_cores(root_path, &layers)?;
    for layer in &layers {
        let rel = layer
            .strip_prefix(root_path)
            .map_err(|_| error::web_template_path_missing(&root_path.display().to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        // rel = "web/frontend/react-vite" → core = segment đầu (web/lib/cicd).
        let core = rel
            .split('/')
            .next()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| error::web_template_path_missing(&layer.display().to_string()))?
            .to_string();
        let pkg = registry_package_name(&core, &rel);
        println!("  publishing {} (as {pkg})", rel);
        publish(TemplatePublishArgs {
            core,
            name: rel,
            registry: None,
            version: None,
        })
        .await?;
    }
    let core_label = if cores.len() == 1 { "core" } else { "cores" };
    println!(
        "Publish-all done: {} layers across {} {}: {}",
        layers.len(),
        cores.len(),
        core_label,
        cores.join(", ")
    );
    Ok(())
}

pub(in crate::commands::template) fn published_template_cores(
    root: &Path,
    layers: &[PathBuf],
) -> Result<Vec<String>> {
    let mut cores = std::collections::BTreeSet::new();
    for layer in layers {
        let relative = layer
            .strip_prefix(root)
            .map_err(|_| error::web_template_path_missing(&layer.display().to_string()))?;
        let Some(core) = relative.components().next() else {
            return Err(error::web_template_path_missing(
                &layer.display().to_string(),
            ));
        };
        let core = core.as_os_str().to_string_lossy().into_owned();
        if core.is_empty() {
            return Err(error::web_template_path_missing(
                &layer.display().to_string(),
            ));
        }
        cores.insert(core);
    }
    Ok(cores.into_iter().collect())
}

/// Thu thập mọi folder có template.toml + sources dưới root (không đệ quy vào leaf đã chứa).
pub(super) fn collect_layers_under(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.join("template.toml").is_file() && path.join("sources").is_dir() {
                out.push(path);
            } else {
                collect_layers_under(&path, out);
            }
        }
    }
}
