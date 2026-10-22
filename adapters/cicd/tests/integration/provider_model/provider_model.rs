//! Provider Model behavior tests.
//! Test hành vi cho nhóm provider model.

use mgc_cicd_adapter::CicdProvider;

pub fn cicd_provider_as_str_values() {
    assert_eq!(CicdProvider::GithubActions.as_str(), "github-actions");
    assert_eq!(CicdProvider::Gitlab.as_str(), "gitlab");
    assert_eq!(CicdProvider::CircleCi.as_str(), "circleci");
    assert_eq!(CicdProvider::Cloudflare.as_str(), "cloudflare");
    assert_eq!(CicdProvider::Aws.as_str(), "aws");
    assert_eq!(CicdProvider::Gcp.as_str(), "gcp");
    assert_eq!(CicdProvider::Argocd.as_str(), "argocd");
}
