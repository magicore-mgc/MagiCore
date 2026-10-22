// Shared add contract — normalize CLI options and construct one core command.
// Hợp đồng add dùng chung — chuẩn hóa tùy chọn và dựng đúng command core.
use crate::dispatch::types::CoreCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AddTarget {
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

impl AddTarget {
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

// Union only at the adapter boundary — target arms project their supported fields.
// Tập hợp chỉ ở ranh giới adapter — mỗi target chỉ chuyển trường được hỗ trợ.
#[derive(Default)]
pub(crate) struct AddRequest {
    pub(crate) packages: Vec<String>,
    pub(crate) dev: bool,
    pub(crate) exact: bool,
    pub(crate) optional: bool,
    pub(crate) peer: bool,
    pub(crate) no_save: bool,
    pub(crate) install: bool,
    pub(crate) global: bool,
    pub(crate) compat_runtime: Option<String>,
    pub(crate) version: Option<String>,
}

pub(crate) fn to_core_command(target: AddTarget, request: AddRequest) -> CoreCommand {
    match target {
        AddTarget::Web => CoreCommand::AddWeb {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            install: request.install,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::Game => CoreCommand::AddGame {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::Ai => CoreCommand::AddAi {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::Clo => CoreCommand::AddClo {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::Cicd => CoreCommand::AddCicd {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::Iot => CoreCommand::AddIot {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::App => CoreCommand::AddApp {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::Lib => CoreCommand::AddLib {
            packages: request.packages,
            dev: request.dev,
            exact: request.exact,
            optional: request.optional,
            peer: request.peer,
            no_save: request.no_save,
            global: request.global,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
        AddTarget::Hardware => CoreCommand::AddHardware {
            packages: request.packages,
            compat_runtime: request.compat_runtime,
            version: request.version,
        },
    }
}

#[cfg(test)]
#[path = "test/add_mapping.rs"]
mod tests;
