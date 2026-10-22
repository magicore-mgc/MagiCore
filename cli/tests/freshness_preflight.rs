#![allow(clippy::unwrap_used)]
//! Freshness pre-flight tests — kiểm thử freshness pre-flight.
//!
//! `check_dev_freshness` fails before anything spawns when manifest
//! dependencies drift from the lock (Spec rev4 §8); scripts-only edits
//! never trigger a resolve.
//! (`check_dev_freshness` fail trước khi spawn khi dependency manifest lệch
//! lock (Spec rev4 §8); sửa scripts không trigger resolve.)

use mgc::commands::core::web::check_dev_freshness;

fn write_project(dir: &std::path::Path, package_json: &str, lock: Option<&str>) {
    std::fs::write(dir.join("package.json"), package_json).unwrap();
    if let Some(content) = lock {
        std::fs::write(dir.join("mgc.lock"), content).unwrap();
    }
}

fn lock_with(packages: &[(&str, &str)]) -> String {
    let mut lock = mgc_lockfile::Lockfile::new();
    for (name, version) in packages {
        let mut package = mgc_lockfile::Package::new(
            (*name).into(),
            (*version).into(),
            format!("https://registry.npmjs.org/{name}/-/{name}-{version}.tgz"),
            "sha256-test-integrity".to_string(),
        );
        package.owner_core = Some("web".into());
        package.ecosystem = mgc_lockfile::EcosystemTag::Web;
        package.registry = Some("https://registry.npmjs.org/".into());
        lock.add_package(package);
    }
    mgc_lockfile::update_owner_root_pins(
        &mut lock,
        "web",
        mgc_lockfile::EcosystemTag::Web,
        packages
            .iter()
            .map(|(name, version)| format!("{name}@{version}")),
    );
    mgc_lockfile::writer::serialize_lockfile(&lock).unwrap()
}

fn v4_lock_with(packages: &[(&str, &str)]) -> String {
    let mut lock = mgc_lockfile::Lockfile::new();
    for (name, version) in packages {
        let mut package = mgc_lockfile::Package::new(
            (*name).into(),
            (*version).into(),
            format!("https://registry.npmjs.org/{name}/-/{name}-{version}.tgz"),
            "sha512-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==".to_string(),
        );
        package.owner_core = Some("web".into());
        package.ecosystem = mgc_lockfile::EcosystemTag::Web;
        package.registry = Some("https://registry.npmjs.org/".into());
        lock.add_package(package);
    }
    mgc_lockfile::update_owner_root_pins(
        &mut lock,
        "web",
        mgc_lockfile::EcosystemTag::Web,
        packages
            .iter()
            .map(|(name, version)| format!("{name}@{version}")),
    );
    let (mut document, _) = mgc_lockfile::migrate::migrate_v3_to_v4(lock).unwrap();
    mgc_lockfile::writer::finalize_v4_document(&mut document).unwrap()
}

// Missing lock fails closed with an install hint.
// (Thiếu lock thì fail-closed kèm hướng dẫn install.)
#[test]
fn missing_lock_fails_closed() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","scripts":{"dev":"next dev"},"dependencies":{"react":"18.2.0"}}"#,
        None,
    );
    let err = check_dev_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("mgc install"),
        "unexpected error: {err}"
    );
}

// The dev lock requirement also applies to empty dependency sets.
// Dev vẫn yêu cầu lock khi tập dependency rỗng.
#[test]
fn empty_manifest_still_requires_lock_for_dev() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","scripts":{"dev":"node server.js"},"dependencies":{}}"#,
        None,
    );
    let err = check_dev_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("mgc.lock"),
        "unexpected error: {err}"
    );
}

// A project with no ecosystem dependency manifest needs no dependency lock.
// Project không có manifest dependency thì không cần lock dependency.
#[test]
fn missing_manifest_needs_no_lock_for_dev() {
    let dir = tempfile::TempDir::new().unwrap();
    check_dev_freshness(dir.path()).unwrap();
}

