//! Direct support, board aliases and default-board mappings.
//! Danh sách hỗ trợ trực tiếp, alias board và board mặc định.

pub(in crate::scaffold::processors::iot::implementation) fn supports(framework: &str) -> bool {
    matches!(
        framework,
        "platformio"
            | "firmware"
            | "zephyr"
            | "zephyr-arm"
            | "esp32-rust"
            | "esp32"
            | "esp32c3"
            | "esp32s3"
            | "esp32dev"
            | "nodemcu-32s"
            | "nrf52dk_nrf52832"
            | "stm32f4_disc"
    )
}

pub(in crate::scaffold::processors::iot::implementation) fn framework_for_board(
    board: &str,
) -> Option<&'static str> {
    match board {
        "esp32" | "esp32c3" | "esp32s3" => Some("esp32-rust"),
        "esp32dev" | "nodemcu-32s" => Some("platformio"),
        "nrf52dk_nrf52832" | "stm32f4_disc" => Some("zephyr-arm"),
        _ => None,
    }
}

pub(in crate::scaffold::processors::iot::implementation) fn default_board(
    framework: &str,
) -> Option<&'static str> {
    match framework {
        "esp32-rust" => Some("esp32c3"),
        "platformio" | "firmware" => Some("esp32dev"),
        "zephyr" | "zephyr-arm" => Some("nrf52dk_nrf52832"),
        _ => None,
    }
}
