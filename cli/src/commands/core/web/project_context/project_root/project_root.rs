// Web project root and install hint — phân giải root và lệnh cài.
// Web project-root and install-command hints.
use super::*;

pub(in crate::commands::core::web::implementation) fn install_hint_command() -> &'static str {
    #[cfg(all(
        feature = "web",
        not(any(
            feature = "game",
            feature = "ai",
            feature = "clo",
            feature = "cicd",
            feature = "iot",
            feature = "app",
            feature = "lib"
        ))
    ))]
    {
        return "mgc install";
    }

    #[cfg(all(
        feature = "web",
        any(
            feature = "game",
            feature = "ai",
            feature = "clo",
            feature = "cicd",
            feature = "iot",
            feature = "app",
            feature = "lib"
        )
    ))]
    {
        return "mgc install-web";
    }

    #[allow(unreachable_code)]
    "mgc install"
}
/// Find project root for Web commands.
/// Tìm thư mục gốc project cho lệnh Web.
pub(in crate::commands::core::web::implementation) fn project_root() -> Result<std::path::PathBuf> {
    let started_at = std::time::Instant::now();
    let cwd = std::env::current_dir().map_err(|e| crate::error::cwd_deleted(&e))?;
    let root = shared::find_project_root(&cwd)?
        .ok_or_else(|| crate::error::no_mgc_project_found("web"))?;
    shared::ensure_project_core_identity(&root, "web")?;
    web_command_profile_mark("project_root", started_at);
    Ok(root)
}
