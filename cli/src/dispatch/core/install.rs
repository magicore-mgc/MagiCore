use anyhow::Result;

use crate::commands;

use super::super::types::CoreCommand;

pub fn matches(command: &CoreCommand) -> bool {
    if matches!(command, CoreCommand::InstallWeb { .. }) {
        return cfg!(feature = "web");
    }
    matches!(
        command,
        CoreCommand::InstallWeb { .. }
            | CoreCommand::InstallGame { .. }
            | CoreCommand::InstallAi { .. }
            | CoreCommand::InstallClo { .. }
            | CoreCommand::InstallCicd { .. }
            | CoreCommand::InstallIot { .. }
            | CoreCommand::InstallApp { .. }
            | CoreCommand::InstallLib { .. }
            | CoreCommand::InstallLibBinary { .. }
            | CoreCommand::InstallHardware { .. }
    )
}

pub async fn dispatch(command: CoreCommand) -> Result<()> {
    if let CoreCommand::InstallWeb { .. } = command {
        #[cfg(not(feature = "web"))]
        return Err(crate::error::core_not_in_build("web"));
    }
    match command {
        #[cfg(feature = "web")]
        CoreCommand::InstallWeb {
            packages,
            frozen,
            ignore_scripts,
            allow_scripts,
            prefer_dedupe,
            repair,
            offline,
            compat_runtime,
        } => {
            commands::core::install::web::install(
                packages,
                frozen,
                ignore_scripts,
                allow_scripts,
                prefer_dedupe,
                repair,
                offline,
                compat_runtime,
            )
            .await
        }
        #[cfg(feature = "game")]
        CoreCommand::InstallGame {
            packages,
            frozen,
            compat_runtime,
        } => commands::core::install::game::install(packages, frozen, compat_runtime).await,
        #[cfg(not(feature = "game"))]
        CoreCommand::InstallGame { .. } => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "ai")]
        CoreCommand::InstallAi {
            packages,
            dry_run,
            compat_runtime,
            frozen,
            offline,
        } => {
            commands::core::install::ai::install(packages, dry_run, compat_runtime, frozen, offline)
                .await
        }
        #[cfg(not(feature = "ai"))]
        CoreCommand::InstallAi { .. } => Err(crate::error::core_not_in_build("ai")),
        #[cfg(feature = "clo")]
        CoreCommand::InstallClo {
            packages,
            dry_run,
            frozen,
            compat_runtime,
        } => commands::core::install::clo::install(packages, dry_run, frozen, compat_runtime).await,
        #[cfg(not(feature = "clo"))]
        CoreCommand::InstallClo { .. } => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "cicd")]
        CoreCommand::InstallCicd {
            packages,
            dry_run,
            compat_runtime,
        } => commands::core::install::cicd::install(packages, dry_run, compat_runtime).await,
        #[cfg(not(feature = "cicd"))]
        CoreCommand::InstallCicd { .. } => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "iot")]
        CoreCommand::InstallIot {
            packages,
            frozen,
            compat_runtime,
        } => commands::core::install::iot::install(packages, frozen, compat_runtime).await,
        #[cfg(not(feature = "iot"))]
        CoreCommand::InstallIot { .. } => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "app")]
        CoreCommand::InstallApp {
            packages,
            dry_run,
            compat_runtime,
            frozen,
            offline,
        } => {
            commands::core::install::app::install(
                packages,
                dry_run,
                compat_runtime,
                frozen,
                offline,
            )
            .await
        }
        #[cfg(not(feature = "app"))]
        CoreCommand::InstallApp { .. } => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "lib")]
        CoreCommand::InstallLibBinary {
            target,
            manifest,
            sign_key,
            trust_key,
        } => run_install_lib_binary(target, manifest, sign_key, trust_key).await,
        #[cfg(not(feature = "lib"))]
        CoreCommand::InstallLibBinary { .. } => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "lib")]
        CoreCommand::InstallLib {
            packages,
            compat_runtime,
            frozen,
            offline,
        } => {
            commands::core::install::library::install(packages, compat_runtime, frozen, offline)
                .await
        }
        #[cfg(not(feature = "lib"))]
        CoreCommand::InstallLib { .. } => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "hardware")]
        CoreCommand::InstallHardware {
            packages,
            compat_runtime,
        } => commands::core::install::hardware::install(packages, compat_runtime).await,
        #[cfg(not(feature = "hardware"))]
        CoreCommand::InstallHardware { .. } => Err(crate::error::core_not_in_build("hardware")),
        _ => unreachable!("non-install command routed to install dispatcher"),
    }
}

/// Install a signed binary into the lib keg store and report the result.
/// Cài binary có chữ ký vào keg store của lib và báo kết quả.
#[cfg(feature = "lib")]
async fn run_install_lib_binary(
    target: String,
    manifest: Option<String>,
    sign_key: Option<String>,
    trust_key: Vec<String>,
) -> Result<()> {
    let request = commands::core::install::library::InstallLibRequest {
        target: &target,
        manifest: manifest.as_deref().map(std::path::Path::new),
        key_id: sign_key.as_deref(),
        trust_keys: trust_key,
    };
    let outcome = commands::core::install::library::install_lib(request).await?;
    let verb = if outcome.reused {
        "Re-activated"
    } else {
        "Installed"
    };
    println!(
        "✓ {verb} {}@{} from {}",
        outcome.name,
        outcome.version,
        outcome.keg_dir.display()
    );
    Ok(())
}
