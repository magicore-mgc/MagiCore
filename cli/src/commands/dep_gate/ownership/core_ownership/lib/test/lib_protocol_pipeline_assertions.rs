//! Lib ownership assertions — assertion ownership Lib.
//! Keeps the existing assertion statements grouped by lifecycle concern.
//! Giữ assertion hiện có, nhóm theo phần lifecycle.

use crate::commands::dep_gate::test_support::*;

pub(super) fn assert_protocol_pipeline() {
    assert!(matches!(
        owner_for(&ctx("lib", Some(eco::TS), DepOp::Install)),
        DepOwner::Native
    ));
    assert!(
        gate(
            &ctx("lib", Some(eco::TS), DepOp::Install),
            None,
            &native(),
            None
        )
        .is_ok()
    );
    // Protocol lanes have the native pipeline; each edit still needs its exact manifest-aware cell.
    // Pipeline protocol dùng engine native; mỗi mutation vẫn cần cell manifest cụ thể.
    for lang in [eco::RUST, eco::PYTHON, eco::GO] {
        assert!(
            gate(
                &ctx("lib", Some(lang), DepOp::Install),
                None,
                &native(),
                None
            )
            .is_ok(),
            "lib[{lang}] install is native (no toolchain spawn)"
        );
        assert!(
            gate(
                &ctx("lib", Some(lang), DepOp::Resolve),
                None,
                &native(),
                None
            )
            .is_ok(),
            "lib[{lang}] resolve is native"
        );
    }
    for (lang, format) in [(eco::JAVA, "maven-pom"), (eco::DOTNET, "csproj")] {
        for op in [DepOp::Install, DepOp::Add, DepOp::Remove, DepOp::Update] {
            assert!(matches!(
                owner_for(&lib_ctx(lang, Some(format), op)),
                DepOwner::Native
            ));
        }
    }
}
