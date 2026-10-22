//! Audit Sbom behavior tests.
//! Checks audit reporting and SBOM generation from the lockfile fixture.

use crate::integration::support::tmp;
use mgc_hardware_adapter::{HardwareAdapter, generate_sbom};
use mgc_types::capabilities::AuditProvider;

pub async fn audit_returns_clean_for_hardware_project() {
    let dir = tmp("audit");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"hardware\"\n").unwrap();
    let report = HardwareAdapter.audit(&dir).await.unwrap();
    assert_eq!(report.vulnerabilities.len(), 0);
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
