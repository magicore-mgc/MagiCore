// CLI dispatch entry — apply global policy, select route and invoke the command.
// Entry này áp dụng policy toàn cục rồi gọi tuyến xử lý lệnh.
use super::audit_policy::reject_unsupported_audit_strict;
use super::dispatch_command::dispatch_command;
use super::identity::{explicit_dependency_core, validate_explicit_dependency_core};
use super::recursive::run_recursive;
use super::recursive_policy::{
    recursive_supported, reject_filter_without_recursive, reject_unsupported_filter,
    reject_unsupported_recursive,
};
use crate::Cli;
#[cfg(test)]
use crate::Commands;
use anyhow::Result;

pub async fn run(cli: Cli) -> Result<()> {
    mgc_ui::set_quiet(cli.quiet);
    let core = cli.core.as_deref();

    if let Some(dir) = cli.dir.as_deref() {
        std::env::set_current_dir(dir)
            .map_err(|e| crate::error::dir_missing(&dir.display().to_string(), e.to_string()))?;
    }

    if cli.recursive {
        if let Some(command) = cli.command.as_ref() {
            if !recursive_supported(command) {
                reject_unsupported_recursive(Some(command))?;
            }
            // Publish recursive là pipeline topo riêng — --filter chưa nối vào đó.
            if cli.filter.is_some() && super::command_name::is_publish(command) {
                reject_unsupported_filter(command)?;
            }
        }
    } else if cli.filter.is_some() {
        reject_filter_without_recursive()?;
    }

    if cli.audit_strict {
        if let Some(command) = cli.command.as_ref() {
            reject_unsupported_audit_strict(command)?;
        }
        // SAFETY: dispatch entry runs before any worker thread is spawned;
        // MGC_AUDIT_STRICT is read later but not concurrently mutated elsewhere.
        // AN TOÀN: ghi ở entry dispatch trước khi spawn worker thread.
        #[allow(unsafe_code)]
        unsafe {
            std::env::set_var("MGC_AUDIT_STRICT", "1");
        }
    }

    match cli.command {
        // Publish recursive là pipeline riêng (topo sort) — giữ path cũ,
        // chỉ bật khi user truyền --recursive.
        Some(command) if super::command_name::is_publish(&command) => {
            dispatch_command(command, core, cli.recursive).await
        }
        Some(command) if cli.recursive => run_recursive(command, core, cli.filter.as_deref()).await,
        Some(command) => {
            if let Some(command_core) = explicit_dependency_core(&command) {
                let cwd = std::env::current_dir()?;
                let root =
                    mgc_config::project::ProjectConfig::find_project_root(&cwd).unwrap_or(cwd);
                validate_explicit_dependency_core(&root, command_core, core)?;
            }
            dispatch_command(command, core, false).await
        }
        None => {
            let cores = crate::factory::available_cores();
            mgc_ui::help::print_custom_help(&cores);
            Ok(())
        }
    }
}

#[cfg(test)]
#[path = "../../test/engine_test.rs"]
mod tests;
