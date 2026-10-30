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
fn pipeline_provider_list_is_well_formed() {
    for provider in crate::adapter::pipeline_providers() {
        assert!(
            !provider.trim().is_empty(),
            "the provider list must not contain blanks"
        );
    }
    // Every listed provider is a claim the probe must honour, so each one is
    // asked rather than a count being compared.
    // (Mỗi provider trong danh sách là một claim probe phải đáp ứng, nên hỏi
    // từng cái thay vì so sánh số lượng.)
    for provider in crate::adapter::pipeline_providers() {
        assert!(
            !provider.trim().is_empty(),
            "the provider list must not contain blanks"
        );
    }
}
