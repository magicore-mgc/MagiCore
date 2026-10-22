// Shared update contract — construct a selected core command without policy decisions.
// Hợp đồng update dùng chung — dựng lệnh core đã chọn mà không quyết định policy.
use crate::dispatch::types::CoreCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UpdateTarget {
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

impl UpdateTarget {
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

pub(crate) struct UpdateRequest {
    pub(crate) packages: Vec<String>,
    pub(crate) install: bool,
    pub(crate) compat_runtime: Option<String>,
}

pub(crate) fn to_core_command(target: UpdateTarget, request: UpdateRequest) -> CoreCommand {
    match target {
        UpdateTarget::Web => CoreCommand::UpdateWeb {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::Game => CoreCommand::UpdateGame {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::Ai => CoreCommand::UpdateAi {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::Clo => CoreCommand::UpdateClo {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::Cicd => CoreCommand::UpdateCicd {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::Iot => CoreCommand::UpdateIot {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::App => CoreCommand::UpdateApp {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::Lib => CoreCommand::UpdateLib {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
        UpdateTarget::Hardware => CoreCommand::UpdateHardware {
            packages: request.packages,
            install: request.install,
            compat_runtime: request.compat_runtime,
        },
    }
}

#[cfg(test)]
#[path = "test/update_mapping.rs"]
mod tests;
