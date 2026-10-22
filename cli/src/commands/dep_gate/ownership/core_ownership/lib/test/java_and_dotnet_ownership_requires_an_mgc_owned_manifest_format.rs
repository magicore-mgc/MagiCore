//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn java_and_dotnet_ownership_requires_an_mgc_owned_manifest_format() {
    for op in [DepOp::Install, DepOp::Add, DepOp::Remove, DepOp::Update] {
        assert!(matches!(
            owner_for(&lib_ctx(eco::JAVA, None, op)),
            DepOwner::Unsupported
        ));
        assert!(matches!(
            owner_for(&lib_ctx(eco::JAVA, Some("gradle"), op)),
            DepOwner::Unsupported
        ));
        assert!(matches!(
            owner_for(&lib_ctx(eco::JAVA, Some("maven-pom"), op)),
            DepOwner::Native
        ));
        assert!(matches!(
            owner_for(&lib_ctx(eco::DOTNET, None, op)),
            DepOwner::Unsupported
        ));
        assert!(matches!(
            owner_for(&lib_ctx(eco::DOTNET, Some("csproj"), op)),
            DepOwner::Native
        ));
    }
}
