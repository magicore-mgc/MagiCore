//! Public path smoke test; behavior tests are registered next to source owners.

use mgc_exec::spawn_profile::{SPAWN_PROFILE_SCHEMA, profile_file_name};

#[test]
fn profile_schema_and_name_are_available_from_public_module() {
    assert_eq!(SPAWN_PROFILE_SCHEMA, "spawn-profile-v1");
    assert_eq!(
        profile_file_name("next", "dev", "darwin"),
        "next-dev-darwin.json"
    );
}
