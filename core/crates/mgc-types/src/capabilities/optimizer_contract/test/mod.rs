//! Verify the optimizer capability contract admits exactly the audited cores.
//! Kiểm hợp đồng capability optimizer chứa đúng các core đã audit.

#![cfg(test)]

use super::super::{OPTIMIZER_CAPABILITY_CORES, optimizer_capability_supported};
use crate::ecosystem::Ecosystem;

/// The audited admission set: every core whose `mgc optimizer` run is complete
/// without a language runtime.
/// (Tập đã audit: mọi core mà `mgc optimizer` chạy trọn khi không có runtime.)
#[test]
fn contract_matches_the_audited_admission_set() {
    assert_eq!(
        OPTIMIZER_CAPABILITY_CORES,
        &["web", "lib", "ai", "app", "game", "iot", "clo", "cicd"]
    );
}

#[test]
fn every_contract_core_is_a_canonical_ecosystem_name() {
    for core in OPTIMIZER_CAPABILITY_CORES {
        let ecosystem = Ecosystem::from_str(core).unwrap_or_else(|| {
            panic!("'{core}' is listed but is not an ecosystem name");
        });
        assert_eq!(
            ecosystem.as_str(),
            *core,
            "contract entries must use the canonical CLI name"
        );
        assert!(optimizer_capability_supported(core));
    }
}

#[test]
fn hardware_is_not_admitted() {
    // PA-1: the optimizer/bench template listing is scaffold evidence only.
    // (PA-1: template optimizer/bench chỉ là dẫn chứng scaffold.)
    assert!(!optimizer_capability_supported("hardware"));
    assert!(!OPTIMIZER_CAPABILITY_CORES.contains(&"hardware"));
}

#[test]
fn lookup_accepts_the_cloud_host_tag() {
    assert!(optimizer_capability_supported("clo"));
    assert!(optimizer_capability_supported("cloud"));
    assert!(!optimizer_capability_supported("unknown"));
    assert!(!optimizer_capability_supported(""));
}

#[test]
fn every_ecosystem_is_either_admitted_or_explicitly_absent() {
    // Exhaustive on purpose: adding an ecosystem forces this test to state
    // whether its core has an optimizer.
    // (Exhaustive có chủ đích: thêm ecosystem buộc phải khai là có/không optimizer.)
    let mut absent = Vec::new();
    for ecosystem in Ecosystem::ALL {
        if !optimizer_capability_supported(ecosystem.as_str()) {
            absent.push(ecosystem.as_str());
        }
    }
    assert_eq!(absent, vec!["hardware"]);
}
