//! Keyring destination containment — Kiểm tra containment đích keyring.
//! Compare canonical home and destination — So sánh home và đích sau canonicalize.

use crate::CryptoResult;
use crate::keyring::error::KeyringError;
use std::ffi::OsString;
use std::path::{Component, Path, PathBuf};

pub(in crate::keyring) fn validate_keyring_path(path: &Path) -> CryptoResult<()> {
    if let Some(home) = dirs::home_dir() {
        validate_keyring_path_under(path, &home)?;
    }
    Ok(())
}

pub(super) fn validate_keyring_path_under(path: &Path, home: &Path) -> CryptoResult<()> {
    let canonical = canonicalize_with_missing_tail(path).ok_or(KeyringError::PathOutsideHome)?;
    let canonical_home =
        canonicalize_with_missing_tail(home).ok_or(KeyringError::HomeDirectoryMissing)?;
    if !canonical.starts_with(&canonical_home) {
        return Err(KeyringError::PathOutsideHome.into());
    }
    Ok(())
}

fn canonicalize_with_missing_tail(path: &Path) -> Option<PathBuf> {
    let mut current = path;
    let mut missing = Vec::<OsString>::new();
    loop {
        if let Ok(mut canonical) = current.canonicalize() {
            for component in missing.iter().rev() {
                canonical.push(component);
            }
            return Some(canonical);
        }
        match current.components().next_back()? {
            Component::Normal(name) => missing.push(name.to_os_string()),
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => return None,
        }
        current = current.parent()?;
        if current.as_os_str().is_empty() {
            current = Path::new(".");
        }
    }
}
