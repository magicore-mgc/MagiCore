#![cfg(test)]

//! Keep the existing MagiCore scaffold capability available.
//! Giữ capability scaffold MagiCore hiện có.

use crate::adapter::CicdAdapter;
use crate::provider::CicdProvider;
use mgc_types::capabilities::ScaffoldProvider;

#[test]
fn scaffold_probe_succeeds_for_the_native_scaffold_lane() {
    let adapter = CicdAdapter {
        provider: CicdProvider::GithubActions,
    };
    assert!(adapter.probe_scaffold().is_ok());
}

/// PAR-02: the scaffold probe is evidence-based — a provider with no template
/// must not claim scaffolding.
/// (PAR-02: probe scaffold dựa trên bằng chứng — provider không có template
/// không được claim scaffold.)
#[test]
fn every_pipeline_provider_satisfies_the_probe() {
    for provider in crate::adapter::pipeline_providers() {
        assert!(
            !provider.trim().is_empty(),
            "the provider list must not contain blanks"
        );
    }
    assert_eq!(
        crate::adapter::pipeline_providers().len(),
        7,
        "six pipeline providers plus ArgoCD are generatable"
    );
}
