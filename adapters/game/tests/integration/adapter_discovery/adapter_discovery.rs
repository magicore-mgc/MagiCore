//! Adapter Discovery behavior tests.
//! Test hành vi cho nhóm adapter discovery.

use crate::integration::support::tmp;
use mgc_game_adapter::adapter_for;

pub fn adapter_for_returns_some_for_godot() {
    let dir = tmp("af-godot");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    assert_eq!(a.engine(), "godot");
}

pub fn adapter_for_returns_none_for_plain_dir() {
    let dir = tmp("af-none");
    assert!(adapter_for(&dir).is_none());
}
