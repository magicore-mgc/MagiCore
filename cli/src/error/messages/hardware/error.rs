//! Hardware target and device diagnostics.
//! Lỗi target phần cứng và thiết bị.

use anyhow::Error;
use anyhow::anyhow;

/// Android device discovery or launch failed.
/// Không thể truy vấn hoặc khởi chạy ứng dụng trên thiết bị Android.
pub fn android_device_command_failed(e: impl std::fmt::Display) -> Error {
    anyhow!("Android device command failed: {e}")
}

/// flash only supports esp32-rust currently
pub fn flash_framework_unsupported(framework: &str) -> Error {
    anyhow!(
        "'mgc flash' currently supports only the esp32-rust framework (you are using {framework}) — platformio/zephyr flash is P2"
    )
}

/// unknown hardware package for add-hardware
pub fn unknown_hardware_package(pkg: &str) -> Error {
    anyhow!("unknown hardware package '{pkg}' (optimizer | bench)")
}

/// Hardware template directory is absent — nothing to remove.
/// (Thư mục template hardware không có — không có gì để gỡ.)
pub fn hardware_template_not_materialized(pkg: &str, root: &std::path::Path) -> Error {
    anyhow!(
        "hardware template '{pkg}' is not materialized in '{}' — nothing to remove",
        root.display()
    )
}

/// Hardware template directory could not be removed.
/// (Không gỡ được thư mục template hardware.)
pub fn hardware_template_remove_failed(pkg: &str, reason: &dyn std::fmt::Display) -> Error {
    anyhow!("cannot remove hardware template '{pkg}': {reason}")
}

pub fn espflash_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "espflash failed: {e} — install espflash first (`cargo install espflash`) and ensure it is in PATH"
    )
}

pub fn no_board_specified(boards: &str) -> Error {
    anyhow!(
        "No board specified — add `[iot] board = \"<id>\"` to mgc.toml (known boards: {boards}) or pass `mgc flash --board <id>`"
    )
}

pub fn unsupported_board(board: &str, boards: &str) -> Error {
    anyhow!("Unsupported board '{board}' — known boards: {boards}")
}

pub fn fw_path_not_utf8(elf: &std::path::Path) -> Error {
    anyhow!("firmware path is not valid UTF-8: {}", elf.display())
}

pub fn unknown_hardware_framework(fw: &str) -> Error {
    anyhow!(
        "unknown hardware framework '{fw}' (optimizer | bench) — templates materialize directly into the project; no separate project scaffolding"
    )
}

pub fn no_hardware_framework() -> Error {
    anyhow!("no hardware framework selected")
}
