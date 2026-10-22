//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

#[test]
fn cloud_cdk_and_pulumi_dependency_lifecycle_is_native_only_with_js_manifest() {
    use crate::commands::dep_gate::{DepContext, DepOp, DepOwner, eco, owner_for};

    for framework in ["cdk", "pulumi"] {
        for op in [
            DepOp::Install,
            DepOp::Add,
            DepOp::Remove,
            DepOp::Update,
            DepOp::List,
        ] {
            let context = DepContext::new("clo", Some(eco::JS), Some(framework), None, op)
                .with_manifest_format(Some("package-json"));
            assert!(
                matches!(owner_for(&context), DepOwner::Native),
                "clo/{framework} {} must use the embedded MGC JS dependency engine",
                op.as_str()
            );
        }
        let missing_manifest =
            DepContext::new("clo", Some(eco::JS), Some(framework), None, DepOp::Install);
        assert!(matches!(
            owner_for(&missing_manifest),
            DepOwner::Unsupported
        ));
    }

    for framework in ["terraform", "cloudflare"] {
        let context = DepContext::new("clo", None, Some(framework), None, DepOp::Install);
        assert!(matches!(owner_for(&context), DepOwner::Unsupported));
    }
}
