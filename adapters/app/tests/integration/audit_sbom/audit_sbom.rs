//! App audit and SBOM tests — kiểm thử audit và SBOM App.
//! Owns project audit and SBOM cases. — Sở hữu audit project và các case SBOM.

use mgc_app_adapter::{adapter_for, generate_sbom};
use mgc_types::capabilities::AuditProvider;

use super::support::tmp;

pub(crate) async fn audit_returns_clean_for_empty_project() {
    let dir = tmp("audit");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    let report = a.audit(&dir).await.unwrap();
    assert_eq!(report.vulnerabilities.len(), 0);
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
