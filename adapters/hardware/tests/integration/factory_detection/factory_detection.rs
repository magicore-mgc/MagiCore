//! Factory Detection behavior tests.
//! Detects marked Hardware projects and rejects unsupported or unrelated project markers.

use crate::integration::support::tmp;
use mgc_hardware_adapter::adapter_for;

pub fn adapter_for_returns_some_for_hardware_ecosystem() {
    let dir = tmp("hw");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"hardware\"\n").unwrap();
    assert!(adapter_for(&dir).is_some());
}

pub fn adapter_for_returns_some_for_game_ecosystem_cross_core() {
    // The hardware optimizer and benchmark are cross-core add-ons; game projects are supported too.
    // hardware optimizer/bench là cross-core add-on — game project cũng supported
    let dir = tmp("game-cross");
    std::fs::write(dir.join("mgc.toml"), "ecosystem = \"game\"\n").unwrap();
    assert!(
        adapter_for(&dir).is_some(),
        "cross-core: hardware optimizer can be added to game project"
    );
}

pub fn adapter_for_returns_none_for_dir_without_mgc_toml() {
    let dir = tmp("empty");
    assert!(adapter_for(&dir).is_none());
}

pub fn adapter_for_returns_none_for_non_mgc_ecosystem() {
    let dir = tmp("web-eco");
    // The Hardware adapter does not detect projects marked as web.
    // web ecosystem — hardware adapter không detect
    std::fs::write(dir.join("package.json"), r#"{"name":"web"}"#).unwrap();
    assert!(adapter_for(&dir).is_none());
}
