use super::*;
use crate::commands::core::dev::cicd::DeployTarget;

#[test]
fn configured_cloud_targets_are_explicitly_unsupported_without_vendor_cli() {
    let cf = DeployTarget {
        provider: "cloudflare".into(),
        stack: String::new(),
        region: String::new(),
    };
    let error = target_deploy_unavailable(&cf);
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

    let aws = DeployTarget {
        provider: "aws".into(),
        stack: "my-infra".into(),
        region: "ap-southeast-1".into(),
    };
    let error = target_deploy_unavailable(&aws);
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
    assert!(error.to_string().contains("my-infra"));
    assert!(error.to_string().contains("ap-southeast-1"));

    let no_stack = DeployTarget {
        provider: "aws".into(),
        stack: String::new(),
        region: String::new(),
    };
    assert!(
        target_deploy_unavailable(&no_stack)
            .to_string()
            .contains("no MagiCore-native provider engine")
    );
}
