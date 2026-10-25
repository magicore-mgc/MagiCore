//! Dispatch only answers for pairs that have a registered initializer.
//! Điều phối chỉ trả lời cho cặp đã đăng ký initializer.

use super::super::{record_for, try_run};

#[test]
fn a_registered_pair_reports_a_record() {
    assert!(record_for("game", "godot").is_some());
    assert!(record_for("iot", "platformio").is_some());
    assert!(record_for("clo", "terraform").is_some());
}

#[test]
fn an_unregistered_pair_does_not_run_anything() {
    let dir = tempfile::tempdir().unwrap();
    let ran = try_run("game", "unity-no-cli", dir.path()).expect("no record is not an error");
    assert!(!ran, "an unregistered framework must not run a program");
}
