use super::super::provider_config;
use super::super::templates::{
    CIRCLE_TEMPLATE, GITLAB_TEMPLATE, WORKFLOW_TEMPLATE, render_ci_template,
};
use anyhow::Result;
use std::path::PathBuf;

fn generation_plan(
    provider: mgc_cicd_adapter::CicdProvider,
) -> Result<(PathBuf, String, bool, &'static str)> {
    match provider {
        mgc_cicd_adapter::CicdProvider::GithubActions => Ok((
            PathBuf::from(".github").join("workflows").join("ci.yml"),
            render_ci_template(WORKFLOW_TEMPLATE, "CI"),
            true,
            "CI workflow generated",
        )),
        mgc_cicd_adapter::CicdProvider::Gitlab => Ok((
            PathBuf::from(".gitlab-ci.yml"),
            render_ci_template(GITLAB_TEMPLATE, ""),
            false,
            "GitLab CI generated",
        )),
        mgc_cicd_adapter::CicdProvider::CircleCi => Ok((
            PathBuf::from(".circleci").join("config.yml"),
            render_ci_template(CIRCLE_TEMPLATE, ""),
            true,
            "CircleCI config generated",
        )),
        other => Err(crate::error::ci_template_unknown(other.as_str())),
    }
}

/// `mgc ci-generate` — sinh file CI theo provider (07 §4).
/// Workflow: checkout → setup-magicore → mgc install → mgc verify.
pub fn ci_generate() -> Result<()> {
    let root = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
    let provider = provider_config()?;
    let (relative_path, workflow, create_parent, success_label) = generation_plan(provider)?;
    let path = root.join(relative_path);
    if create_parent && let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, workflow)?;
    mgc_ui::success(&format!("{success_label}: {}", path.display()));
    Ok(())
}

#[cfg(test)]
#[path = "test/generation.rs"]
mod tests;
