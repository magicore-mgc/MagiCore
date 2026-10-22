//! Tests for IoT framework command selection.

use std::path::Path;

use super::super::framework_command::iot_dev_command;

#[test]
fn iot_dev_esp32_uses_espflash_monitor() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-iot-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[package]\nname = \"demo\"\n").unwrap();
    let (cmd, args) = iot_dev_command(&dir).unwrap();
    assert_eq!(cmd, "espflash");
    assert_eq!(args, vec!["monitor"]);
}

#[test]
fn iot_dev_platformio_uses_pio() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-pio-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("platformio.ini"), "[env:esp32dev]\n").unwrap();
    let (cmd, args) = iot_dev_command(&dir).unwrap();
    assert_eq!(cmd, "pio");
    assert_eq!(args, vec!["run"]);
}

#[test]
fn iot_dev_unknown_framework_bails() {
    assert!(iot_dev_command(Path::new("/nonexistent")).is_err());
}
