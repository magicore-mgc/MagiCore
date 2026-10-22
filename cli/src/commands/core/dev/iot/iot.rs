use super::{chip, find_elf, resolve_target, run_tool};
use anyhow::Result;
use mgc_ui::info;
use std::path::PathBuf;

fn project_root() -> Result<PathBuf> {
    crate::commands::core::shared::core_project_root("iot")
}

/// Build + flash firmware esp32 (Q16). Board đọc từ arg hoặc mgc.toml `[iot] board`;
/// fail-closed nếu không xác định target (00-index §3, 04 §3).
pub async fn flash(board_override: Option<&str>, skip_build: bool) -> Result<()> {
    let root = project_root()?;
    let adapter = mgc_iot_adapter::adapter_for(&root)
        .ok_or_else(|| crate::error::no_framework_detected("IoT", &root))?;

    if adapter.framework() != "esp32-rust" {
        return Err(crate::error::flash_framework_unsupported(
            adapter.framework(),
        ));
    }

    let board = board_override
        .map(str::to_string)
        .or_else(|| adapter.board(&root))
        .ok_or_else(|| {
            let boards = mgc_iot_adapter::boards_for_framework(adapter.framework())
                .iter()
                .map(|board| board.id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            crate::error::no_board_specified(&boards)
        })?;

    let explicit_target = adapter.target(&root);
    let target = resolve_target(adapter.framework(), &board, explicit_target.as_deref())?;

    info(&format!(
        "Board: {board} ({}), target: {target}",
        chip(&board)
    ));

    if !skip_build {
        info("Building firmware (cargo build --release)...");
        let build_args = [
            "build",
            "--release",
            "--locked",
            "--offline",
            "--target",
            &target,
        ];
        run_tool(&root, "cargo", &build_args)?;
    }

    let elf = find_elf(&root, &target)?;
    let elf_str = elf
        .to_str()
        .ok_or_else(|| crate::error::fw_path_not_utf8(&elf))?;
    info(&format!("Flashing firmware: espflash flash {elf_str}"));
    let flash_args = ["flash", elf_str];
    run_tool(&root, "espflash", &flash_args).map_err(|e| crate::error::espflash_failed(&e))
}
