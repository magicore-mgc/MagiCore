//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

use crate::commands::dep_gate::test_support::*;

#[test]
fn cloud_list_is_native_only_for_web_backed_cdk_or_pulumi_manifests() {
    for framework in ["cdk", "pulumi"] {
        let context = DepContext::new("clo", Some(eco::JS), Some(framework), None, DepOp::List)
            .with_manifest_format(Some("package-json"));
        assert!(matches!(owner_for(&context), DepOwner::Native));
    }
    for (ecosystem, framework) in [
        (Some("terraform"), Some("terraform")),
        (Some("cloudflare"), Some("cloudflare")),
        (None, Some("pulumi")),
    ] {
        let context = DepContext::new("clo", ecosystem, framework, None, DepOp::List);
        assert!(matches!(owner_for(&context), DepOwner::Unsupported));
    }
}
