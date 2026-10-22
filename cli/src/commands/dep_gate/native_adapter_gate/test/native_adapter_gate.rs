//! Native adapter gate contract — contract gate adapter native.
//! Exercises identity, required capabilities, update support, and compat refusal.
//! Kiểm tra identity, capability bắt buộc, update và từ chối compat.

use super::adapter_stub::{AdapterStub, ai_identity};
use crate::commands::compat::CompatMode;
use crate::commands::dep_gate::{DepContext, DepOp, gate_native_adapter};
use mgc_types::adapter::ManifestIdentity;
use mgc_types::capabilities::Capability;

fn ai_context(operation: DepOp) -> DepContext<'static> {
    DepContext::new("ai", Some("python"), None, None, operation)
}

#[test]
fn owned_pep621_adapter_with_content_store_can_install() {
    let adapter = AdapterStub {
        identity: Some(ai_identity()),
        capabilities: &[Capability::ContentStoreProvider],
        native_update: false,
    };
    assert!(
        gate_native_adapter(&ai_context(DepOp::Install), &adapter, &CompatMode::Native).is_ok()
    );
}

#[test]
fn identity_mismatch_fails_before_operation_capabilities() {
    let adapter = AdapterStub {
        identity: Some(ManifestIdentity {
            core: "lib".into(),
            language: "rust".into(),
            ..ai_identity()
        }),
        capabilities: &[Capability::ContentStoreProvider],
        native_update: false,
    };
    let error = gate_native_adapter(&ai_context(DepOp::Install), &adapter, &CompatMode::Native)
        .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("native adapter identity mismatch")
    );
}

#[test]
fn missing_content_store_capability_fails_closed() {
    let adapter = AdapterStub {
        identity: Some(ai_identity()),
        capabilities: &[],
        native_update: false,
    };
    assert!(
        gate_native_adapter(&ai_context(DepOp::Install), &adapter, &CompatMode::Native).is_err()
    );
}

#[test]
fn compatibility_is_rejected_before_missing_identity_can_be_used() {
    let adapter = AdapterStub {
        identity: None,
        capabilities: &[],
        native_update: false,
    };
    let error = gate_native_adapter(
        &ai_context(DepOp::Install),
        &adapter,
        &CompatMode::Explicit("pip".to_string()),
    )
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("disabled for dependency operations")
    );
}

#[test]
fn update_requires_both_resolver_capability_and_native_update_support() {
    let adapter = AdapterStub {
        identity: Some(ai_identity()),
        capabilities: &[Capability::DependencyResolver],
        native_update: false,
    };
    assert!(
        gate_native_adapter(&ai_context(DepOp::Update), &adapter, &CompatMode::Native).is_err()
    );
}

#[test]
fn resolver_capability_with_native_update_support_can_update() {
    let adapter = AdapterStub {
        identity: Some(ai_identity()),
        capabilities: &[Capability::DependencyResolver],
        native_update: true,
    };
    assert!(gate_native_adapter(&ai_context(DepOp::Update), &adapter, &CompatMode::Native).is_ok());
}
