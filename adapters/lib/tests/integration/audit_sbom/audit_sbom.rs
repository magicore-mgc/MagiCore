//! Lib audit and SBOM tests — kiểm thử audit và SBOM Lib.
//! Owns real scanner status and lockfile-backed SBOM cases. — Sở hữu trạng thái scanner thật và SBOM từ lockfile.

use mgc_lib_adapter::{adapter_for, generate_sbom};
use mgc_types::capabilities::AuditProvider;

use super::support::tmp;

pub(crate) async fn audit_lib_project_reports_scanner_truthfully() {
    let dir = tmp("audit-lib");
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"audit-lib\"\nversion = \"0.1.0\"\n\n[package.metadata.magicore]\ncore = \"lib\"\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nserde = \"1\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("Cargo.lock"),
        "version = 4\n\n[[package]]\nname = \"serde\"\nversion = \"1.0.219\"\nsource = \"sparse+https://index.crates.io/\"\n",
    )
    .unwrap();
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("src/lib.rs"), "// empty lib for audit test\n").unwrap();
    let a = adapter_for(&dir, None, None).unwrap().unwrap();

    let prior_osv_base = std::env::var_os("MGC_OSV_API_BASE");
    // A refused loopback port deterministically exercises tool/network
    // failure without binding sockets or depending on public OSV uptime.
    // (Port loopback đóng tạo lỗi mạng tất định, không cần server/mạng ngoài.)
    // SAFETY: test-only env mutation; callers hold `env_lock` so no other
    // test thread observes a partially written environment.
    // AN TOÀN: chỉ test đổi env; caller giữ `env_lock` nên thread test khác
    // không thấy môi trường ghi dở.
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe { std::env::set_var("MGC_OSV_API_BASE", "http://127.0.0.1:1/v1") };
    let report_result = a.audit(&dir).await;
    // SAFETY: test-only operation on process-local state / FFI with
    // caller-owned buffers kept alive for the call.
    // AN TOÀN: chỉ test thao tác state cục bộ process / FFI với
    // buffer do caller sở hữu và còn sống trong lời gọi.
    unsafe {
        if let Some(value) = prior_osv_base {
            std::env::set_var("MGC_OSV_API_BASE", value);
        } else {
            std::env::remove_var("MGC_OSV_API_BASE");
        }
    }
    let report = report_result.unwrap();
    // Rust is scanned by MGC's native OSV route. This fixture deliberately
    // refuses the local OSV endpoint, so the honest result is Failed rather
    // than ToolMissing for an external cargo-audit executable.
    // Rust được quét bằng OSV native của MGC. Fixture cố ý từ chối endpoint
    // OSV local nên kết quả trung thực là Failed, không phải ToolMissing do
    // thiếu executable cargo-audit bên ngoài.
    assert!(matches!(
        report.scanner_status,
        mgc_types::adapter::ScannerStatus::Failed { .. }
    ));
}

pub(crate) fn generate_sbom_uses_lockfile_v2_fixture() {
    let mut lockfile = mgc_lockfile::Lockfile::new();
    lockfile.add_package(mgc_lockfile::Package::new(
        "test-pkg".to_string(),
        "1.0.0".to_string(),
        "https://example.com/test.tgz".to_string(),
        "blake3:test123".to_string(),
    ));
    let json = generate_sbom(&lockfile, mgc_sbom::SbomOptions::default()).unwrap();
    assert!(json.contains("CycloneDX"));
    assert!(json.contains("test-pkg"));
    assert!(json.contains("1.0.0"));
}
