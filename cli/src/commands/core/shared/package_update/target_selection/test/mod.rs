//! Update-target "current version" tests — test version hiện tại của update.
//! `mgc update` reports `from → to`. Reading `from` from the manifest RANGE shows
//! the range minimum, so a project declaring `^5.0.0` while running `5.1.16` was
//! told "5.0.0 → 6.0.2" — a wrong starting point for a major-version decision.
//! (`mgc update` báo `from → to`. Đọc `from` từ RANGE trong manifest cho ra min
//! của range, nên project khai `^5.0.0` mà chạy `5.1.16` bị báo "5.0.0 → 6.0.2".)

use super::{InstalledVersions, installed_version};
use mgc_types::DependencySpec;
use mgc_types::package::{PackageName, VersionRange};

fn dependency(name: &str, range: &str) -> DependencySpec {
    DependencySpec::new(
        PackageName::new(name.to_string()).expect("valid name"),
        VersionRange::parse(range).expect("valid range"),
    )
}

fn lock_with(name: &str, version: &str) -> InstalledVersions {
    vec![(name.to_string(), version.to_string())]
}

/// The lock's version wins over the range minimum.
/// (Version trong lock thắng min của range.)
#[test]
fn the_installed_version_comes_from_the_lock() {
    let spec = dependency("nanoid", "^5.0.0");
    let lock = lock_with("nanoid", "5.1.16");
    assert_eq!(
        installed_version(&spec, Some(&lock)),
        "5.1.16",
        "a project running 5.1.16 must not report 5.0.0"
    );
}

/// Without a lock the range minimum is the only available answer.
/// (Không có lock thì min của range là câu trả lời duy nhất.)
#[test]
fn without_a_lock_the_range_minimum_is_used() {
    let spec = dependency("nanoid", "^5.0.0");
    assert_eq!(installed_version(&spec, None), "5.0.0");
}

/// A dependency the lock does not carry falls back to its range.
/// (Dependency không có trong lock quay về range.)
#[test]
fn a_dependency_absent_from_the_lock_falls_back_to_its_range() {
    let spec = dependency("vitest", "^2.0.0");
    let lock = lock_with("nanoid", "5.1.16");
    assert_eq!(installed_version(&spec, Some(&lock)), "2.0.0");
}

/// With multiple locked versions the first match is reported; the caller's
/// later resolution still upgrades the one it targets.
/// (Nhiều version trong lock: báo cái khớp đầu tiên; bước resolve sau vẫn nâng
/// đúng cái đích.)
#[test]
fn a_multi_version_entry_still_reports_a_known_version() {
    let mut lock = lock_with("picomatch", "2.3.2");
    lock.push(("picomatch".to_string(), "4.0.7".to_string()));

    let reported = installed_version(&dependency("picomatch", "^2.0.0"), Some(&lock));
    assert!(
        reported == "2.3.2" || reported == "4.0.7",
        "a multi-version entry must report one of its real versions, got {reported}"
    );
}
