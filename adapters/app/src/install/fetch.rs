//! `install/fetch.rs` — Package fetching for mobile app platforms.
//! URL builders for pub.dev (Flutter), Maven (Kotlin), CocoaPods (iOS).

use mgc_types::{MgError, MgResult, PackageId};
use std::path::PathBuf;

/// Fetch Flutter package from pub.dev.
/// Tải Flutter package từ pub.dev.
pub fn fetch_flutter_package(
    _project_root: &std::path::Path,
    _package_id: &PackageId,
) -> MgResult<PathBuf> {
    // Flutter packages fetched by MGC live in its managed pub store.
    let cache_dir = mgc_platform::paths::mgc_data_dir()
        .join("store")
        .join("pub")
        .join("hosted")
        .join("pub.dev");

    let package_dir = cache_dir.join(format!("{}-{}", _package_id.name(), _package_id.version()));

    if package_dir.exists() {
        Ok(package_dir)
    } else {
        Err(MgError::Other(format!(
            "Flutter package not found in cache: {}",
            package_dir.display()
        )))
    }
}

/// Fetch Kotlin/Android package from Maven Central.
/// Tải Kotlin/Android package từ Maven Central.
pub fn fetch_kotlin_package(
    _project_root: &std::path::Path,
    _package_id: &PackageId,
) -> MgResult<PathBuf> {
    Err(MgError::Unsupported {
        core: "app",
        capability: "native Kotlin package fetch",
        guidance:
            "MagiCore does not own a native Gradle resolver or verified Kotlin package store yet"
                .to_string(),
    })
}

/// Fetch Swift package from Swift Package Manager cache.
/// Tải Swift package từ Swift Package Manager cache.
pub fn fetch_swift_package(
    _project_root: &std::path::Path,
    _package_id: &PackageId,
) -> MgResult<PathBuf> {
    // Resolve the same managed root used by native install, including its
    // override, and address the versioned checkout produced by SwiftPM.
    // (Dùng cùng gốc store với native install, gồm cả override, và tìm
    // checkout SwiftPM có ghim phiên bản.)
    let swift_root = std::env::var("MGC_SWIFT_STORE_ROOT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            mgc_platform::paths::mgc_data_dir()
                .join("store")
                .join("swift")
        });

    let identity = _package_id.name().as_str().replace('/', ".");
    let package_dir = swift_root
        .join("checkouts")
        .join(format!("{identity}-{}", _package_id.version()));

    if package_dir.exists() {
        Ok(package_dir)
    } else {
        Err(MgError::Other(format!(
            "Swift package not found in cache: {}",
            package_dir.display()
        )))
    }
}

/// Fetch CocoaPods pod from Pods/ directory.
/// Tải CocoaPods pod từ thư mục Pods/.
pub fn fetch_cocoapods_pod(
    project_root: &std::path::Path,
    _package_id: &PackageId,
) -> MgResult<PathBuf> {
    let pods_dir = project_root.join("Pods").join(_package_id.name().as_str());

    if pods_dir.exists() {
        Ok(pods_dir)
    } else {
        Err(MgError::Other(format!(
            "CocoaPods pod not found: {}",
            pods_dir.display()
        )))
    }
}

/// Construct URL for Flutter package on pub.dev.
/// Xây dựng URL cho Flutter package trên pub.dev.
pub fn pub_dev_package_url(package_name: &str) -> String {
    format!("https://pub.dev/packages/{}", package_name)
}

/// Construct URL for Maven artifact.
/// Xây dựng URL cho Maven artifact.
pub fn maven_central_url(group_id: &str, artifact_id: &str, version: &str) -> String {
    let group_path = group_id.replace('.', "/");
    format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}-{}.jar",
        group_path, artifact_id, version, artifact_id, version
    )
}

/// Construct URL for CocoaPods spec.
/// Xây dựng URL cho CocoaPods spec.
pub fn cocoapods_spec_url(pod_name: &str) -> String {
    format!("https://cdn.cocoapods.org/Specs/{}.podspec.json", pod_name)
}
