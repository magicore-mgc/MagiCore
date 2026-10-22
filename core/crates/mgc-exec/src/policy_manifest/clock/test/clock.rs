use crate::policy_manifest::now_unix_secs;

#[test]
fn unix_time_source_never_returns_a_negative_value() {
    assert!(now_unix_secs() >= 0);
}