// Declared but unlocked dependency fails closed.
// (Dependency khai báo nhưng chưa lock thì fail-closed.)
#[test]
fn undeclared_drift_fails_closed() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"react":"18.2.0","left-pad":"1.3.0"}}"#,
        Some(&lock_with(&[("react", "18.2.0")])),
    );
    let err = check_dev_freshness(dir.path()).unwrap_err();
    let message = err.to_string();
    assert!(message.contains("left-pad"), "unexpected error: {message}");
    assert!(
        message.contains("mgc install"),
        "unexpected error: {message}"
    );
}

// Matching manifest and lock passes.
// (Manifest khớp lock thì qua.)
#[test]
fn matching_lock_passes() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"react":"18.2.0"},"devDependencies":{"typescript":"5.4.0"}}"#,
        Some(&lock_with(&[("react", "18.2.0"), ("typescript", "5.4.0")])),
    );
    check_dev_freshness(dir.path()).unwrap();
}

// A same-name dependency with an out-of-range locked version is stale.
// Dependency cùng tên nhưng version lock nằm ngoài range là stale.
#[test]
fn locked_version_outside_manifest_range_fails_closed() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"next":"^16.4.0"}}"#,
        Some(&lock_with(&[("next", "16.3.8")])),
    );
    let err = check_dev_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("next") && err.to_string().contains("16.4.0"),
        "unexpected error: {err}"
    );
}

// A locked version inside the declared range remains fresh.
// Version đã lock nằm trong range khai báo vẫn fresh.
#[test]
fn locked_version_inside_manifest_range_passes() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"next":"^16.3.0"}}"#,
        Some(&lock_with(&[("next", "16.3.8")])),
    );
    check_dev_freshness(dir.path()).unwrap();
}

// A dist tag containing `x` is not a semver range and remains root-checked.
// Dist tag có `x` không phải range semver nhưng vẫn phải có root lock.
#[test]
fn npm_dist_tag_containing_x_passes_root_freshness() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"next":"experimental"}}"#,
        Some(&lock_with(&[("next", "16.3.8")])),
    );
    check_dev_freshness(dir.path()).unwrap();
}

// Duplicate declarations must share one locked version satisfying all ranges.
// Khai báo trùng phải cùng thỏa bằng một version lock duy nhất.
#[test]
fn duplicate_dependency_ranges_all_must_match() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"next":"^16.3.0"},"devDependencies":{"next":"^16.4.0"}}"#,
        Some(&lock_with(&[("next", "16.3.8")])),
    );
    let err = check_dev_freshness(dir.path()).unwrap_err();
    let message = err.to_string();
    assert!(message.contains("next"), "unexpected error: {message}");
    assert!(message.contains("^16.3.0"), "unexpected error: {message}");
    assert!(message.contains("^16.4.0"), "unexpected error: {message}");
}

// Removed declarations leave stale direct roots and fail before launch.
// Xóa khai báo nhưng root pin cũ vẫn còn thì phải lỗi trước khi launch.
#[test]
fn removed_dependency_fails_closed() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"react":"18.2.0"}}"#,
        Some(&lock_with(&[("react", "18.2.0"), ("left-pad", "1.3.0")])),
    );
    let err = check_dev_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("left-pad"),
        "unexpected error: {err}"
    );
    assert!(err.to_string().contains("mgc install"));
}

// V4 lock identity is read without flattening it into the legacy model.
// (Identity V4 được đọc trực tiếp, không ép về model legacy.)
#[test]
fn v4_lock_passes_freshness_and_resolves_framework_version() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","scripts":{"dev":"next dev"},"dependencies":{"next":"16.3.8"}}"#,
        Some(&v4_lock_with(&[("next", "16.3.8")])),
    );
    check_dev_freshness(dir.path()).unwrap();
    assert_eq!(
        mgc::commands::freshness::resolve_web_framework(dir.path()).unwrap(),
        Some(("next".to_string(), "16.3.8".to_string()))
    );
    mgc::commands::freshness::check_profile_gate(
        dir.path(),
        &dir.path().join("profiles"),
        "dev",
        None,
    )
    .unwrap();
}

