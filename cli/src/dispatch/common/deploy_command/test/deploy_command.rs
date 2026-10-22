// Deploy route planning tests — verify feature-gated target selection.
// Test plan deploy — xác minh chọn target theo feature build.
use super::{DeployTarget, plan_target};

#[cfg(feature = "cicd")]
#[test]
fn compiled_cicd_lane_is_selected() {
    assert_eq!(plan_target(Some("cicd")).unwrap(), DeployTarget::Cicd);
}

#[cfg(all(not(feature = "cicd"), feature = "clo"))]
#[test]
fn missing_cicd_lane_returns_the_shared_error() {
    let actual = plan_target(Some("cicd")).unwrap_err();
    assert_eq!(
        actual.to_string(),
        crate::error::core_not_in_build("cicd").to_string()
    );
}

#[cfg(feature = "clo")]
#[test]
fn cloud_is_the_fallback_for_unset_or_other_core() {
    assert_eq!(plan_target(None).unwrap(), DeployTarget::Clo);
    assert_eq!(plan_target(Some("game")).unwrap(), DeployTarget::Clo);
}

#[cfg(not(feature = "clo"))]
#[test]
fn missing_cloud_lane_returns_the_shared_error() {
    let actual = plan_target(None).unwrap_err();
    assert_eq!(
        actual.to_string(),
        crate::error::core_not_in_build("clo").to_string()
    );
}
