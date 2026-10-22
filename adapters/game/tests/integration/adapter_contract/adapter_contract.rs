//! Adapter Contract behavior tests.
//! Test hành vi cho nhóm adapter contract.

use crate::integration::support::tmp;
use mgc_game_adapter::adapter_for;
use mgc_types::capabilities::{CoreIdent, ProjectDetector};

pub fn adapter_name_and_ecosystem() {
    let dir = tmp("name-eco");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    assert_eq!(a.name(), "game");
    assert_eq!(format!("{:?}", a.ecosystem()), "Game");
}

pub fn can_handle_returns_true_for_game_project() {
    let dir = tmp("ch-true");
    std::fs::write(dir.join("project.godot"), "[application]\n").unwrap();
    let a = adapter_for(&dir).unwrap();
    assert!(a.can_handle(&dir));
}
