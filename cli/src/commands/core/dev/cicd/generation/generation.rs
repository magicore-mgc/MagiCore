use super::super::provider_config;
use super::super::templates::{
    ARGOCD_APPLICATION_TEMPLATE, ARGOCD_PROJECT_TEMPLATE, AZURE_TEMPLATE, BITBUCKET_TEMPLATE,
    CIRCLE_TEMPLATE, GITLAB_TEMPLATE, JENKINS_TEMPLATE, WORKFLOW_TEMPLATE, render_ci_template,
    render_ci_template_with_repo,
};
use anyhow::Result;
use std::path::PathBuf;

fn generation_plan(
    provider: mgc_cicd_adapter::CicdProvider,
    name: &str,
) -> Result<Vec<(PathBuf, String, bool, &'static str)>> {
    match provider {
        mgc_cicd_adapter::CicdProvider::GithubActions => Ok(vec![(
            PathBuf::from(".github").join("workflows").join("ci.yml"),
            render_ci_template(WORKFLOW_TEMPLATE, "CI"),
            true,
            "CI workflow generated",
        )]),
        mgc_cicd_adapter::CicdProvider::Gitlab => Ok(vec![(
            PathBuf::from(".gitlab-ci.yml"),
            render_ci_template(GITLAB_TEMPLATE, ""),
            false,
            "GitLab CI generated",
        )]),
        mgc_cicd_adapter::CicdProvider::CircleCi => Ok(vec![(
            PathBuf::from(".circleci").join("config.yml"),
            render_ci_template(CIRCLE_TEMPLATE, ""),
            true,
            "CircleCI config generated",
        )]),
        // Jenkins reads a `Jenkinsfile` at the repository root; no parent
        // directory has to be created for it.
        // (Jenkins đọc `Jenkinsfile` ở root repo; không cần tạo thư mục cha.)
        mgc_cicd_adapter::CicdProvider::Jenkins => Ok(vec![(
            PathBuf::from("Jenkinsfile"),
            render_ci_template(JENKINS_TEMPLATE, ""),
            false,
            "Jenkinsfile generated",
        )]),
        mgc_cicd_adapter::CicdProvider::AzurePipelines => Ok(vec![(
            PathBuf::from("azure-pipelines.yml"),
            render_ci_template(AZURE_TEMPLATE, ""),
            false,
            "Azure Pipelines config generated",
        )]),
        mgc_cicd_adapter::CicdProvider::BitbucketPipelines => Ok(vec![(
            PathBuf::from("bitbucket-pipelines.yml"),
            render_ci_template(BITBUCKET_TEMPLATE, ""),
            false,
            "Bitbucket Pipelines config generated",
        )]),
        // An ArgoCD project is detected by its marker, so it must also be
        // generatable; emitting a standard AppProject/Application pair is the
        // only output that a real cluster accepts.
        // (Project ArgoCD được nhận diện qua marker nên cũng phải sinh được;
        // cặp AppProject/Application chuẩn là output thật mà cluster chấp nhận.)
        mgc_cicd_adapter::CicdProvider::Argocd => Ok(vec![
            (
                PathBuf::from(".argocd").join("app-project.yaml"),
                render_ci_template(ARGOCD_PROJECT_TEMPLATE, name),
                true,
                "ArgoCD AppProject generated",
            ),
            (
                PathBuf::from(".argocd").join("application.yaml"),
                render_ci_template_with_repo(ARGOCD_APPLICATION_TEMPLATE, name, &repo_url()),
                true,
                "ArgoCD Application generated",
            ),
        ]),
        other => Err(crate::error::ci_template_unknown(other.as_str())),
    }
}

/// `mgc ci-generate` — sinh file CI theo provider (07 §4).
/// Workflow: checkout → setup-magicore → mgc install → mgc verify.
/// Repository URL recorded in generated ArgoCD manifests.
/// URL repository ghi trong manifest ArgoCD sinh ra.
///
/// Read-only lookup of the configured origin; an unreadable remote yields a
/// placeholder rather than an empty field, because an empty `repoURL` makes
/// the manifest invalid at apply time.
/// (Đọc origin đã cấu hình; remote không đọc được trả placeholder thay vì
/// trường rỗng, vì `repoURL` rỗng làm manifest không apply được.)
fn repo_url() -> String {
    // Read `.git/config` rather than spawning git: the generator runs inside a
    // dependency-audited command surface, and the only value needed is the
    // configured origin URL.
    // (Đọc `.git/config` thay vì spawn git: generator nằm trong bề mặt lệnh đã
    // audit dependency, và giá trị cần duy nhất là origin URL đã cấu hình.)
    let output = mgc_config::project::read_regular_project_text(
        &std::env::current_dir()
            .unwrap_or_default()
            .join(".git")
            .join("config"),
        "git remote origin",
    );
    match output {
        Ok(Some(text)) => origin_from_git_config(&text)
            .unwrap_or_else(|| "https://example.invalid/repository".to_string()),
        _ => "https://example.invalid/repository".to_string(),
    }
}

/// Extract `remote.origin.url` from a git config document.
/// Lấy `remote.origin.url` từ nội dung git config.
fn origin_from_git_config(text: &str) -> Option<String> {
    let mut in_origin = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_origin = trimmed.replace(' ', "") == "[remote\"origin\"]";
            continue;
        }
        if in_origin
            && let Some((key, value)) = trimmed.split_once('=')
            && key.trim() == "url"
        {
            let url = value.trim().to_string();
            if !url.is_empty() {
                return Some(url);
            }
        }
    }
    None
}

pub fn ci_generate() -> Result<()> {
    // SEC-10: fail closed before touching the filesystem when the build has no
    // release pin.
    // (SEC-10: fail-closed trước khi chạm filesystem khi build không có pin.)
    super::super::templates::ensure_release_pin()?;
    let root = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
    let provider = provider_config()?;
    let name = project_display_name(&root);
    let plan = generation_plan(provider, &name)?;
    for (relative_path, workflow, create_parent, success_label) in plan {
        let path = root.join(relative_path);
        if create_parent && let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // Never clobber a hand-edited pipeline without leaving the old bytes: a
        // generated file that silently replaced local edits loses work.
        // (Không ghi đè pipeline đã sửa tay mà không giữ bytes cũ: file sinh ra
        // thay thế im lặng sẽ mất việc.)
        if path.exists() {
            let existing = std::fs::read_to_string(&path)?;
            if existing != workflow {
                let backup = path.with_extension("yml.bak");
                std::fs::write(&backup, existing)?;
                mgc_ui::info(&format!("previous pipeline saved to {}", backup.display()));
            }
        }
        std::fs::write(&path, workflow)?;
        mgc_ui::success(&format!("{success_label}: {}", path.display()));
    }
    Ok(())
}

/// Project name recorded in generated manifests (directory name).
/// Tên project ghi trong manifest sinh ra (tên thư mục).
fn project_display_name(root: &std::path::Path) -> String {
    root.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("project")
        .to_string()
}

#[cfg(test)]
#[path = "test/generation.rs"]
mod tests;

#[cfg(test)]
#[path = "test/golden_files.rs"]
mod golden_files;
