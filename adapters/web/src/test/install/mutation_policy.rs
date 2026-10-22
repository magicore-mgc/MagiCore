//! Web mutation-policy integration test.
//! Test tích hợp policy mutation Web.

use super::*;

#[tokio::test]
async fn direct_web_adapter_mutations_fail_closed_without_touching_manifest() {
    let dir = tempdir_real().unwrap();
    let manifest = r#"{"name":"demo","version":"1.0.0","dependencies":{"left-pad":"1.0.0"}}"#;
    std::fs::write(dir.path().join("package.json"), manifest).unwrap();
    let adapter = WebAdapter::new().unwrap();
    let name = PackageName::new("left-pad").unwrap();
    let range = VersionRange::parse("^1.0.0").unwrap();

    assert!(
        adapter
            .add(dir.path(), &name, Some(&range), AddOptions::default())
            .await
            .is_err()
    );
    assert!(adapter.remove(dir.path(), &name).await.is_err());
    assert!(adapter.update(dir.path(), Some(&name)).await.is_err());
    assert_eq!(
        std::fs::read_to_string(dir.path().join("package.json")).unwrap(),
        manifest
    );
    assert!(!dir.path().join(".magicore").exists());
}
