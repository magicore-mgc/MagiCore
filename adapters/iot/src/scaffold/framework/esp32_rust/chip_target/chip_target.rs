//! Map an ESP32 chip family to the Rust target triple Cargo must build and the
//! linker arguments that target needs.
//! Map họ chip ESP32 sang target triple Rust mà Cargo phải build kèm đối số
//! linker mà target đó cần.

/// Rust target triple plus its linker configuration for one chip.
/// Target triple Rust kèm cấu hình linker cho một chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::scaffold) struct ChipTarget {
    /// Fully qualified Rust target triple passed to `[build] target`.
    /// Target triple Rust đầy đủ đặt vào `[build] target`.
    pub(in crate::scaffold) triple: &'static str,
    /// Loader script the chip's runtime expects the linker to apply.
    /// Script loader mà runtime của chip cần linker áp dụng.
    pub(in crate::scaffold) linker_script: &'static str,
}

/// Loader script shipped by the ESP32 runtime crates.
/// Script loader do các crate runtime ESP32 cung cấp.
const LINKALL_SCRIPT: &str = "linkall.x";

/// Resolve the build target for an `esp-hal` chip feature id.
///
/// Triples follow the ESP32 Rust book's supported lists. ESP32-C3 is a
/// RISC-V **imc** core, so it needs `riscv32imc`; the `imac` variant belongs
/// to ESP32-C6/H2 and would not run on C3. Xtensa chips (ESP32, ESP32-S3) are
/// not upstream Rust targets and require the `espup` toolchain.
///
/// Trả `None` cho chip chưa được xác minh, để generator fail closed thay vì
/// đoán triple.
pub(in crate::scaffold) fn chip_target(chip: &str) -> Option<ChipTarget> {
    let triple = match chip {
        // ESP32-C2/C3: RV32IMC (no atomics, no double-precision float).
        // ESP32-C2/C3: RV32IMC (không atomic, không float độ chính xác kép).
        "esp32c2" | "esp32c3" => "riscv32imc-unknown-none-elf",
        // ESP32-C6/H2: RV32IMAC (adds the atomic extension).
        // ESP32-C6/H2: RV32IMAC (thêm tập lệnh atomic).
        "esp32c6" | "esp32h2" => "riscv32imac-unknown-none-elf",
        // ESP32-S2/S3 and classic ESP32: Xtensa, via the `espup` fork.
        // ESP32-S2/S3 và ESP32 cổ điển: Xtensa, qua fork `espup`.
        "esp32s2" => "xtensa-esp32s2-none-elf",
        "esp32s3" => "xtensa-esp32s3-none-elf",
        "esp32" => "xtensa-esp32-none-elf",
        _ => return None,
    };
    Some(ChipTarget {
        triple,
        linker_script: LINKALL_SCRIPT,
    })
}

/// Render the `.cargo/config.toml` that pins the build to the chip target.
///
/// Without this file Cargo builds for the host triple, and ESP32 crates fail
/// at link with host-object errors ("invalid Mach-O section specifier",
/// `portable-atomic` `compile_error!`). The `linkall.x` linker argument is
/// equally required: the bare link fails with undefined runtime symbols.
/// The triple is written raw (not a TOML quoted value) because it is already a
/// validated `&'static str` from this module's table.
///
/// Nếu thiếu file này Cargo build cho triple của host và ESP32 fail khi link;
/// đối số `linkall.x` cũng bắt buộc, thiếu sẽ lỗi symbol runtime chưa định nghĩa.
pub(in crate::scaffold) fn render_cargo_config(target: ChipTarget) -> String {
    format!(
        // Pin the board target — ghim target của board.
        "[build]\ntarget = \"{triple}\"\n\n\
         [target.{triple}]\nrustflags = [\"-C\", \"link-arg=-T{}\"]\n",
        target.linker_script,
        triple = target.triple,
    )
}
