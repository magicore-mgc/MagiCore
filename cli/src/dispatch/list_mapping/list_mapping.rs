// Shared list contract — construct the matching core command without policy decisions.
// Hợp đồng list dùng chung — dựng command core tương ứng mà không quyết định policy.
use crate::dispatch::types::CoreCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ListTarget {
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

impl ListTarget {
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

pub(crate) struct ListRequest {
    pub(crate) compat_runtime: Option<String>,
}

pub(crate) fn to_core_command(target: ListTarget, request: ListRequest) -> CoreCommand {
    match target {
        ListTarget::Web => CoreCommand::ListWeb {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::Game => CoreCommand::ListGame {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::Ai => CoreCommand::ListAi {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::Clo => CoreCommand::ListClo {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::Cicd => CoreCommand::ListCicd {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::Iot => CoreCommand::ListIot {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::App => CoreCommand::ListApp {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::Lib => CoreCommand::ListLib {
            compat_runtime: request.compat_runtime,
        },
        ListTarget::Hardware => CoreCommand::ListHardware {
            compat_runtime: request.compat_runtime,
        },
    }
}

#[cfg(test)]
#[path = "test/list_mapping.rs"]
mod tests;
