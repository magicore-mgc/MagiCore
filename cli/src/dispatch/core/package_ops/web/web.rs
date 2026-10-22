use anyhow::{Result, anyhow};

use crate::commands;
use crate::dispatch::types::CoreCommand;

pub(in crate::dispatch) fn matches(command: &CoreCommand) -> bool {
    matches!(
        command,
        CoreCommand::AddWeb { .. }
            | CoreCommand::RemoveWeb { .. }
            | CoreCommand::ListWeb { .. }
            | CoreCommand::UpdateWeb { .. }
    )
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match command {
        CoreCommand::AddWeb {
            packages,
            dev,
            exact,
            optional,
            peer,
            no_save,
            install,
            global,
            compat_runtime,
            version,
        } => {
            commands::core::add::web::add(
                packages,
                version,
                dev,
                exact,
                optional,
                peer,
                no_save,
                install,
                global,
                compat_runtime,
            )
            .await
        }
        CoreCommand::RemoveWeb {
            packages,
            install,
            compat_runtime,
        } => commands::core::remove::web::remove(packages, install, compat_runtime).await,
        CoreCommand::ListWeb { compat_runtime } => {
            commands::core::list::web::list(compat_runtime).await
        }
        CoreCommand::UpdateWeb {
            packages,
            install,
            compat_runtime,
        } => commands::core::update::web::update(packages, install, compat_runtime).await,
        _ => Err(anyhow!(
            "package dispatcher routed a command to the wrong core owner (expected web)"
        )),
    }
}
