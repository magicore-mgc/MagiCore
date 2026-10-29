//! Match a detected core to its existing dev implementation.
//! Ghép core đã nhận diện với implementation dev hiện có.

use std::path::Path;

use crate::commands::compat::CompatMode;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum DevRoute<'a> {
    Web,
    Game,
    Iot,
    Cloud,
    Cicd,
    App,
    Ai,
    Lib,
    Hardware,
    Unsupported(&'a str),
}

pub(super) fn route_for(core: &str) -> DevRoute<'_> {
    match core {
        "web" => DevRoute::Web,
        "game" => DevRoute::Game,
        "iot" => DevRoute::Iot,
        // The CLI core name is `clo`, the adapter reports `cloud`; `mgc dev --core clo`
        // sets the CLI name while an unchosen project detection reports the
        // adapter name. Accepting only one of them made the other unreachable.
        // (Tên CLI là `clo`, adapter báo `cloud`; `mgc dev --core clo` đặt tên CLI
        // còn detect tự động báo tên adapter. Chỉ nhận một tên từng làm tên kia
        // không route được.)
        "cloud" | "clo" => DevRoute::Cloud,
        "cicd" => DevRoute::Cicd,
        "app" => DevRoute::App,
        "ai" => DevRoute::Ai,
        "lib" | "library" => DevRoute::Lib,
        "hardware" => DevRoute::Hardware,
        other => DevRoute::Unsupported(other),
    }
}

pub(in crate::commands::dev::dev) async fn dispatch(
    core: &str,
    root: &Path,
    host: String,
    port: Option<u16>,
    clear: bool,
    compat: &CompatMode,
    compat_runner: Option<&str>,
) -> anyhow::Result<()> {
    match route_for(core) {
        #[cfg(feature = "web")]
        DevRoute::Web => {
            if clear {
                mgc_ui::info("--clear is delegated to the selected web framework when supported.");
            }
            // Port resolution qua bảng trung tâm dev_port (RULE §13 — hoán vị 4·3·1·5).
            // Port resolution goes through the centralized dev_port table.
            let port = crate::commands::core::dev_port::resolve_port("web", port);
            crate::commands::core::dev::web::dev_at_root(
                root,
                Some(host),
                port,
                compat,
                compat_runner,
            )
            .await
        }
        #[cfg(not(feature = "web"))]
        DevRoute::Web => Err(crate::error::core_not_in_build("web")),
        #[cfg(feature = "game")]
        DevRoute::Game => crate::commands::core::dev::game::dev(root).await,
        #[cfg(not(feature = "game"))]
        DevRoute::Game => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "iot")]
        DevRoute::Iot => crate::commands::core::dev::iot::dev_command::dev(root).await,
        #[cfg(not(feature = "iot"))]
        DevRoute::Iot => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "clo")]
        DevRoute::Cloud => crate::commands::core::dev::clo::dev(false).await,
        #[cfg(not(feature = "clo"))]
        DevRoute::Cloud => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "cicd")]
        DevRoute::Cicd => crate::commands::core::dev::cicd::dev(false).await,
        #[cfg(not(feature = "cicd"))]
        DevRoute::Cicd => Err(crate::error::core_not_in_build("cicd")),
        #[cfg(feature = "app")]
        DevRoute::App => crate::commands::core::dev::app::dev(false).await,
        #[cfg(not(feature = "app"))]
        DevRoute::App => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "ai")]
        DevRoute::Ai => crate::commands::core::dev::ai::dev(false).await,
        #[cfg(not(feature = "ai"))]
        DevRoute::Ai => Err(crate::error::core_not_in_build("ai")),
        // A library serves no process, so `dev` is the build-watch loop that
        // `dev_port` already documents for this core; it opens no port and must
        // not pretend to. `clear`/`host`/`port` have no meaning here, and
        // silently ignoring them would hide a user's mistaken expectation.
        // (Thư viện không serve process, nên `dev` là vòng build-watch mà
        // `dev_port` đã ghi; không mở port và không giả vờ có. `--clear`,
        // `--host`, `--port` vô nghĩa ở đây — bỏ qua im lặng sẽ che kỳ vọng sai.)
        #[cfg(feature = "lib")]
        DevRoute::Lib => {
            if clear {
                return Err(crate::error::dev_lib_flag_unsupported("--clear"));
            }
            if host != "localhost" {
                return Err(crate::error::dev_lib_flag_unsupported("--host"));
            }
            if port.is_some() {
                return Err(crate::error::dev_lib_flag_unsupported("--port"));
            }
            crate::commands::core::dev::lib::dev(false).await
        }
        #[cfg(not(feature = "lib"))]
        DevRoute::Lib => Err(crate::error::core_not_in_build("lib")),
        // Hardware has no program to run: the honest answer is "not applicable",
        // with the commands that do exist, not "not implemented yet".
        // (Hardware không có chương trình để chạy: trả lời trung thực là "không
        // áp dụng" kèm lệnh có thật, không phải "chưa implement".)
        DevRoute::Hardware => Err(crate::error::dev_hardware_not_applicable()),
        DevRoute::Unsupported(other) => Err(crate::error::dev_core_not_implemented(other)),
    }
}
