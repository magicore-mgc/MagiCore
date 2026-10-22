// Shared install contract — normalize CLI options and construct one core command.
// Hợp đồng install dùng chung — chuẩn hóa tùy chọn và dựng đúng command core.
use crate::dispatch::types::CoreCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum InstallTarget {
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

impl InstallTarget {
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

// Union only at the adapter boundary — each target projects its supported fields.
// Tập hợp chỉ ở ranh giới adapter — mỗi target chỉ chuyển các trường được hỗ trợ.
#[derive(Default)]
pub(crate) struct InstallRequest {
    pub(crate) packages: Vec<String>,
    pub(crate) dry_run: bool,
    pub(crate) frozen: bool,
    pub(crate) ignore_scripts: bool,
    pub(crate) allow_scripts: bool,
    pub(crate) prefer_dedupe: bool,
    pub(crate) repair: bool,
    pub(crate) offline: bool,
    pub(crate) compat_runtime: Option<String>,
}

pub(crate) fn to_core_command(target: InstallTarget, request: InstallRequest) -> CoreCommand {
    match target {
        InstallTarget::Web => CoreCommand::InstallWeb {
            packages: request.packages,
            frozen: request.frozen,
            ignore_scripts: request.ignore_scripts,
            allow_scripts: request.allow_scripts,
            prefer_dedupe: request.prefer_dedupe,
            repair: request.repair,
            offline: request.offline,
            compat_runtime: request.compat_runtime,
        },
        InstallTarget::Game => CoreCommand::InstallGame {
            packages: request.packages,
            frozen: request.frozen,
            compat_runtime: request.compat_runtime,
        },
        InstallTarget::Ai => CoreCommand::InstallAi {
            packages: request.packages,
            dry_run: request.dry_run,
            compat_runtime: request.compat_runtime,
            frozen: request.frozen,
            offline: request.offline,
        },
        InstallTarget::Clo => CoreCommand::InstallClo {
            packages: request.packages,
            dry_run: request.dry_run,
            frozen: request.frozen,
            compat_runtime: request.compat_runtime,
        },
        InstallTarget::Cicd => CoreCommand::InstallCicd {
            packages: request.packages,
            dry_run: request.dry_run,
            compat_runtime: request.compat_runtime,
        },
        InstallTarget::Iot => CoreCommand::InstallIot {
            packages: request.packages,
            frozen: request.frozen,
            compat_runtime: request.compat_runtime,
        },
        InstallTarget::App => CoreCommand::InstallApp {
            packages: request.packages,
            dry_run: request.dry_run,
            compat_runtime: request.compat_runtime,
            frozen: request.frozen,
            offline: request.offline,
        },
        InstallTarget::Lib => CoreCommand::InstallLib {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
            frozen: request.frozen,
            offline: request.offline,
        },
        InstallTarget::Hardware => CoreCommand::InstallHardware {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
        },
    }
}

#[cfg(test)]
#[path = "test/install_mapping.rs"]
mod tests;
