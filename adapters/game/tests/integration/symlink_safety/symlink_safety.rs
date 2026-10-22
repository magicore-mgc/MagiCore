//! Symlink Safety behavior tests.
//! Test hành vi cho nhóm symlink safety.

use crate::integration::support::tmp;
use mgc_game_adapter::detect_engine;

#[cfg(unix)]
pub fn engine_detection_does_not_follow_external_mgc_config_symlink() {
    let project = tmp("external-mgc-config-link");
    let external = tmp("external-mgc-config-target");
    std::fs::write(external.join("mgc.toml"), "[game]\nengine = 'bevy'\n").unwrap();
    std::os::unix::fs::symlink(external.join("mgc.toml"), project.join("mgc.toml")).unwrap();

    assert_eq!(detect_engine(&project), None);
    let _ = std::fs::remove_dir_all(project);
    let _ = std::fs::remove_dir_all(external);
}
