use super::super::DeployTarget;
use anyhow::Result;

/// Chain từ mgc.toml `[cicd] verify` — default theo core của project.
/// Chain từ mgc.toml `[cicd] verify` — mặc định theo core của project.
///
/// A CI/CD project produces pipeline configuration, so a default chain that
/// ends in `build` could never succeed: the build step rejects that core by
/// design. The default therefore mirrors what the core can actually do, and an
/// explicit `[cicd].verify` still wins.
/// (Project CI/CD sinh cấu hình pipeline nên chain mặc định kết thúc bằng
/// `build` không bao giờ thành công: bước build từ chối core đó theo thiết kế.
/// Mặc định vì thế phản ánh đúng khả năng của core, còn `[cicd].verify` tường
/// minh vẫn thắng.)
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
        .unwrap_or_else(|| default_chain(root));
    Ok(chain)
}

/// Default verify chain for the project's core.
/// Chain verify mặc định theo core của project.
fn default_chain(root: &std::path::Path) -> Vec<String> {
    let core = mgc_config::project::ProjectConfig::load(root)
        .ok()
        .flatten()
        .map(|config| config.ecosystem)
        .unwrap_or_default();
    if core == "cicd" {
        vec!["audit".into(), "test".into()]
    } else {
        vec!["audit".into(), "test".into(), "build".into()]
    }
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
