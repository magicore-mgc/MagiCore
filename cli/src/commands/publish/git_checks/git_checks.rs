use super::GitReadQuery;
use anyhow::{Result, anyhow, bail};
use std::path::Path;

/// Check the selected project's tree and tracking branch before publishing.
/// Kiểm tra working tree và upstream của đúng project trước khi publish.
pub(in crate::commands) fn git_checks(
    publish_branch: Option<&str>,
    project_root: &Path,
) -> Result<()> {
    // Tree clean (except ignored)
    let status = run_git_capture(project_root, GitReadQuery::WorkingTreeStatus)?;
    if !status.trim().is_empty() {
        bail!("Git tree not clean — commit or stash changes first (or --no-git-checks)");
    }

    // Current branch
    let branch = match publish_branch {
        Some(branch) => {
            run_git_capture(
                project_root,
                GitReadQuery::ValidateBranch(branch.to_string()),
            )?;
            branch.to_string()
        }
        None => run_git_capture(project_root, GitReadQuery::CurrentBranch)?
            .trim()
            .to_string(),
    };
    if branch.is_empty() {
        bail!("Could not determine the Git branch to publish");
    }

    // Has upstream
    let upstream = run_git_capture(project_root, GitReadQuery::UpstreamRef(branch.clone()));
    if upstream.is_err() {
        if publish_branch.is_some() {
            bail!("Publish branch {branch} has no resolvable upstream");
        }
        return Ok(());
    }

    // Check lag between the selected local branch and its matching upstream.
    // So sánh nhánh cục bộ đã chọn với upstream tương ứng để phát hiện commit bị chậm.
    let lag = run_git_capture(project_root, GitReadQuery::CommitsBehind(branch.clone()))?;
    let count = parse_git_behind_count(&lag)?;
    if count > 0 {
        bail!(
            "Branch {} lags {} commits behind upstream — pull first",
            branch,
            count
        );
    }

    Ok(())
}

pub(in crate::commands) fn parse_git_behind_count(output: &str) -> Result<usize> {
    output
        .trim()
        .parse::<usize>()
        .map_err(|_| anyhow!("Git returned an invalid upstream commit count"))
}

pub(in crate::commands) fn git_exec_options(project_root: &Path) -> mgc_exec::prelude::ExecOptions {
    mgc_exec::prelude::ExecOptions {
        cwd: Some(project_root.to_path_buf()),
        clean_env: true,
        ..Default::default()
    }
}

pub(in crate::commands) fn run_git_capture(
    project_root: &Path,
    query: GitReadQuery,
) -> Result<String> {
    let args = query.argv();
    let opts = git_exec_options(project_root);
    let report = mgc_exec::prelude::run("git", &args, &opts)?;
    Ok(report.stdout_tail)
}
