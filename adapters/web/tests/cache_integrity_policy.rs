#![allow(clippy::unwrap_used)]
// Cache preparation verifies supplied SRI before deriving a cache identity.
// Test strict/permissive trong process riêng, không race env giữa các test.
use mgc_types::adapter::ResolvedPackage;
use mgc_types::{PackageId, PackageName, Version};
const CHILD_ENV: &str = "MGC_CACHE_INTEGRITY_TEST_CHILD";
fn run_isolated(name: &str, strict: bool) -> bool {
    if std::env::var(CHILD_ENV).as_deref() == Ok(name) {
        return true;
    }
    let mut command = std::process::Command::new(std::env::current_exe().unwrap());
    command
        .args(["--exact", name, "--test-threads=1", "--nocapture"])
        .env(CHILD_ENV, name)
        .env_remove("MGC_STRICT_INTEGRITY")
        .env_remove("MAGICORE_STRICT_INTEGRITY")
        // SEC-01 made unsigned tarballs deny-by-default; this control asserts
        // the documented escape hatch is the only way to accept one.
        // (SEC-01 khiến tarball không SRI bị chặn mặc định; control này kiểm
        // escape hatch là đường duy nhất được chấp nhận.)
        .env_remove("MAGICORE_ALLOW_UNSIGNED_INTEGRITY")
        .env_remove("MGC_ALLOW_UNSIGNED_INTEGRITY");
    if strict {
        command.env("MGC_STRICT_INTEGRITY", "1");
    } else {
        command.env("MAGICORE_ALLOW_UNSIGNED_INTEGRITY", "1");
    }
    let output = command
        .output()
        .expect("run isolated cache integrity control");
    assert!(
        output.status.success(),
        "isolated cache integrity control failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    false
}
#[test]
fn strict_missing_sri_cache_preparation_must_deny() {
    if !run_isolated("strict_missing_sri_cache_preparation_must_deny", true) {
        return;
    }
    let mut pkg = ResolvedPackage {
        id: PackageId::new(
            PackageName::new("fixture").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity: String::new(),
        tarball_url: String::new(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    };
    let result = mgc_web_adapter::install::integrity::prepare_verified_tarball_for_cache(
        &mut pkg,
        b"unverified-new-bytes",
    );
    eprintln!(
        "strict_missing_accepted={} integrity_was_imputed={}",
        result.is_ok(),
        !pkg.integrity.is_empty()
    );
    assert!(
        result.is_err(),
        "strict missing SRI must not be replaced by a self-computed hash"
    );
}
fn package(integrity: String) -> ResolvedPackage {
    ResolvedPackage {
        id: PackageId::new(
            PackageName::new("fixture").unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        integrity,
        tarball_url: String::new(),
        deps: vec![],
        peer_deps: vec![],
        direct: true,
        dev: false,
    }
}
#[test]
fn matching_sri_cache_preparation_accepts() {
    if !run_isolated("matching_sri_cache_preparation_accepts", true) {
        return;
    }
    let mut pkg=package("sha512-ojkxJesguv5tUca+J4N0oV1Nk/1JHzsvRA1VQCWmM64t/SYemnt7/4QCU1srnoQw7ESAi5JeQq4nxIDfvlJlFg==".to_string());
    let before = pkg.integrity.clone();
    assert!(
        mgc_web_adapter::install::integrity::prepare_verified_tarball_for_cache(
            &mut pkg,
            b"unverified-new-bytes"
        )
        .is_ok()
    );
    assert_eq!(pkg.integrity, before, "supplied SRI must remain unchanged");
}
#[test]
fn wrong_sri_cache_preparation_denies() {
    if !run_isolated("wrong_sri_cache_preparation_denies", true) {
        return;
    }
    let mut pkg = package("sha512-invalid".to_string());
    assert!(
        mgc_web_adapter::install::integrity::prepare_verified_tarball_for_cache(
            &mut pkg,
            b"unverified-new-bytes"
        )
        .is_err()
    );
    assert_eq!(
        pkg.integrity, "sha512-invalid",
        "rejection must preserve supplied evidence"
    );
}
#[test]
fn permissive_missing_sri_cache_preparation_accepts() {
    if !run_isolated("permissive_missing_sri_cache_preparation_accepts", false) {
        return;
    }
    let mut pkg = package(String::new());
    assert!(
        mgc_web_adapter::install::integrity::prepare_verified_tarball_for_cache(
            &mut pkg,
            b"unverified-new-bytes"
        )
        .is_ok()
    );
    assert_eq!(
        pkg.integrity,
        "sha512-ojkxJesguv5tUca+J4N0oV1Nk/1JHzsvRA1VQCWmM64t/SYemnt7/4QCU1srnoQw7ESAi5JeQq4nxIDfvlJlFg==",
        "the escape hatch must still record a deterministic cache identity"
    );
}
