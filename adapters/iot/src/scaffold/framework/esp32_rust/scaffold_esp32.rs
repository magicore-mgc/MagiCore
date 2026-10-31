//! Write the current ESP32 Rust starter files.
//! Ghi các file khởi tạo ESP32 Rust hiện có.

use super::super::serialize_toml_string;
use super::chip_target::{chip_target, render_cargo_config};
use crate::framework::board_chip_for_framework;
use mgc_types::{MgError, MgResult};
use std::path::Path;

/// Pinned `esp-hal` release the starter targets.
/// Bản phát hành `esp-hal` được ghim cho starter.
const ESP_HAL_VERSION: &str = "1.2";

/// Starter source that links as a real bare-metal binary.
///
/// A `#[no_std]` binary needs a `#[panic_handler]`; without one the final link
/// fails with "`#[panic_handler]` function required, but not found". `#[main]`
/// from `esp-hal` also installs the runtime entry point the linker script
/// expects.
/// Binary `#[no_std]` bắt buộc có `#[panic_handler]`, thiếu sẽ fail khi link;
/// `#[main]` của `esp-hal` đồng thời cài entry point mà linker script cần.
const STARTER_MAIN: &str = "#![no_std]\n#![no_main]\n\nuse esp_hal::main;\n\n#[main]\nfn main() -> ! {\n    loop {}\n}\n\n#[panic_handler]\nfn panic(_info: &core::panic::PanicInfo<'_>) -> ! {\n    loop {\n        core::hint::spin_loop();\n    }\n}\n";

pub(in crate::scaffold) async fn scaffold_esp32(
    name: &str,
    board: &str,
    dir: &Path,
) -> MgResult<()> {
    // `esp-hal` gates every real dependency behind a chip feature, so a
    // manifest without one resolves an empty graph and `mgc install` cannot
    // produce a lock. Emit the feature that matches the selected board — the
    // same chip `mgc.toml` records, so the two never disagree.
    // (`esp-hal` đặt mọi dependency thật sau chip feature, nên manifest thiếu
    // feature sẽ resolve graph rỗng và `mgc install` không tạo được lock. Ghi
    // feature khớp board đã chọn — cùng chip mà `mgc.toml` ghi, không lệch.)
    let chip = board_chip_for_framework("esp32-rust", board).ok_or_else(|| MgError::Unsupported {
        core: "iot",
        capability: "scaffold_board",
        guidance: format!(
            "board '{board}' has no chip registered for framework 'esp32-rust'; cannot derive the esp-hal chip feature"
        ),
    })?;

    // Cargo defaults to the host triple when no target is pinned, so the
    // ESP32 crates are compiled for the developer machine and fail at link.
    // Resolve the chip target before any write so an unmapped chip is rejected
    // without leaving a half-built project behind.
    // Cargo mặc định build cho triple của host nếu không ghim target, khiến
    // crate ESP32 fail khi link. Lấy target chip trước khi ghi file để chip
    // chưa map bị từ chối mà không để lại project dở dang.
    let target = chip_target(&chip).ok_or_else(|| MgError::Unsupported {
        core: "iot",
        capability: "scaffold_board",
        guidance: format!(
            "chip '{chip}' for board '{board}' has no verified Rust target triple; refusing to guess a build target"
        ),
    })?;

    let cargo_name = serialize_toml_string(name);
    let cargo_chip = serialize_toml_string(&chip);
    let cargo = format!(
        "[package]\nname={}\nversion=\"0.1.0\"\nedition=\"2021\"\n\n[dependencies]\nesp-hal={{version=\"{}\",features=[{}]}}\n",
        cargo_name, ESP_HAL_VERSION, cargo_chip
    );
    std::fs::write(dir.join("Cargo.toml"), cargo)?;

    let mgc_name = serialize_toml_string(name);
    let mgc = format!(
        "name={}\nversion=\"0.1.0\"\necosystem=\"iot\"\n\n[iot]\nframework=\"esp32-rust\"\nboard=\"{}\"\n",
        mgc_name, board
    );
    std::fs::write(dir.join("mgc.toml"), mgc)?;

    // `.cargo/` is a directory Cargo reads from the project root.
    // `.cargo/` là thư mục Cargo đọc từ root của project.
    std::fs::create_dir_all(dir.join(".cargo"))?;
    std::fs::write(dir.join(".cargo/config.toml"), render_cargo_config(target))?;

    std::fs::create_dir_all(dir.join("src"))?;
    std::fs::write(dir.join("src/main.rs"), STARTER_MAIN)?;
    Ok(())
}
