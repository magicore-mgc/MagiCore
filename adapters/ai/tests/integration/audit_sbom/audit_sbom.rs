//! AI audit and SBOM tests — kiểm thử audit và SBOM AI.
//! Owns model-artifact audit and lockfile-backed SBOM cases. — Sở hữu audit model artifact và SBOM từ lockfile.

use mgc_ai_adapter::{adapter_for, generate_sbom};
use mgc_types::capabilities::AuditProvider;

use super::support::tmp;

pub(crate) async fn audit_returns_clean_report_for_empty_project() {
    let dir = tmp("audit");
    std::fs::write(dir.join("mgc.toml"), "[ai]\nframework = \"mcp-server\"\n").unwrap();
    let adapter = adapter_for(&dir).unwrap();
    let report = adapter.audit(&dir).await.unwrap();
    assert_eq!(report.vulnerabilities.len(), 0);
}

pub(crate) async fn ai_audit_scans_each_model_root_and_checkpoint_weights() {
    let dir = tmp("audit-all-model-roots");
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"audit-roots\"\necosystem = \"ai\"\n\n[ai]\nframework = \"python-agent\"\n",
    )
    .unwrap();
    let models = dir.join("models");
    let checkpoints = dir.join("checkpoints");
    std::fs::create_dir_all(&models).unwrap();
    std::fs::create_dir_all(&checkpoints).unwrap();
    let mut safe = 2u64.to_le_bytes().to_vec();
    safe.extend_from_slice(b"{}");
    std::fs::write(models.join("safe.safetensors"), safe).unwrap();
    std::fs::write(
        checkpoints.join("fixture.pth"),
        b"legacy checkpoint cos\nsystem\ncommand\n.",
    )
    .unwrap();

    let adapter = adapter_for(&dir).unwrap();
    let report = adapter.audit(&dir).await.unwrap();

    assert!(
        report.vulnerabilities.iter().any(|finding| {
            finding.package.to_string().starts_with("model-artifact-")
                && finding.title.contains("fixture.pth")
                && !finding.title.contains(dir.to_string_lossy().as_ref())
        }),
        "the audit report must identify malicious checkpoint weights without exposing the host path"
    );
}

pub(crate) fn generate_sbom_uses_lockfile_v2_fixture() {
    let mut lockfile = mgc_lockfile::Lockfile::new();
    lockfile.add_package(mgc_lockfile::Package::new(
        "transformers".to_string(),
        "4.35.0".to_string(),
        "https://pypi.org/transformers-4.35.0.tgz".to_string(),
        "blake3:ai123".to_string(),
    ));
    let json = generate_sbom(&lockfile, mgc_sbom::SbomOptions::default()).unwrap();
    assert!(json.contains("CycloneDX"));
    assert!(json.contains("transformers"));
    assert!(json.contains("4.35.0"));
}
