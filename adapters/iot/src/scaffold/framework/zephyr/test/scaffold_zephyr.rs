#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Preserve the current Zephyr scaffold output bytes.
//! Giữ nguyên byte output scaffold Zephyr hiện có.

use crate::framework::IotFramework;
use crate::scaffold::scaffold_project;
use crate::scaffold::test::support::tmp;

#[tokio::test]
async fn test_scaffold_zephyr() {
    let temp = tmp();
    scaffold_project(
        IotFramework::Zephyr,
        "firmware",
        "nrf52dk_nrf52832",
        temp.path(),
    )
    .await
    .unwrap();

    assert_eq!(
        std::fs::read_to_string(temp.path().join("west.yml")).unwrap(),
        "manifest:\n  projects:\n    - name: zephyr\n      url: https://github.com/zephyrproject-rtos/zephyr\n"
    );
    let mgc = std::fs::read_to_string(temp.path().join("mgc.toml")).unwrap();
    assert!(mgc.contains("framework=\"zephyr\""));
    assert!(mgc.contains("board=\"nrf52dk_nrf52832\""));
}

#[tokio::test]
async fn project_name_cannot_inject_an_mgc_table() {
    let temp = tmp();
    let project_name = "device\"\n[injected]\nowned = \"yes";
    scaffold_project(
        IotFramework::Zephyr,
        project_name,
        "nrf52dk_nrf52832",
        temp.path(),
    )
    .await
    .unwrap();

    let mgc: toml::Value =
        toml::from_str(&std::fs::read_to_string(temp.path().join("mgc.toml")).unwrap()).unwrap();

    assert_eq!(mgc["name"].as_str(), Some(project_name));
    assert!(mgc.get("injected").is_none());
}
