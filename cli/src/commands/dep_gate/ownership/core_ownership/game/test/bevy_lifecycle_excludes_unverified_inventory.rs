//! Bevy dependency owner — owner dependency Bevy.
//! Only implemented lifecycle verbs are native; list has no installed-state evidence.
//! Chỉ verb lifecycle đã triển khai là native; list chưa có bằng chứng trạng thái cài.

use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco, owner_for};

#[test]
fn bevy_cargo_lifecycle_is_native_but_list_is_not() {
    for operation in [DepOp::Install, DepOp::Add, DepOp::Update, DepOp::Remove] {
        let context = DepContext::new("game", Some(eco::BEVY), None, None, operation)
            .with_manifest_format(Some("cargo-toml"));
        assert!(matches!(owner_for(&context), DepOwner::Native));
    }
    let list = DepContext::new("game", Some(eco::BEVY), None, None, DepOp::List)
        .with_manifest_format(Some("cargo-toml"));
    assert!(matches!(owner_for(&list), DepOwner::Unsupported));
}
