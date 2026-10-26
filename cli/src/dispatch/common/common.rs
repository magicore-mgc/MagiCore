//! Shared CLI command router — preserves the exhaustive common-command route table.
//! Bộ định tuyến lệnh CLI dùng chung — giữ match exhaustive cho mọi common command.

use anyhow::Result;

use crate::commands;
use crate::context::ProjectContext;

use crate::dispatch::types::CommonCommand;

pub async fn dispatch_common(
    command: CommonCommand,
    core: Option<&str>,
    recursive: bool,
) -> Result<()> {
    match command {
        CommonCommand::Init {
            template,
            signature,
        } => commands::init::run(template, signature).await,
        CommonCommand::Config { cmd, local } => commands::config::run(cmd, local).await,
        CommonCommand::Stage { dir } => {
            commands::publish::stage(dir.map(|d| d.display().to_string())).await
        }
        CommonCommand::Import {
            dir,
            allow_unsigned,
        } => commands::import::run(dir, allow_unsigned).await,
        CommonCommand::Migrate { cmd } => commands::migrate::run(cmd).await,
        CommonCommand::Sbom {
            format,
            output,
            name,
            version,
            dir,
        } => commands::sbom::run(format, output, name, version, dir).await,
        CommonCommand::Dev {
            host,
            port,
            clear,
            compat_runtime,
            compat_runner,
        } => super::dev_route::dev(core, host, port, clear, compat_runtime, compat_runner).await,
        CommonCommand::Build {
            target,
            compat_runtime,
            compat_runner,
            engine,
            allow_output_warnings,
        } => {
            super::dev_route::build(
                core,
                target,
                compat_runtime,
                compat_runner,
                engine,
                allow_output_warnings,
            )
            .await
        }
        CommonCommand::Flash { board, skip_build } => {
            super::dev_route::flash(board, skip_build).await
        }
        CommonCommand::Deploy { run } => super::deploy_command::dispatch(run).await,
        #[cfg(feature = "cicd")]
        CommonCommand::CiGenerate => commands::core::dev::cicd::ci_generate(),
        #[cfg(not(feature = "cicd"))]
        CommonCommand::CiGenerate => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "cicd")]
        CommonCommand::Verify => commands::core::dev::cicd::verify().await,
        #[cfg(not(feature = "cicd"))]
        CommonCommand::Verify => Err(crate::error::core_not_in_build("cicd")),
        CommonCommand::Start => commands::start::run(core).await,
        CommonCommand::Exec {
            command,
            args,
            compat_runner,
        } => commands::exec::run(core, command, args, compat_runner.as_deref()),
        CommonCommand::Adopt { core, path } => {
            let root = match path {
                Some(root) => root,
                None => std::env::current_dir()?,
            };
            commands::adopt::run(&core, &root)
        }
        CommonCommand::Version { json } => commands::version::run(json),
        CommonCommand::Info { package, json } => commands::info::run(package, json).await,
        CommonCommand::Search {
            query,
            json,
            exact,
            page,
        } => commands::search::run(query, json, exact, page).await,
        CommonCommand::Outdated { json } => commands::outdated::run(core, json).await,
        CommonCommand::Audit { cmd, fix, format } => {
            super::audit_command::dispatch(cmd, fix, format, core).await
        }
        CommonCommand::SelfUpdate {
            version,
            variant,
            dry_run,
            trust_root,
            allow_unsigned,
        } => {
            commands::self_update::run(version, variant, dry_run, trust_root, allow_unsigned).await
        }
        CommonCommand::SignRelease { manifest, key_hex } => {
            commands::sign_release::run(manifest, key_hex)
        }
        CommonCommand::Run {
            script,
            args,
            compat_runtime,
            compat_runner,
        } => super::dev_route::run(script, args, core, compat_runtime, compat_runner).await,
        CommonCommand::Test {
            args,
            compat_runtime,
            compat_runner,
        } => super::dev_route::test(args, core, compat_runtime, compat_runner).await,
        CommonCommand::Optimizer { force } => commands::optimizer::run(core, force).await,
        CommonCommand::Dlx {
            package,
            bin,
            allow_scripts,
            args,
        } => commands::dlx::run(package, bin, allow_scripts, args).await,
        CommonCommand::Tool { cmd } => commands::tool::run(cmd),
        CommonCommand::Cache {
            action,
            target,
            yes,
            dry_run,
            ci,
        } => commands::cache::run(action, target, yes, dry_run, ci, core).await,
        #[cfg(feature = "web")]
        CommonCommand::Link { package } => {
            let ctx = ProjectContext::load_with_core(core)?;
            commands::core::web::link(ctx.adapter(), ctx.root(), package.as_deref()).await
        }
        #[cfg(not(feature = "web"))]
        CommonCommand::Link { .. } => Err(crate::error::core_not_in_build("web")),
        #[cfg(feature = "web")]
        CommonCommand::Unlink { package } => {
            let ctx = ProjectContext::load_with_core(core)?;
            commands::core::web::unlink(ctx.adapter(), ctx.root(), package.as_deref()).await
        }
        #[cfg(not(feature = "web"))]
        CommonCommand::Unlink { .. } => Err(crate::error::core_not_in_build("web")),
        #[cfg(feature = "web")]
        CommonCommand::Why { package } => {
            let ctx = ProjectContext::load_with_core(core)?;
            commands::core::web::why(ctx.adapter(), ctx.root(), &package).await
        }
        #[cfg(not(feature = "web"))]
        CommonCommand::Why { .. } => Err(crate::error::core_not_in_build("web")),
        CommonCommand::Login {
            registry,
            username,
            password_stdin,
            local,
        } => {
            commands::login::run(crate::commands::login::LoginArgs {
                registry,
                username,
                password_stdin,
                local,
            })
            .await
        }
        CommonCommand::Registry { cmd } => {
            commands::registry::run(crate::commands::registry::RegistryArgs { cmd }, core).await
        }
        CommonCommand::Model { cmd } => {
            commands::model::run(crate::commands::model::ModelArgs { cmd }).await
        }
        CommonCommand::Publish {
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
        } => {
            super::publish_route::dispatch(
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
                core,
                recursive,
            )
            .await
        }
        CommonCommand::Patch { cmd } => {
            commands::patch::run(crate::commands::patch::PatchArgs { cmd }).await
        }
        CommonCommand::Dedupe {
            dry_run,
            prefer_latest,
            json,
        } => {
            commands::dedupe::run(crate::commands::dedupe::DedupeArgs {
                dry_run,
                prefer_latest,
                json,
            })
            .await
        }
        CommonCommand::Store { cmd } => commands::store::run(cmd).await,
        CommonCommand::Bench { args } => commands::bench::handle(args).await,
        CommonCommand::Trust { cmd } => commands::trust::run(cmd).await,
        CommonCommand::Hooks { cmd } => commands::hooks::handle(cmd),
        CommonCommand::Docs { output } => commands::docs::handle(output),
        CommonCommand::Completion { shell } => commands::completion::handle(shell),
        CommonCommand::Telemetry { cmd } => commands::telemetry::handle(cmd),
        CommonCommand::Network { cmd } => commands::network::handle(cmd),
        CommonCommand::Doctor { cmd } => commands::doctor::handle(cmd),
        CommonCommand::Workspace { cmd } => commands::workspace::run(cmd).await,
        CommonCommand::Mcp => commands::mcp::run().await,
        CommonCommand::Capabilities => commands::capabilities::run(core),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