// Missing V4 package identities still fail with the normal install guidance.
// (Thiếu identity package trong V4 vẫn báo hướng dẫn install.)
#[test]
fn v4_lock_with_missing_package_fails_closed() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"next":"16.3.8","react":"19.0.0"}}"#,
        Some(&v4_lock_with(&[("next", "16.3.8")])),
    );
    let err = check_dev_freshness(dir.path()).unwrap_err();
    assert!(err.to_string().contains("react"), "unexpected error: {err}");
}

// A malformed profile is not the same as a lane that has never been profiled.
// Profile sai không được coi như lane chưa từng có profile.
#[test]
fn invalid_spawn_profile_fails_before_native_dev() {
    let dir = tempfile::TempDir::new().unwrap();
    let package_json =
        r#"{"name":"a","scripts":{"dev":"next dev"},"dependencies":{"next":"16.3.8"}}"#;
    let lock = lock_with(&[("next", "16.3.8")]);
    write_project(dir.path(), package_json, Some(&lock));

    let store = tempfile::TempDir::new().unwrap();
    let profile_path = store
        .path()
        .join(format!("next-dev-{}.json", std::env::consts::OS));
    for profile_text in [
        "{ malformed profile }".to_string(),
        serde_json::json!({
            "schema_version": "spawn-profile-v99",
            "mgc_version": "1.1.0-rc.10",
            "captured_at": 1,
            "profile": {
                "framework": "next",
                "version": "16.3.8",
                "os": std::env::consts::OS,
                "operation": "dev",
                "allowed_pm_invocations": [],
                "requires_pm": []
            }
        })
        .to_string(),
    ] {
        std::fs::write(&profile_path, profile_text).unwrap();
        let err =
            mgc::commands::freshness::check_profile_gate(dir.path(), store.path(), "dev", None)
                .unwrap_err();
        assert!(
            err.to_string().contains("invalid spawn profile"),
            "got: {err}"
        );
        assert!(err.to_string().contains("refusing"), "got: {err}");
    }
}

// Invalid JSON must not become an empty dependency set.
// (JSON không hợp lệ không được biến thành tập dependency rỗng.)
#[test]
fn malformed_package_json_fails_any_freshness() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":{"next": "16.3.8"}"#,
        Some(&lock_with(&[("next", "16.3.8")])),
    );
    let err = mgc::commands::freshness::check_any_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("valid JSON"),
        "unexpected error: {err}"
    );
}

// A valid JSON document with an invalid dependency-section type also fails closed.
// (JSON hợp lệ nhưng section dependency sai kiểu vẫn fail-closed.)
#[test]
fn malformed_package_dependency_shape_fails_any_freshness() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","dependencies":["next"]}"#,
        Some(&lock_with(&[("next", "16.3.8")])),
    );
    let err = mgc::commands::freshness::check_any_freshness(dir.path()).unwrap_err();
    assert!(
        err.to_string().contains("must be an object"),
        "unexpected error: {err}"
    );
}

// Scripts-only edits never trigger a resolve.
// (Sửa scripts không trigger resolve.)
#[test]
fn scripts_only_change_passes() {
    let dir = tempfile::TempDir::new().unwrap();
    write_project(
        dir.path(),
        r#"{"name":"a","scripts":{"dev":"next dev --turbo"},"dependencies":{"react":"18.2.0"}}"#,
        Some(&lock_with(&[("react", "18.2.0")])),
    );
    check_dev_freshness(dir.path()).unwrap();
}

// No package.json means nothing to compare.
// (Không có package.json thì không có gì để so.)
#[test]
fn no_manifest_passes() {
    let dir = tempfile::TempDir::new().unwrap();
    check_dev_freshness(dir.path()).unwrap();
}
