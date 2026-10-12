#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Install module tests for app adapter.

use mgc_app_adapter::install::fetch::*;
use mgc_app_adapter::install::verify::*;
#[cfg(windows)]
use mgc_types::ResolvedGraph;
#[cfg(windows)]
use mgc_types::adapter::InstallOptions;
use mgc_types::{PackageId, PackageName, Version};

#[test]
fn pub_dev_package_url_format() {
    let url = pub_dev_package_url("http");
    assert_eq!(url, "https://pub.dev/packages/http");
}

#[test]
fn maven_central_url_format() {
    let url = maven_central_url("com.google.android", "material", "1.0.0");
    assert_eq!(
        url,
        "https://repo1.maven.org/maven2/com/google/android/material/1.0.0/material-1.0.0.jar"
    );
}

#[test]
fn kotlin_package_fetch_refuses_external_gradle_cache() {
    let project = tempfile::tempdir().unwrap();
    let package = PackageId::new(
        PackageName::new("sample").unwrap(),
        Version::parse("1.0.0").unwrap(),
    );
    let error = fetch_kotlin_package(project.path(), &package).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("does not own a native Gradle resolver")
    );
}

#[test]
fn cocoapods_spec_url_format() {
    let url = cocoapods_spec_url("Alamofire");
    assert_eq!(
        url,
        "https://cdn.cocoapods.org/Specs/Alamofire.podspec.json"
    );
}

#[test]
fn verify_pubspec_lock_missing_file() {
    let tmp = std::env::temp_dir().join(format!("mgc-app-verify-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();

    let result = verify_pubspec_lock(&tmp);
    assert!(result.is_err());

    std::fs::remove_dir_all(&tmp).unwrap();
}

#[test]
fn verify_gradle_lockfile_optional() {
    let tmp = std::env::temp_dir().join(format!("mgc-app-gradle-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();

    // gradle.lockfile is optional - should not error
    let result = verify_gradle_lockfile(&tmp);
    assert!(result.is_ok());

    std::fs::remove_dir_all(&tmp).unwrap();
}

#[test]
fn verify_package_resolved_missing_file() {
    let tmp = std::env::temp_dir().join(format!("mgc-app-swift-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();

    let result = verify_package_resolved(&tmp);
    assert!(result.is_err());

    std::fs::remove_dir_all(&tmp).unwrap();
}

#[test]
fn verify_package_file_with_hash() {
    let tmp = std::env::temp_dir().join(format!("mgc-app-hash-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).unwrap();
    let file_path = tmp.join("test.tar.gz");
    let content = b"test flutter package";
    std::fs::write(&file_path, content).unwrap();

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(content);
    let expected = hex::encode(hasher.finalize());

    let result = verify_package_file(&file_path, Some(&expected));
    assert!(result.is_ok());

    std::fs::remove_dir_all(&tmp).unwrap();
}

#[cfg(windows)]
fn windows_path_form(path: &std::path::Path, short: bool) -> std::path::PathBuf {
    use std::ffi::OsString;
    use std::os::windows::ffi::{OsStrExt, OsStringExt};
    use windows_sys::Win32::Storage::FileSystem::{GetLongPathNameW, GetShortPathNameW};

    let encoded = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let convert = if short {
        GetShortPathNameW
    } else {
        GetLongPathNameW
    };
    let required = unsafe { convert(encoded.as_ptr(), std::ptr::null_mut(), 0) };
    assert_ne!(required, 0, "Windows path conversion query failed");
    let mut buffer = vec![0_u16; required as usize];
    let written = unsafe { convert(encoded.as_ptr(), buffer.as_mut_ptr(), required) };
    assert!(
        written > 0 && written < required,
        "Windows path conversion failed"
    );
    std::path::PathBuf::from(OsString::from_wide(&buffer[..written as usize]))
}

#[cfg(windows)]
#[tokio::test]
async fn flutter_package_config_expands_windows_short_project_path() {
    let parent = tempfile::tempdir().unwrap();
    let long_project = parent
        .path()
        .join("flutter_project_directory_long_enough_for_83_names");
    std::fs::create_dir(&long_project).unwrap();
    std::fs::write(long_project.join("pubspec.yaml"), "name: sample\n").unwrap();
    let short_project = windows_path_form(&long_project, true);
    let expected_project = windows_path_form(&short_project, false);

    mgc_app_adapter::install::run_install(
        mgc_app_adapter::language::AppLanguage::Flutter,
        &ResolvedGraph::empty(),
        &short_project,
        InstallOptions {
            offline: true,
            ..InstallOptions::default()
        },
        None,
        Vec::new(),
    )
    .await
    .unwrap();

    let config: serde_json::Value = serde_json::from_slice(
        &std::fs::read(short_project.join(".dart_tool/package_config.json")).unwrap(),
    )
    .unwrap();
    let package_root = config["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == "sample")
        .unwrap()["rootUri"]
        .as_str()
        .unwrap();
    assert_eq!(
        package_root,
        url::Url::from_directory_path(expected_project)
            .unwrap()
            .as_str()
    );
}
