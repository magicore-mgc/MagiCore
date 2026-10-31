//! Resolve a requested package or subpath to a canonical installed file.
//! Resolve package/subpath được request thành file canonical đã cài.

use std::{
    fs,
    path::{Path, PathBuf},
};

use self::manifest_entry::{exports_root_target, exports_subpath_target, resolve_manifest_entry};
use self::package_selector::{
    PackageSubpath, resolve_package_subpath, split_package_name_and_subpath,
};
use self::path_containment::{
    canonicalize_if_present, directory_inside_package, resolve_file_inside_package,
};
use super::super::name_validation::dep_name_is_valid;
use super::PackageEntryError;

#[path = "manifest_entry/mod.rs"]
mod manifest_entry;
#[path = "package_selector/mod.rs"]
mod package_selector;
#[path = "path_containment/mod.rs"]
mod path_containment;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;

/// Find a package/subpath file using browser/module/main/index precedence.
/// Tìm file package/subpath theo thứ tự browser/module/main/index.
///
/// A top-level package directory may be a pnpm/workspace symlink. Its
/// resolved directory becomes the containment root; a requested subpath
/// and its selected entry must remain under that root.
/// Package top-level có thể là symlink pnpm/workspace. Thư mục sau resolve
/// là root containment; subpath request và entry phải còn trong root đó.
pub(in crate::bundler::dependency_cache) fn resolve_package_entry(
    node_modules: &Path,
    pkg_name: &str,
) -> Result<Option<PathBuf>, PackageEntryError> {
    if !dep_name_is_valid(pkg_name) {
        return Err(PackageEntryError::InvalidPackageName);
    }

    let (package_name, package_subpath) = split_package_name_and_subpath(pkg_name)?;
    let package_root = match canonicalize_if_present(&node_modules.join(package_name))? {
        Some(path) => path,
        None => return Ok(None),
    };
    if !fs::metadata(&package_root)
        .map_err(PackageEntryError::Io)?
        .is_dir()
    {
        return Ok(None);
    }

    if package_subpath.as_os_str().is_empty() {
        return resolve_package_root_entry(&package_root);
    }

    // A requested subpath is matched the way Node resolves it: an existing
    // file or directory first, then the manifest `exports` map (which can
    // declare extensionless targets such as `./client`), then the literal
    // file, then `<subpath>.js`, then `<subpath>/index.js`. A miss is only
    // reported after every candidate is absent.
    // (Subpath được request khớp theo cách Node resolve: file/thư mục có
    // thật trước, rồi map `exports` của manifest (có thể khai báo target
    // không đuôi như `./client`), rồi file nguyên văn, rồi `<subpath>.js`,
    // rồi `<subpath>/index.js`. Chỉ báo miss sau khi hết candidate.)
    let manifest = read_manifest(&package_root)?;
    match resolve_package_subpath(&package_root, &package_subpath)? {
        Some(PackageSubpath::File(path)) => Ok(Some(path)),
        Some(PackageSubpath::Directory(directory)) => {
            resolve_directory_entry(&package_root, &directory, manifest.as_ref())
        }
        None => resolve_missing_subpath(&package_root, &package_subpath, manifest.as_ref()),
    }
}

/// Resolve the bare package request through `exports["."]`, then manifest/index.
/// Resolve request package trần qua `exports["."]`, rồi manifest/index.
fn resolve_package_root_entry(package_root: &Path) -> Result<Option<PathBuf>, PackageEntryError> {
    if let Some(target) = read_manifest(package_root)?
        .as_ref()
        .and_then(exports_root_target)
        && let Some(path) = manifest_entry::resolve_export_file(package_root, &target)?
    {
        return Ok(Some(path));
    }
    resolve_manifest_entry(package_root, package_root)
}

/// Resolve an existing subpath directory through its own manifest or `index.js`.
/// Resolve thư mục subpath có thật qua manifest riêng hoặc `index.js`.
fn resolve_directory_entry(
    package_root: &Path,
    directory: &Path,
    manifest: Option<&serde_json::Value>,
) -> Result<Option<PathBuf>, PackageEntryError> {
    if directory != package_root
        && let Some(target) = manifest
            .and_then(|manifest| exports_directory_target(manifest, directory, package_root))
        && let Some(path) = manifest_entry::resolve_export_file(package_root, &target)?
    {
        return Ok(Some(path));
    }
    resolve_manifest_entry(package_root, directory)
}

/// Resolve a subpath that has no literal file or directory on disk.
/// Resolve subpath không có file hoặc thư mục tương ứng trên đĩa.
fn resolve_missing_subpath(
    package_root: &Path,
    package_subpath: &Path,
    manifest: Option<&serde_json::Value>,
) -> Result<Option<PathBuf>, PackageEntryError> {
    let request = package_subpath.to_string_lossy().replace('\\', "/");

    if let Some(target) = manifest.and_then(|manifest| exports_subpath_target(manifest, &request))
        && let Some(path) = manifest_entry::resolve_export_file(package_root, &target)?
    {
        return Ok(Some(path));
    }

    let literal = package_root.join(package_subpath);
    if let Some(path) = resolve_file_inside_package(package_root, &literal)? {
        return Ok(Some(path));
    }
    if let Some(path) = resolve_file_inside_package(package_root, &literal.with_extension("js"))? {
        return Ok(Some(path));
    }

    // Directory index only applies when the directory exists; otherwise the
    // subpath is genuinely absent.
    // (Index thư mục chỉ áp dụng khi thư mục tồn tại; nếu không, subpath
    // thật sự không có.)
    let Some(path) = resolve_file_inside_package(package_root, &literal.join("index.js"))? else {
        return Ok(None);
    };
    // A symlinked candidate is only reachable through a real directory.
    // (Candidate là symlink chỉ dùng được khi nó trỏ tới thư mục thật.)
    if !directory_inside_package(package_root, &literal)? {
        return Ok(None);
    }
    Ok(Some(path))
}

/// Read and parse the package root manifest; a missing or malformed manifest is `None`.
/// Đọc và parse manifest package root; manifest thiếu hoặc sai định dạng là `None`.
fn read_manifest(package_root: &Path) -> Result<Option<serde_json::Value>, PackageEntryError> {
    let Some(manifest_path) =
        resolve_file_inside_package(package_root, &package_root.join("package.json"))?
    else {
        return Ok(None);
    };
    let raw = fs::read_to_string(manifest_path).map_err(PackageEntryError::Io)?;
    Ok(serde_json::from_str(&raw).ok())
}

/// Map an existing subpath directory to its `exports` request form.
/// Map thư mục subpath có thật sang dạng request `exports`.
fn exports_directory_target(
    manifest: &serde_json::Value,
    directory: &Path,
    package_root: &Path,
) -> Option<String> {
    let relative = directory.strip_prefix(package_root).ok()?;
    let request = relative.to_string_lossy().replace('\\', "/");
    exports_subpath_target(manifest, &request)
}
