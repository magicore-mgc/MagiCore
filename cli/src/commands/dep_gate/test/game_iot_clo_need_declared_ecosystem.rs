//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn game_iot_clo_need_declared_ecosystem() {
    for (core, ecosystem, framework) in [
        ("game", eco::BEVY, eco::BEVY),
        ("iot", "esp32-rust", "esp32-rust"),
    ] {
        for op in [DepOp::Install, DepOp::Add, DepOp::Update, DepOp::Remove] {
            let without_manifest =
                DepContext::new(core, Some(ecosystem), Some(framework), None, op);
            assert!(
                matches!(owner_for(&without_manifest), DepOwner::Unsupported),
                "{core}/{framework}/{op:?} without Cargo.toml must be unsupported"
            );
            let with_manifest = without_manifest.with_manifest_format(Some("cargo-toml"));
            assert!(
                matches!(owner_for(&with_manifest), DepOwner::Native),
                "{core}/{framework}/{op:?} with Cargo.toml is native"
            );
        }
    }
    let bevy =
        ctx("game", Some(eco::BEVY), DepOp::Install).with_manifest_format(Some("cargo-toml"));
    let esp32 =
        ctx("iot", Some("esp32-rust"), DepOp::Install).with_manifest_format(Some("cargo-toml"));
    assert!(gate(&bevy, None, &native(), None).is_ok());
    assert!(gate(&ctx("game", None, DepOp::Install), None, &native(), None).is_err());
    assert!(matches!(owner_for(&esp32), DepOwner::Native));
    for fw in ["platformio", "zephyr"] {
        assert!(
            gate(&ctx("iot", Some(fw), DepOp::Install), None, &native(), None).is_err(),
            "iot[{fw}] has no adapter-owned install lane; compatibility is not silently opened"
        );
    }
    assert!(gate(&ctx("iot", None, DepOp::Install), None, &native(), None).is_err());
    assert!(
        gate(
            &ctx("clo", Some(eco::TERRAFORM), DepOp::Install),
            Some("terraform"),
            &native(),
            None
        )
        .is_err()
    );
    assert!(
        gate(
            &ctx("clo", None, DepOp::Install),
            Some("terraform"),
            &native(),
            None
        )
        .is_err()
    );
}
