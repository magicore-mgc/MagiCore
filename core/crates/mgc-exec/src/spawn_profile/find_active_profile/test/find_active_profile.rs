use super::super::find_active_profile;
use crate::spawn_profile::store_profile;
use crate::spawn_profile::test::support::sample;

// Round-trip store then exact-tuple lookup hits.
#[test]
fn roundtrip_lookup_hits() {
    let dir = tempfile::TempDir::new().unwrap();
    store_profile(dir.path(), &sample()).unwrap();
    let found = find_active_profile(
        dir.path(),
        "next",
        "16.3.8",
        "darwin",
        "dev",
        Some("blake3-deadbeef"),
    )
    .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().allowed_pm_invocations.len(), 1);
}

// Missing profile means level-2 tripwire-only (None, not an error).
#[test]
fn missing_profile_is_none() {
    let dir = tempfile::TempDir::new().unwrap();
    assert_eq!(
        find_active_profile(dir.path(), "next", "16.3.8", "darwin", "dev", None).unwrap(),
        None
    );
}

// Version drift and lock drift both expire the profile.
#[test]
fn drift_expires_profile() {
    let dir = tempfile::TempDir::new().unwrap();
    store_profile(dir.path(), &sample()).unwrap();
    assert_eq!(
        find_active_profile(
            dir.path(),
            "next",
            "16.4.0",
            "darwin",
            "dev",
            Some("blake3-deadbeef")
        )
        .unwrap(),
        None
    );
    assert_eq!(
        find_active_profile(
            dir.path(),
            "next",
            "16.3.8",
            "darwin",
            "dev",
            Some("blake3-other")
        )
        .unwrap(),
        None
    );
    assert_eq!(
        find_active_profile(dir.path(), "next", "16.3.8", "darwin", "dev", None).unwrap(),
        None
    );
}

// Unknown schema fails closed.
#[test]
fn unknown_schema_rejected() {
    let dir = tempfile::TempDir::new().unwrap();
    store_profile(dir.path(), &sample()).unwrap();
    let path = dir.path().join("next-dev-darwin.json");
    let mut text = std::fs::read_to_string(&path).unwrap();
    text = text.replace(
        crate::spawn_profile::SPAWN_PROFILE_SCHEMA,
        "spawn-profile-v99",
    );
    std::fs::write(&path, text).unwrap();
    assert!(
        find_active_profile(
            dir.path(),
            "next",
            "16.3.8",
            "darwin",
            "dev",
            Some("blake3-deadbeef")
        )
        .is_err()
    );
}
