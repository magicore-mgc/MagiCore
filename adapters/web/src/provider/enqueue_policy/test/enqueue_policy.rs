//! Required host/foreign platform edge admission / nhận edge platform host và foreign.

use crate::provider::NpmDependencyProvider;
use mgc_resolver::DependencyProvider;
use mgc_types::PackageName;

#[tokio::test]
async fn test_required_platform_binary_edges_skip_foreign_targets() {
    let current_target = format!(
        "{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    );
    let foreign_target = if current_target == "linux-x64" {
        "darwin-arm64"
    } else {
        "linux-x64"
    };
    let provider = NpmDependencyProvider::new("https://registry.example", None, None);
    let current = mgc_resolver::ResolvedDep {
        package: PackageName::new(format!("@typescript/typescript-{current_target}")).unwrap(),
        spec: "7.0.2".to_string(),
        optional: false,
        peer: false,
    };
    let foreign = mgc_resolver::ResolvedDep {
        package: PackageName::new(format!("@typescript/typescript-{foreign_target}")).unwrap(),
        spec: "7.0.2".to_string(),
        optional: false,
        peer: false,
    };

    assert!(provider.should_enqueue(&current).await.unwrap());
    assert!(!provider.should_enqueue(&foreign).await.unwrap());
}
