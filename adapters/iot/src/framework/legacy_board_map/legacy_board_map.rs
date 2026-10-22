//! Legacy board-map snapshot retained for source compatibility.
//! Snapshot map board cũ được giữ để tương thích mã nguồn.

/// Static board triples retained for callers using the legacy API.
/// Bộ ba board tĩnh giữ lại cho caller dùng API cũ.
pub const KNOWN_BOARDS: &[(&str, &str, &str)] = &[
    ("esp32", "esp32", "xtensa-esp32-none-elf"),
    ("esp32c3", "esp32c3", "riscv32imac-unknown-none-elf"),
    ("esp32s3", "esp32s3", "xtensa-esp32s3-none-elf"),
    ("esp32dev", "esp32", "xtensa-esp32-none-elf"),
    ("nodemcu-32s", "esp32", "xtensa-esp32-none-elf"),
    ("nrf52dk_nrf52832", "nrf52", "thumbv7em-none-eabihf"),
    ("stm32f4_disc", "stm32", "thumbv7em-none-eabihf"),
];

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
