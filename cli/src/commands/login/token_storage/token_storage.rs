//! Select the npmrc target and persist the scoped registry token.
//! Chọn đích npmrc và lưu token registry theo scope.

use std::path::PathBuf;

use mgc_config::npmrc::NpmRc;

use super::error::TokenStorageError;

pub(in crate::commands::login) fn store_registry_token(
    local: bool,
    registry: &str,
    token: &str,
) -> Result<PathBuf, TokenStorageError> {
    let npmrc_path = if local {
        project_npmrc_path(std::env::current_dir()?)
    } else {
        user_npmrc_path(dirs::home_dir())?
    };
    NpmRc::save_auth_token_for_registry_url(&npmrc_path, registry, token)
        .map_err(TokenStorageError::Persistence)?;
    Ok(npmrc_path)
}

fn project_npmrc_path(current_dir: PathBuf) -> PathBuf {
    current_dir.join(".npmrc")
}

fn user_npmrc_path(home_dir: Option<PathBuf>) -> Result<PathBuf, TokenStorageError> {
    home_dir
        .map(|path| path.join(".npmrc"))
        .ok_or(TokenStorageError::NoHomeDirectory)
}

#[cfg(test)]
#[path = "test/token_storage.rs"]
mod tests;
