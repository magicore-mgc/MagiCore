// Shared remove contract — construct the selected core command without policy decisions.
// Hợp đồng remove dùng chung — dựng lệnh core đã chọn mà không quyết định policy.
use crate::dispatch::types::CoreCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RemoveTarget {
    Web,
    Game,
    Ai,
    Clo,
    Cicd,
    Iot,
    App,
    Lib,
    Hardware,
}

impl RemoveTarget {
    pub(crate) fn from_core_name(core: &str) -> Option<Self> {
        match core {
            "web" => Some(Self::Web),
            "game" => Some(Self::Game),
            "ai" => Some(Self::Ai),
            "clo" => Some(Self::Clo),
            "cicd" => Some(Self::Cicd),
            "iot" => Some(Self::Iot),
            "app" => Some(Self::App),
            "lib" => Some(Self::Lib),
            "hardware" => Some(Self::Hardware),
            _ => None,
        }
    }
}

pub(crate) struct RemoveRequest {
    pub(crate) packages: Vec<String>,
    pub(crate) install_after_remove: bool,
    pub(crate) compat_runtime: Option<String>,
}

pub(crate) fn to_core_command(target: RemoveTarget, request: RemoveRequest) -> CoreCommand {
    match target {
        RemoveTarget::Web => CoreCommand::RemoveWeb {
            packages: request.packages,
            install: request.install_after_remove,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::Game => CoreCommand::RemoveGame {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::Ai => CoreCommand::RemoveAi {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::Clo => CoreCommand::RemoveClo {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::Cicd => CoreCommand::RemoveCicd {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::Iot => CoreCommand::RemoveIot {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::App => CoreCommand::RemoveApp {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::Lib => CoreCommand::RemoveLib {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
        RemoveTarget::Hardware => CoreCommand::RemoveHardware {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
    }
}

#[cfg(test)]
#[path = "test/remove_mapping.rs"]
mod tests;
