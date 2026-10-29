//! Audit Sbom behavior tests.
//! Test hành vi cho nhóm audit sbom.

use crate::integration::support::tmp;
use mgc_game_adapter::{adapter_for, generate_sbom};
use mgc_types::capabilities::AuditProvider;

/// PAR-04: a Godot project has no shared manifest, so its audit must say
/// "not scanned" rather than return an empty list that reads as "no
/// vulnerabilities found".
/// (PAR-04: project Godot không có manifest chung, nên audit phải nói "chưa
/// quét" thay vì trả danh sách rỗng đọc thành "không có lỗ hổng".)
pub async fn audit_reports_unsupported_instead_of_a_clean_scan() {
    let dir = tmp("audit-game");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let report = a.audit(&dir).await.unwrap();
    assert!(
        report.vulnerabilities.is_empty(),
        "a Godot project has no scannable manifest"
    );
    // The report must not claim a completed clean scan: only `Available` with
    // zero findings may read as "clean".
    // (Report không được nhận là đã quét sạch: chỉ `Available` + 0 finding mới
    // đọc thành "clean".)
    assert!(
        !matches!(report.scanner_status, mgc_types::ScannerStatus::Available),
        "an unscanned project must not claim a clean scan: {:?}",
        report.scanner_status
    );
}

pub fn generate_sbom_uses_lockfile_v2_fixture() {
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
