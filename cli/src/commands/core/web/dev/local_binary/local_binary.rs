// Resolve project-local binaries and PATH — phân giải binary cục bộ và PATH.
// Project-local executable resolution.
use super::*;
use crate::commands::core::web::implementation::project_context::project_root::install_hint_command;

pub(in crate::commands::core::web::implementation) fn resolve_local_bin(
    project_root: &Path,
    bin_name: &str,
) -> Result<PathBuf> {
    let bin_dir = project_root.join("node_modules").join(".bin");
    let candidates = local_bin_candidates(&bin_dir, bin_name);
    if let Some(path) = candidates.into_iter().find(|path| path.exists()) {
        return Ok(path);
    }

    Err(crate::error::web_missing_executable(
        bin_name,
        install_hint_command(),
        project_root,
    ))
}
pub(super) fn local_bin_candidates(bin_dir: &Path, bin_name: &str) -> Vec<PathBuf> {
    #[cfg(not(windows))]
    {
        vec![bin_dir.join(bin_name)]
    }
    #[cfg(windows)]
    {
        let mut candidates = vec![bin_dir.join(bin_name)];
        candidates.push(bin_dir.join(format!("{bin_name}.cmd")));
        candidates.push(bin_dir.join(format!("{bin_name}.exe")));
        candidates
    }
}
pub(in crate::commands::core::web::implementation) fn prepend_path(
    local_bin: &Path,
) -> Result<OsString> {
    let mut paths = vec![local_bin.to_path_buf()];
    if let Some(existing) = std::env::var_os("PATH") {
        paths.extend(std::env::split_paths(&existing));
    }
    Ok(std::env::join_paths(paths)?)
}
