#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Preserve the existing PlatformIO scaffold output contract.
//! Giữ contract output scaffold PlatformIO hiện có.

use crate::framework::IotFramework;
use crate::scaffold::scaffold_project;
use crate::scaffold::test::support::tmp;

#[tokio::test]
async fn test_scaffold_platformio() {
    let tmp = tmp();
    scaffold_project(IotFramework::Platformio, "test", "esp32dev", tmp.path())
        .await
        .unwrap();
    assert!(tmp.path().join("platformio.ini").exists());
}

#[tokio::test]
async fn project_name_cannot_inject_an_mgc_table() {
    let temp = tmp();
    let project_name = "device\"\n[injected]\nowned = \"yes";
    scaffold_project(
        IotFramework::Platformio,
        project_name,
        "esp32dev",
        temp.path(),
    )
    .await
    .unwrap();

    let mgc: toml::Value =
        toml::from_str(&std::fs::read_to_string(temp.path().join("mgc.toml")).unwrap()).unwrap();

    assert_eq!(mgc["name"].as_str(), Some(project_name));
    assert!(mgc.get("injected").is_none());
}
