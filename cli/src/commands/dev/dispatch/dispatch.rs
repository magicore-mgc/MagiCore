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
    Unsupported(&'a str),
}

pub(super) fn route_for(core: &str) -> DevRoute<'_> {
    match core {
        "web" => DevRoute::Web,
        "game" => DevRoute::Game,
        "iot" => DevRoute::Iot,
        "cloud" => DevRoute::Cloud,
        "cicd" => DevRoute::Cicd,
        "app" => DevRoute::App,
        "ai" => DevRoute::Ai,
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
        DevRoute::Unsupported(other) => Err(crate::error::dev_core_not_implemented(other)),
    }
}
