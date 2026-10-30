use super::{
    project_publish::publish_project, trusted_publish::publish_trusted_non_npm,
    workspace::topo_sort_workspaces,
};
use crate::commands::install;
use anyhow::{Result, anyhow};
use clap::Args;
use mgc_config::project::ProjectConfig;
use mgc_pack::tarball::pack;
use mgc_ui::info;
use std::fs;
use std::path::PathBuf;

#[derive(Args, Clone)]
pub struct PublishArgs {
    #[arg(long, value_parser = ["npm", "pypi", "oci"], default_value = "npm")]
    pub protocol: String,
    #[arg(long, requires = "trusted", help = "package name for PyPI or OCI")]
    pub package: Option<String>,
    #[arg(
        long,
        requires = "trusted",
        help = "version for PyPI or OCI tag for OCI"
    )]
    pub version: Option<String>,
    #[arg(
        long = "artifact",
        requires = "trusted",
        help = "PyPI or OCI artifact file (repeatable)"
    )]
    pub artifacts: Vec<PathBuf>,
    #[arg(
        long,
        requires = "trusted",
        help = "local Docker image reference for OCI image publish"
    )]
    pub image: Option<String>,
    #[arg(long, help = "dist-tag (default: latest)")]
    pub tag: Option<String>,
    #[arg(long, help = "access level: public|restricted")]
    pub access: Option<String>,
    #[arg(long, help = "pack + verify, do not publish")]
    pub dry_run: bool,
    #[arg(long, help = "output JSON")]
    pub json: bool,
    #[arg(long, help = "OTP code for 2FA")]
    pub otp: Option<String>,
    #[arg(long, help = "delete existing version then republish (409)")]
    pub force: bool,
    #[arg(long, help = "skip lifecycle scripts")]
    pub ignore_scripts: bool,
    #[arg(long, help = "explicitly allow package publish lifecycle scripts")]
    pub allow_scripts: bool,
    #[arg(long, help = "skip git checks")]
    pub no_git_checks: bool,
    #[arg(
        long,
        help = "publish branch (default: current branch tracking remote)"
    )]
    pub publish_branch: Option<String>,
    #[arg(long, help = "all in one PUT (Phase 3)")]
    pub batch: bool,
    #[arg(long, help = "write JSON report file")]
    pub report_summary: bool,
    #[arg(long, help = "version bump patch")]
    pub patch: bool,
    #[arg(long, help = "version bump minor")]
    pub minor: bool,
    #[arg(long, help = "version bump major")]
    pub major: bool,
    #[arg(long, help = "override registry URL")]
    pub registry: Option<String>,
    #[arg(long, help = "override token (env MGC_NPM_TOKEN recommended)")]
    pub token: Option<String>,
    #[arg(
        long,
        conflicts_with = "token",
        help = "publish with a short-lived CI OIDC token"
    )]
    pub trusted: bool,
    #[arg(
        long,
        requires = "trusted",
        help = "OIDC audience (defaults to the registry origin)"
    )]
    pub trusted_audience: Option<String>,
}

// Manual Debug redacts the credential-bearing fields so a future debug/panic
// format cannot leak a publish token or a 2FA one-time code.
// Debug thủ công che các trường mang credential để log/panic sau này không rò
// token publish hoặc mã OTP 2FA.
impl std::fmt::Debug for PublishArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn redacted(value: &Option<String>) -> &'static str {
            if value.is_some() {
                "[REDACTED]"
            } else {
                "None"
            }
        }
        f.debug_struct("PublishArgs")
            .field("protocol", &self.protocol)
            .field("package", &self.package)
            .field("version", &self.version)
            .field("artifacts", &self.artifacts)
            .field("image", &self.image)
            .field("tag", &self.tag)
            .field("access", &self.access)
            .field("dry_run", &self.dry_run)
            .field("json", &self.json)
            .field("otp", &redacted(&self.otp))
            .field("force", &self.force)
            .field("ignore_scripts", &self.ignore_scripts)
            .field("allow_scripts", &self.allow_scripts)
            .field("no_git_checks", &self.no_git_checks)
            .field("publish_branch", &self.publish_branch)
            .field("batch", &self.batch)
            .field("report_summary", &self.report_summary)
            .field("patch", &self.patch)
            .field("minor", &self.minor)
            .field("major", &self.major)
            .field("registry", &self.registry)
            .field("token", &redacted(&self.token))
            .field("trusted", &self.trusted)
            .field("trusted_audience", &self.trusted_audience)
            .finish()
    }
}

pub async fn run(args: PublishArgs, core: Option<&str>, recursive: bool) -> Result<()> {
    if args.protocol != "npm" {
        return publish_trusted_non_npm(&args, core).await;
    }
    let cwd = std::env::current_dir()?;

    if recursive {
        let project_root = ProjectConfig::find_project_root(&cwd)
            .ok_or_else(|| anyhow!("Could not find project root — run mgc init"))?;
        let mut workspaces =
            install::discover_workspace_projects(&project_root)?.ok_or_else(|| {
                anyhow!("--recursive requires magicore.workspace.toml (mode = \"monorepo\")")
            })?;
        workspaces = topo_sort_workspaces(workspaces)?;
        for ws in &workspaces {
            info(&format!("Publishing workspace: {}", ws.display()));
            publish_project(&args, ws).await?;
        }
        return Ok(());
    }
    publish_project(&args, &cwd).await
}

/// `mgc stage` — pack project vào .mgc-stage/ (không đăng registry).
/// Kiểm tra nhanh trước khi publish: tarball hợp lệ, files selection đúng.
pub async fn stage(dir: Option<String>) -> Result<()> {
    // Resolve --dir against the real cwd: downstream pack/tar joins
    // assume an absolute root, and a bare relative dir surfaces as a
    // cryptic empty-path IO error.
    // (--dir tương đối phải nối với cwd — pack/tar cần root tuyệt đối.)
    let cwd = match dir {
        Some(d) => {
            let p = PathBuf::from(&d);
            if p.is_absolute() {
                p
            } else {
                std::env::current_dir()?.join(p)
            }
        }
        None => std::env::current_dir()?,
    };
    let project_root =
        ProjectConfig::find_project_root(&cwd).ok_or_else(crate::error::project_root_missing)?;
    let project = ProjectConfig::load(&project_root)?.ok_or_else(crate::error::mgc_toml_missing)?;

    let entry_prefix = format!("{}-{}", project.name, project.version);
    let filename = format!(
        "{}-{}.tgz",
        project.name.replace(['/', '@'], "-"),
        project.version
    );
    let stage_dir = project_root.join(".mgc-stage");
    fs::create_dir_all(&stage_dir)?;
    let tarball_path = stage_dir.join(&filename);
    pack(&project_root, &tarball_path, &entry_prefix)?;

    let size = fs::metadata(&tarball_path).map(|m| m.len()).unwrap_or(0);
    mgc_ui::success(&format!(
        "staged {} ({:.1} KiB) → {}",
        filename,
        size as f64 / 1024.0,
        tarball_path.display()
    ));
    mgc_ui::info("next: `mgc publish` to upload, or `mgc publish --dry-run` to verify first");
    Ok(())
}

pub(super) const NPM_PROTOCOL: &str = "npm";
