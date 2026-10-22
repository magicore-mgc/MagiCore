use super::*;

#[test]
fn cloudflare_deploy_is_explicitly_unsupported_without_vendor_cli() {
    let error = provider_deploy_unavailable(mgc_cicd_adapter::CicdProvider::Cloudflare);
    assert!(
        error
            .to_string()
            .contains("no MagiCore-native provider engine")
    );
    assert!(
        error
            .to_string()
            .contains("no external provider CLI was started")
    );
}

#[test]
fn gcp_deploy_is_explicitly_unsupported_without_vendor_cli() {
    let error = provider_deploy_unavailable(mgc_cicd_adapter::CicdProvider::Gcp);
    assert!(
        error
            .to_string()
            .contains("no MagiCore-native provider engine")
    );
    assert!(
        error
            .to_string()
            .contains("no external provider CLI was started")
    );
}

#[test]
fn ci_only_providers_bail() {
    assert!(
        provider_deploy_unavailable(mgc_cicd_adapter::CicdProvider::GithubActions)
            .to_string()
            .contains("CI-only")
    );
    assert!(
        provider_deploy_unavailable(mgc_cicd_adapter::CicdProvider::Gitlab)
            .to_string()
            .contains("CI-only")
    );
    assert!(
        provider_deploy_unavailable(mgc_cicd_adapter::CicdProvider::CircleCi)
            .to_string()
            .contains("CI-only")
    );
    assert!(
        provider_deploy_unavailable(mgc_cicd_adapter::CicdProvider::Aws)
            .to_string()
            .contains("no MagiCore-native provider engine")
    );
    assert!(
        provider_deploy_unavailable(mgc_cicd_adapter::CicdProvider::Argocd)
            .to_string()
            .contains("server-side")
    );
}
