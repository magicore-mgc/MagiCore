use super::super::store_profile;
use crate::spawn_profile::test::support::sample;

#[test]
fn creates_store_directory_and_writes_profile_document() {
    let dir = tempfile::TempDir::new().unwrap();
    let store = dir.path().join("profiles/nested");
    store_profile(&store, &sample()).unwrap();
    let path = store.join("next-dev-darwin.json");
    let bytes = std::fs::read(path).unwrap();
    assert!(bytes.ends_with(b"}\n") || bytes.ends_with(b"}"));
}
