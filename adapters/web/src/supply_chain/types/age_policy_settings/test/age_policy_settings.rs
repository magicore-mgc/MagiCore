//! Default contract for normalized release-age settings.
//! Hợp đồng mặc định của policy tuổi phát hành đã chuẩn hóa.

use super::super::AgePolicySettings;

#[test]
fn default_policy_has_no_override_and_keeps_missing_time_blocked() {
    assert_eq!(
        AgePolicySettings::default(),
        AgePolicySettings {
            min_age_secs: None,
            allow_missing_time: false,
        }
    );
}
