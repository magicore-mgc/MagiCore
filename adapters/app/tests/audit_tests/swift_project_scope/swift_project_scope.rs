#![allow(unsafe_code)]

//! Swift lockfile discovery scope cases.
//! Các case giới hạn tìm lockfile Swift.

use crate::audit_tests::tmp;
use mgc_app_adapter::audit::scanner::audit_multi;
use mgc_types::adapter::ScannerStatus;

// The first case redirects its OSV request to loopback; no public OSV call is intended.
// Case đầu chuyển request OSV về loopback; không chủ đích gọi OSV công cộng.

#[tokio::test]
async fn audit_multi_finds_parent_package_resolved() {
    let parent = tmp("multi-parent-resolved");
    std::fs::write(
        parent.join("Package.resolved"),
        r#"{"pins":[{"identity":"SomePkg","location":"https://github.com/owner/repo.git","state":{"version":"1.0.0"}}],"version":2}"#,
    )
    .unwrap();
    let child = parent.join("child");
    std::fs::create_dir_all(&child).unwrap();

    // Dead OSV endpoint: deterministic without network — the swift step
    // must EXIST (Failed naming osv-dev-api), proving the gate saw the
    // parent lockfile. Absent gate → UnsupportedEcosystem empty plan.
    // Endpoint OSV chết: step swift phải TỒN TẠI (Failed nêu osv-dev-api)
    // chứng minh gate thấy lockfile cha. Gate vắng → plan rỗng Unsupported.
    let saved = std::env::var("MGC_OSV_API_BASE").ok();
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("MGC_OSV_API_BASE", "http://127.0.0.1:1/v1") };
    let report = audit_multi(&child).await.unwrap();
    match saved {
        // SAFETY: test-only operation on process-local state or FFI with
        // caller-owned arguments valid for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
        // đối số do caller sở hữu, hợp lệ trong lời gọi.
        Some(v) => unsafe { std::env::set_var("MGC_OSV_API_BASE", v) },
        // SAFETY: test-only operation on process-local state or FFI with
        // caller-owned arguments valid for the call.
        // AN TOÀN: chỉ test thao tác state cục bộ process hoặc FFI với
        // đối số do caller sở hữu, hợp lệ trong lời gọi.
        None => unsafe { std::env::remove_var("MGC_OSV_API_BASE") },
    }
    match &report.scanner_status {
        ScannerStatus::Failed { scanner, .. } => assert_eq!(scanner, "osv-dev-api"),
        other => panic!("parent Package.resolved must enter the plan (Failed), got {other:?}"),
    }
}

#[tokio::test]
async fn audit_multi_ignores_grandparent_package_resolved() {
    let grandparent = tmp("multi-grandparent-resolved");
    std::fs::write(
        grandparent.join("Package.resolved"),
        r#"{"pins":[{"identity":"SomePkg","location":"https://github.com/owner/repo.git","state":{"version":"1.0.0"}}],"version":2}"#,
    )
    .unwrap();
    let child = grandparent.join("mid").join("child");
    std::fs::create_dir_all(&child).unwrap();

    let report = audit_multi(&child).await.unwrap();
    assert!(
        matches!(
            report.scanner_status,
            ScannerStatus::UnsupportedEcosystem { .. }
        ),
        "grandparent lockfile must stay out of scope (empty plan), got {:?}",
        report.scanner_status
    );
}
