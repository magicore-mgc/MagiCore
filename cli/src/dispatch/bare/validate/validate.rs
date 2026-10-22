// Bare command preflight — reject unsupported global, dry-run and offline requests.
// Bộ tiền kiểm từ chối global, dry-run và offline khi core chưa hỗ trợ.
use crate::commands::definitions::commands::BareCommands;
use crate::dispatch::bare::error::BareRouteError;

pub(in crate::dispatch::bare) fn validate_bare_request(
    command: &BareCommands,
    ecosystem: Option<&str>,
) -> anyhow::Result<()> {
    if matches!(command, BareCommands::Add { global: true, .. }) {
        return Err(BareRouteError::GlobalInstallUnsupported.into_error());
    }

    if matches!(command, BareCommands::Install { dry_run: true, .. })
        && !matches!(ecosystem, Some("ai" | "app" | "clo"))
    {
        return Err(BareRouteError::InstallDryRunUnsupported {
            core: ecosystem.unwrap_or("unknown").to_string(),
        }
        .into_error());
    }

    if matches!(command, BareCommands::Install { offline: true, .. })
        && !matches!(ecosystem, Some("web" | "lib" | "ai" | "app"))
    {
        return Err(BareRouteError::OfflineInstallUnsupported {
            core: ecosystem.unwrap_or("unknown").to_string(),
        }
        .into_error());
    }

    Ok(())
}
