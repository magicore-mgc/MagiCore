// Common CLI adapter — map shared commands into the dispatch contract.
// Adapter CLI common — ánh xạ lệnh dùng chung sang hợp đồng dispatch.
use crate::commands::definitions::commands::common::CommonCommands;
use crate::dispatch::types::CommonCommand;

pub(in crate::dispatch) fn common_to_dispatch(command: CommonCommands) -> Option<CommonCommand> {
    match command {
        CommonCommands::Init {
            template,
            signature,
        } => Some(CommonCommand::Init {
            template,
            signature,
        }),
        CommonCommands::Info { package, json } => Some(CommonCommand::Info { package, json }),
        CommonCommands::Search {
            query,
            json,
            exact,
            page,
        } => Some(CommonCommand::Search {
            query,
            json,
            exact,
            page,
        }),
        CommonCommands::Outdated { json } => Some(CommonCommand::Outdated { json }),
        CommonCommands::Audit { cmd, fix, format } => {
            Some(CommonCommand::Audit { cmd, fix, format })
        }
        CommonCommands::SelfUpdate {
            version,
            variant,
            dry_run,
            trust_root,
            allow_unsigned,
        } => Some(CommonCommand::SelfUpdate {
            version,
            variant,
            dry_run,
            trust_root,
            allow_unsigned,
        }),
        CommonCommands::SignRelease { manifest, key_hex } => {
            Some(CommonCommand::SignRelease { manifest, key_hex })
        }
        CommonCommands::Config { cmd, local } => Some(CommonCommand::Config { cmd, local }),
        CommonCommands::Stage { dir } => Some(CommonCommand::Stage { dir }),
        CommonCommands::Import {
            dir,
            allow_unsigned,
        } => Some(CommonCommand::Import {
            dir,
            allow_unsigned,
        }),
        CommonCommands::Migrate { cmd } => Some(CommonCommand::Migrate { cmd }),
        CommonCommands::Sbom {
            format,
            output,
            name,
            version,
            dir,
        } => Some(CommonCommand::Sbom {
            format,
            output,
            name,
            version,
            dir,
        }),
        CommonCommands::Publish {
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
        } => Some(CommonCommand::Publish {
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
        }),
        CommonCommands::Patch { cmd } => Some(CommonCommand::Patch { cmd }),
        CommonCommands::Dedupe {
            dry_run,
            prefer_latest,
            json,
        } => Some(CommonCommand::Dedupe {
            dry_run,
            prefer_latest,
            json,
        }),
        CommonCommands::Login {
            registry,
            username,
            password_stdin,
            local,
        } => Some(CommonCommand::Login {
            registry,
            username,
            password_stdin,
            local,
        }),
        CommonCommands::Registry { cmd } => Some(CommonCommand::Registry { cmd }),
        CommonCommands::Model { cmd } => Some(CommonCommand::Model { cmd }),
        CommonCommands::Mcp => Some(CommonCommand::Mcp),
        // Capabilities has one canonical format (JSON); both flag spellings
        // intentionally map to the same dispatch variant.
        // Capabilities chỉ có một định dạng chuẩn (JSON); hai giá trị flag
        // được ánh xạ có chủ ý về cùng một variant dispatch.
        CommonCommands::Capabilities { json: _ } => Some(CommonCommand::Capabilities),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
