use super::super::SPAWN_PROFILE_SCHEMA;

#[test]
fn schema_identifier_is_stable() {
    assert_eq!(SPAWN_PROFILE_SCHEMA, "spawn-profile-v1");
}
