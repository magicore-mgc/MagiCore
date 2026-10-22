#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Keep the audit probe and clean project audit path available.
//! Giữ probe audit và luồng audit project sạch.

use crate::adapter::CicdAdapter;
use crate::provider::CicdProvider;
use mgc_types::capabilities::AuditProvider;
use tempfile::tempdir;

#[tokio::test]
async fn clean_provider_project_returns_a_clean_audit_report() {
    let project = tempdir().unwrap();
    std::fs::write(
        project.path().join("wrangler.toml"),
        "name = 'worker'
",
    )
    .unwrap();
    let adapter = CicdAdapter {
        provider: CicdProvider::Cloudflare,
    };
    assert!(adapter.probe_audit_provider().is_ok());
    let report = adapter.audit(project.path()).await.unwrap();
    assert!(report.vulnerabilities.is_empty());
}
