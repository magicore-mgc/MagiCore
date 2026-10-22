#![cfg(test)]

//! Keep the public engine and detection routes available.
//! Giữ các route engine và detector công khai sẵn có.

use crate::engine::{GameEngine, detect_engine};
use tempfile::tempdir;

#[test]
fn stable_engine_facade_exports_remain_available() {
    let project = tempdir().expect("temporary project directory should be created");
    let _: Option<GameEngine> = detect_engine(project.path());
}
