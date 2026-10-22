//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn every_lib_native_lane_requires_its_concrete_owned_manifest_format() {
    let cases = [
        (eco::TS, "package-json"),
        (eco::RUST, "cargo-toml"),
        (eco::PYTHON, "pep621-native"),
        (eco::GO, "go-mod"),
        (eco::JAVA, "maven-pom"),
        (eco::DOTNET, "csproj"),
    ];
    for (ecosystem, format) in cases {
        for op in [DepOp::Install, DepOp::Add, DepOp::Remove, DepOp::Update] {
            assert!(
                matches!(
                    owner_for(&lib_ctx(ecosystem, None, op)),
                    DepOwner::Unsupported
                ),
                "lib/{ecosystem}/{op:?} without a manifest must be unsupported"
            );
            assert!(
                matches!(
                    owner_for(&lib_ctx(ecosystem, Some(format), op)),
                    DepOwner::Native
                ),
                "lib/{ecosystem}/{format}/{op:?} should remain native"
            );
        }
    }
    for unsupported in ["pyproject-unsupported", "requirements-txt", "foreign-lock"] {
        assert!(matches!(
            owner_for(&lib_ctx(eco::PYTHON, Some(unsupported), DepOp::Install)),
            DepOwner::Unsupported
        ));
    }
}
