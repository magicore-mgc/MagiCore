use super::super::DeployTarget;
use anyhow::Result;

/// Chain từ mgc.toml `[cicd] verify` — default ["audit", "test", "build"].
pub(in crate::commands::core::dev::cicd) fn verify_chain(
    root: &std::path::Path,
) -> Result<Vec<String>> {
    let content =
        mgc_config::project::read_regular_project_text(&root.join("mgc.toml"), "project config")?
            .ok_or_else(|| anyhow::anyhow!("project config mgc.toml is missing"))?;
    let v: toml::Value = toml::from_str(&content)?;
    let chain = v
        .get("cicd")
        .and_then(|c| c.get("verify"))
        .and_then(|k| k.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|s| s.as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["audit".into(), "test".into(), "build".into()]);
    Ok(chain)
}

/// Đọc `[deploy] targets` từ mgc.toml — None = không có target, dùng provider detect.
pub(in crate::commands::core::dev::cicd) fn deploy_targets(
    root: &std::path::Path,
) -> Result<Option<Vec<DeployTarget>>> {
    let content = match mgc_config::project::read_regular_project_text(
        &root.join("mgc.toml"),
        "project config",
    ) {
        Ok(Some(content)) => content,
        Ok(None) => return Ok(None),
        Err(error) => return Err(error),
    };
    let v: toml::Value = toml::from_str(&content)?;
    let targets = v
        .get("deploy")
        .and_then(|d| d.get("targets"))
        .and_then(|t| t.as_array())
        .filter(|arr| !arr.is_empty())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t.clone().try_into().ok())
                .collect::<Vec<DeployTarget>>()
        });
    Ok(targets)
}

#[cfg(test)]
#[path = "test/project_config.rs"]
mod tests;
