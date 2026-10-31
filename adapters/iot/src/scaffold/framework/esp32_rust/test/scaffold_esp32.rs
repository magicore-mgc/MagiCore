#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Preserve the existing ESP32 Rust scaffold output contract.
//! Giữ contract output scaffold ESP32 Rust hiện có.

use crate::framework::IotFramework;
use crate::scaffold::scaffold_project;
use crate::scaffold::test::support::tmp;

#[tokio::test]
async fn test_scaffold_esp32() {
    let tmp = tmp();
    scaffold_project(IotFramework::Esp32Rust, "test", "esp32c3", tmp.path())
        .await
        .unwrap();
    assert!(tmp.path().join("Cargo.toml").exists());
    assert!(tmp.path().join("mgc.toml").exists());
}

/// The defect: `esp-hal = "1.2"` carried no chip feature, and `esp-hal` gates
/// every real dependency behind one, so `mgc install` resolved an empty graph
/// and refused to write a `Cargo.lock`. The generated manifest must enable the
/// selected board's chip feature, and it must be the same chip `mgc.toml`
/// records.
/// (Lỗi gốc: `esp-hal = "1.2"` thiếu chip feature nên `mgc install` resolve
/// graph rỗng và không ghi được lock. Manifest phải bật chip feature của board
/// đã chọn, đúng chip mà `mgc.toml` ghi.)
#[tokio::test]
async fn generated_manifest_enables_the_selected_board_chip_feature() {
    for board in ["esp32c3", "esp32s3", "esp32"] {
        let temp = tmp();
        scaffold_project(IotFramework::Esp32Rust, "firmware", board, temp.path())
            .await
            .unwrap();

        let cargo = std::fs::read_to_string(temp.path().join("Cargo.toml")).unwrap();
        let manifest: toml::Value = toml::from_str(&cargo).unwrap();
        let esp_hal = manifest
            .get("dependencies")
            .and_then(|dependencies| dependencies.get("esp-hal"))
            .expect("the starter must declare esp-hal");
        let features = esp_hal
            .get("features")
            .and_then(toml::Value::as_array)
            .expect("esp-hal must carry its chip feature");
        let features: Vec<&str> = features.iter().filter_map(toml::Value::as_str).collect();
        assert_eq!(
            features,
            vec![board],
            "{board}: the esp-hal feature must be the selected chip"
        );

        let mgc = std::fs::read_to_string(temp.path().join("mgc.toml")).unwrap();
        let mgc: toml::Value = toml::from_str(&mgc).unwrap();
        let recorded_board = mgc
            .get("iot")
            .and_then(|iot| iot.get("board"))
            .and_then(toml::Value::as_str);
        assert_eq!(
            recorded_board,
            Some(board),
            "{board}: mgc.toml must record the same board the chip feature came from"
        );
    }
}

/// The defect: the generator emitted `Cargo.toml` and `src/main.rs` but no
/// `.cargo/config.toml`, so Cargo built for the host triple and the ESP32
/// crates failed at link ("invalid Mach-O section specifier",
/// `portable-atomic` `compile_error!`). Every esp32-rust project must pin its
/// board target, and the chosen chips must map to distinct correct triples.
/// (Lỗi gốc: generator không ghi `.cargo/config.toml` nên Cargo build cho
/// triple host và crate ESP32 fail khi link. Mỗi project esp32-rust phải ghim
/// target của board, và các chip phải map đúng triple riêng.)
#[tokio::test]
async fn generated_project_pins_the_board_build_target() {
    let expected = [
        ("esp32c3", "riscv32imc-unknown-none-elf"),
        ("esp32s3", "xtensa-esp32s3-none-elf"),
        ("esp32", "xtensa-esp32-none-elf"),
    ];

    for (board, triple) in expected {
        let temp = tmp();
        scaffold_project(IotFramework::Esp32Rust, "firmware", board, temp.path())
            .await
            .unwrap();

        let path = temp.path().join(".cargo/config.toml");
        let config = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{board}: .cargo/config.toml missing: {error}"));
        let config: toml::Value = toml::from_str(&config).unwrap();
        assert_eq!(
            config
                .get("build")
                .and_then(|build| build.get("target"))
                .and_then(toml::Value::as_str),
            Some(triple),
            "{board}: Cargo must build for the board target, not the host"
        );

        // The target lives in exactly one place — the Cargo config Cargo reads
        // — so there is no second copy that can silently disagree.
        // (Target chỉ nằm một nơi — config Cargo mà Cargo đọc — nên không có
        // bản sao thứ hai lệch âm thầm.)

        // The runtime link only succeeds with the loader script applied.
        // Link runtime chỉ thành công khi áp dụng script loader.
        let rustflags = config
            .get("target")
            .and_then(|target| target.get(triple))
            .and_then(|target| target.get("rustflags"))
            .and_then(toml::Value::as_array)
            .expect("the board target must configure rustflags");
        let rustflags: Vec<&str> = rustflags.iter().filter_map(toml::Value::as_str).collect();
        assert!(
            rustflags.contains(&"link-arg=-Tlinkall.x"),
            "{board}: missing loader script, got {rustflags:?}"
        );
    }
}

/// A `#![no_std]` binary without `#[panic_handler]` cannot link, so the
/// starter must carry one; the same defect class as the missing target.
/// Binary `#![no_std]` thiếu `#[panic_handler]` không link được — cùng nhóm lỗi
/// với target bị thiếu.
#[tokio::test]
async fn generated_starter_defines_a_panic_handler() {
    let temp = tmp();
    scaffold_project(IotFramework::Esp32Rust, "firmware", "esp32c3", temp.path())
        .await
        .unwrap();

    let main = std::fs::read_to_string(temp.path().join("src/main.rs")).unwrap();
    assert!(
        main.contains("#[panic_handler]"),
        "the starter must define a panic handler:\n{main}"
    );
}

#[tokio::test]
async fn invalid_cargo_name_is_rejected_before_project_creation() {
    let temp = tmp();
    let project = temp.path().join("firmware");
    let project_name = "device\"\n[injected]\nowned = \"yes";
    let error = scaffold_project(IotFramework::Esp32Rust, project_name, "esp32c3", &project)
        .await
        .expect_err("invalid Cargo names must be rejected");

    let diagnostic = error.to_string();
    assert!(matches!(error, mgc_types::MgError::InvalidPackageName(_)));
    assert!(
        !diagnostic.contains(project_name),
        "diagnostic must not echo raw input"
    );
    assert!(
        !project.exists(),
        "validation must precede filesystem writes"
    );
}
