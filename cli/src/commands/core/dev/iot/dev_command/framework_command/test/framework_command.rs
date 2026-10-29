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

/// PAR-06: `mgc dev` must not spawn the build-shaped `pio run` that `mgc build`
/// refuses, so both routes enforce one policy.
/// (PAR-06: `mgc dev` không được spawn `pio run` dạng build mà `mgc build` từ
/// chối, nên cả hai đường cùng một chính sách.)
#[test]
fn iot_dev_platformio_refuses_the_build_command() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-pio-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("platformio.ini"), "[env:esp32dev]\n").unwrap();
    let error = iot_dev_command(&dir).expect_err("pio run must be refused like in build");
    let message = error.to_string();
    assert!(message.contains("platformio"), "{message}");
    assert!(
        message.contains("monitoring is supported"),
        "the refusal must say what is supported: {message}"
    );
}

/// The same policy applies to Zephyr's `west build`.
/// (Chính sách tương tự áp cho `west build` của Zephyr.)
#[test]
fn iot_dev_zephyr_refuses_the_build_command() {
    let dir = std::env::temp_dir().join(format!("mgc-dev-west-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("west.yml"), "manifest:\n  version: \"0.10\"\n").unwrap();
    let error = iot_dev_command(&dir).expect_err("west build must be refused like in build");
    assert!(error.to_string().contains("zephyr"), "{error}");
}

#[test]
fn iot_dev_unknown_framework_bails() {
    assert!(iot_dev_command(Path::new("/nonexistent")).is_err());
}
