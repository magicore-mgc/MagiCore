use super::super::{project_cargo_lock, rust_packages, rust_root_ids, validate_build_ownership};
use crate::install::cargo_compat::verify_source_checksum;
use crate::install::rust_native_install::require_cargo_source_marker;
use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

/// Validate a Cargo build owned by the requested MagiCore core.
/// Xác minh Cargo build thuộc core MagiCore được yêu cầu.
pub(in crate::install) fn validate_build_for_core_impl(
    project_root: &Path,
    expected_core: &str,
) -> MgResult<Option<PathBuf>> {
    if !matches!(expected_core, "lib" | "iot" | "web") {
        return Err(MgError::Other(format!(
            "Cargo managed build is unsupported for core '{expected_core}'"
        )));
    }
    let lock = crate::install::read_existing_lock(project_root)?;
    let rust_packages = rust_packages(&lock)?;
    validate_build_ownership(&rust_packages, expected_core)?;
    let root_ids = rust_root_ids(project_root, &lock)?;
    let expected = if rust_packages.is_empty() {
        match crate::install::prepare_empty_cargo_lock(project_root, &[])? {
            // `None` means the existing provider lock is byte-identical to
            // the validated empty projection, so no managed registry source
            // is needed for this dependency-free project.
            // (`None` xác nhận lock hiện tại trùng byte projection rỗng đã
            // kiểm; project không dependency nên không cần source managed.)
            None => return Ok(None),
            Some(bytes) => bytes,
        }
    } else {
        project_cargo_lock(project_root, &rust_packages, &root_ids)?
    };
    let cargo_lock_path = project_root.join("Cargo.lock");
    let actual = mgc_adapter_base::project_file::read_regular_text(&cargo_lock_path, "Cargo.lock")
        .map_err(|error| MgError::Other(format!("cannot verify Cargo.lock projection: {error}")))?;
    let expected_value: toml::Value =
        toml::from_str(std::str::from_utf8(&expected).map_err(|error| {
            MgError::Other(format!("generated Cargo.lock is not UTF-8: {error}"))
        })?)
        .map_err(|error| MgError::Other(format!("parse generated Cargo.lock: {error}")))?;
    let actual_value: toml::Value = toml::from_str(&actual)
        .map_err(|error| MgError::Other(format!("Cargo.lock is not valid TOML: {error}")))?;
    if actual_value != expected_value {
        return Err(MgError::Other(format!(
            "Cargo.lock does not match the MGC dependency graph; run `mgc install` for core '{expected_core}'"
        )));
    }
    if rust_packages.is_empty() {
        return Ok(None);
    }

    let store = mgc_platform::paths::GlobalPaths::new()
        .map_err(|error| MgError::Other(format!("cannot resolve MGC store: {error}")))?
        .store;
    let source_root = store.join("cargo").join("registry").join("src");
    for package in &rust_packages {
        let source_dir = source_root.join(format!("{}-{}", package.name, package.version));
        let metadata = std::fs::symlink_metadata(&source_dir).map_err(|error| {
            MgError::Other(format!(
                "managed Cargo source is missing for '{}'@{}: {error}; run `mgc install`",
                package.name, package.version
            ))
        })?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(MgError::Other(format!(
                "managed Cargo source is not a regular directory for '{}'@{}",
                package.name, package.version
            )));
        }
        let expected_sha256 = mgc_lockfile::policy::native_sha256_hex(&package.integrity);
        let expected_sha256 = expected_sha256.as_str();
        let archive_path = source_root
            .parent()
            .ok_or_else(|| MgError::Other("Cargo source registry path is invalid".to_string()))?
            .join("cache")
            .join(format!("{}-{}.crate", package.name, package.version));
        let archive = crate::install::read_cached_rust_artifact(&archive_path, expected_sha256)?
            .ok_or_else(|| {
                MgError::Other(format!(
                    "managed Cargo archive is missing for '{}'@{}",
                    package.name, package.version
                ))
            })?;
        verify_source_checksum(&source_dir, expected_sha256, &archive).map_err(|error| {
            MgError::Other(format!(
                "managed Cargo source verification failed for '{}'@{}: {error}; run `mgc install`",
                package.name, package.version
            ))
        })?;
        // Cargo only accepts a `registry/src` directory that carries its own
        // readiness marker; without it Cargo silently re-extracts the directory,
        // which deletes `.cargo-checksum.json` and leaves the build reporting a
        // missing checksum file. Fail here with a repair instruction instead.
        // (Cargo chỉ nhận thư mục `registry/src` có marker sẵn sàng; thiếu nó
        // Cargo âm thầm giải nén lại, xoá `.cargo-checksum.json` và build báo
        // thiếu checksum. Dừng tại đây kèm hướng sửa thay vì để lỗi xảy ra sau.)
        require_cargo_source_marker(&source_dir).map_err(|error| {
            MgError::Other(format!(
                "managed Cargo source is not ready for '{}'@{}: {error}; run `mgc install`",
                package.name, package.version
            ))
        })?;
    }
    Ok(Some(source_root))
}
