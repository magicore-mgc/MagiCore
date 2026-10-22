//! Symlink Safety behavior tests.
//! Test hành vi cho nhóm symlink safety.

use crate::integration::support::tmp;
use mgc_iot_adapter::{adapter_for, detect_framework};

#[cfg(unix)]
pub fn framework_detection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-mgc-config-link");
    let external = tmp("external-mgc-config-target");
    std::fs::write(
        external.join("mgc.toml"),
        "[iot]\nframework = 'platformio'\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    assert_eq!(detect_framework(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}

#[cfg(unix)]
pub fn board_selection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-board-config-link");
    let external = tmp("external-board-config-target");
    std::fs::write(
        project.join("platformio.ini"),
        "[env:local]\nplatform = espressif32\n",
    )
    .unwrap();
    std::fs::write(
        external.join("mgc.toml"),
        "[iot]\nboard = 'external-board'\n",
    )
    .unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    let adapter = adapter_for(&project).expect("PlatformIO manifest identifies the adapter");
    assert_eq!(adapter.board(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}
