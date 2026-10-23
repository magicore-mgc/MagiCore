//! Publish route owner: maps `CommonCommand::Publish` into `PublishArgs`.
//! Owner route publish: map `CommonCommand::Publish` sang `PublishArgs`.

use anyhow::Result;

use crate::commands;

#[allow(clippy::too_many_arguments)]
pub(crate) async fn dispatch(
    protocol: String,
    package: Option<String>,
    version: Option<String>,
    artifacts: Vec<std::path::PathBuf>,
    image: Option<String>,
    tag: Option<String>,
    access: Option<String>,
    dry_run: bool,
    json: bool,
    otp: Option<String>,
    force: bool,
    ignore_scripts: bool,
    allow_scripts: bool,
    no_git_checks: bool,
    publish_branch: Option<String>,
    batch: bool,
    report_summary: bool,
    patch: bool,
    minor: bool,
    major: bool,
    registry: Option<String>,
    token: Option<String>,
    trusted: bool,
    trusted_audience: Option<String>,
    core: Option<&str>,
    recursive: bool,
) -> Result<()> {
    commands::publish::run(
        crate::commands::publish::PublishArgs {
            protocol,
            package,
            version,
            artifacts,
            image,
            tag,
            access,
            dry_run,
            json,
            otp,
            force,
            ignore_scripts,
            allow_scripts,
            no_git_checks,
            publish_branch,
            batch,
            report_summary,
            patch,
            minor,
            major,
            registry,
            token,
            trusted,
            trusted_audience,
        },
        core,
        recursive,
    )
    .await
}
