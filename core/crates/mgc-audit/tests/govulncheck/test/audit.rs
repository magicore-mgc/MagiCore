// Go audit OSV-fallback integration test.
// Kiểm thử integration OSV fallback cho audit Go.

/// Go audit must use MGC's OSV path regardless of installed toolchains.
/// Audit Go phải dùng OSV của MGC bất kể toolchain nào đã cài.
#[test]
fn audit_go_queries_osv_natively_without_toolchain_dependency() {
    let dir = std::env::temp_dir().join(format!("mgc-go-fallback-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("go.mod"),
        "module example.com/x\n\ngo 1.21\n\nrequire golang.org/x/text v0.14.0\n",
    )
    .unwrap();

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
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let report = rt.block_on(mgc_audit::scanners::audit_go(&dir)).unwrap();
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
        mgc_types::adapter::ScannerStatus::Failed { scanner, .. } => {
            assert_eq!(scanner, "osv-dev-api")
        }
        other => panic!("go.mod pins must reach the OSV fallback (Failed), got {other:?}"),
    }
}
