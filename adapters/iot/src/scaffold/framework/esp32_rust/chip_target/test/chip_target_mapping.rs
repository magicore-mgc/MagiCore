#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Chip-to-target mapping must be exact per chip and fail closed otherwise.
//! Mapping chip sang target phải chính xác theo chip và fail closed nếu thiếu.

use super::super::{chip_target, render_cargo_config};

#[test]
fn esp32c3_maps_to_the_riscv_imc_triple() {
    let target = chip_target("esp32c3").expect("esp32c3 must have a verified target");
    assert_eq!(
        target.triple,
        "riscv32imc-unknown-none-elf",
        // ESP32-C3 is RV32IMC; `riscv32imac` belongs to ESP32-C6/H2.
        // ESP32-C3 là RV32IMC; `riscv32imac` thuộc về ESP32-C6/H2.
    );
    assert_eq!(target.linker_script, "linkall.x");
}

#[test]
fn xtensa_chips_keep_their_own_triples() {
    let triples = ["esp32", "esp32s2", "esp32s3"]
        .into_iter()
        .map(|chip| {
            chip_target(chip)
                .unwrap_or_else(|| panic!("{chip} must have a verified target"))
                .triple
        })
        .collect::<Vec<_>>();
    assert_eq!(
        triples,
        [
            "xtensa-esp32-none-elf",
            "xtensa-esp32s2-none-elf",
            "xtensa-esp32s3-none-elf",
        ]
    );
}

#[test]
fn riscv_atomic_chips_use_the_imac_triple() {
    for chip in ["esp32c6", "esp32h2"] {
        assert_eq!(
            chip_target(chip).unwrap().triple,
            "riscv32imac-unknown-none-elf",
            "{chip} implements the RISC-V atomic extension"
        );
    }
}

#[test]
fn unmapped_chip_fails_closed_instead_of_guessing() {
    for chip in ["", "esp32c61", "unknown", "riscv32imac-unknown-none-elf"] {
        assert!(
            chip_target(chip).is_none(),
            // A guess would silently produce a project that cannot build.
            // Đoán bừa sẽ tạo project không build được mà không báo.
            "{chip} must not inherit another chip's target"
        );
    }
}

#[test]
fn rendered_config_pins_build_target_and_linker_script() {
    let target = chip_target("esp32c3").unwrap();
    let config = render_cargo_config(target);

    let parsed: toml::Value =
        toml::from_str(&config).expect("generated Cargo config is valid TOML");
    assert_eq!(
        parsed
            .get("build")
            .and_then(|build| build.get("target"))
            .and_then(toml::Value::as_str),
        Some("riscv32imc-unknown-none-elf"),
        // Cargo reads this key instead of defaulting to the host triple.
        // Cargo đọc key này thay vì mặc định về triple của host.
    );
    let rustflags = parsed
        .get("target")
        .and_then(|target| target.get("riscv32imc-unknown-none-elf"))
        .and_then(|target| target.get("rustflags"))
        .and_then(toml::Value::as_array)
        .expect("the chip target must configure rustflags");
    let rustflags: Vec<&str> = rustflags.iter().filter_map(toml::Value::as_str).collect();
    assert_eq!(rustflags, ["-C", "link-arg=-Tlinkall.x"]);
}
