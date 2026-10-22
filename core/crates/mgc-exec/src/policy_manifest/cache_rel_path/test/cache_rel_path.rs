use crate::policy_manifest::cache_rel_path;

#[test]
fn cache_path_stays_relative_to_the_configured_data_root() {
    assert_eq!(cache_rel_path().to_string_lossy(), "policy/exceptions.json");
}
