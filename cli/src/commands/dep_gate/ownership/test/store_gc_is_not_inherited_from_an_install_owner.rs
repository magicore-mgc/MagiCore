//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn store_gc_is_not_inherited_from_an_install_owner() {
    for (core, ecosystem) in [
        ("ai", Some(eco::PYTHON)),
        ("app", Some(eco::FLUTTER)),
        ("lib", Some(eco::RUST)),
        ("game", Some(eco::BEVY)),
        ("iot", Some("esp32-rust")),
    ] {
        assert!(
            matches!(
                owner_for(&ctx(core, ecosystem, DepOp::Gc)),
                DepOwner::Unsupported
            ),
            "{core}/{ecosystem:?}/gc must not inherit install ownership"
        );
        assert!(gate(&ctx(core, ecosystem, DepOp::Gc), None, &native(), None).is_err());
    }

    for ecosystem in [Some(eco::JS), Some(eco::TS)] {
        assert!(matches!(
            owner_for(&ctx("web", ecosystem, DepOp::Gc)),
            DepOwner::Native
        ));
        assert!(gate(&ctx("web", ecosystem, DepOp::Gc), None, &native(), None).is_ok());
    }
}
