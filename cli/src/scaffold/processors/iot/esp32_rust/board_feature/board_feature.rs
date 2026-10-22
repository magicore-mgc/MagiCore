//! Map verified boards to ESP-HAL Cargo features.
//! Ánh xạ board đã xác minh sang Cargo feature ESP-HAL.

use anyhow::Result;

use super::super::error::IotEsp32Error;

pub(in crate::scaffold::processors::iot::implementation::esp32_rust::implementation) fn for_board(
    board: &str,
) -> Result<&'static str> {
    match board {
        "esp32" => Ok("esp32"),
        "esp32c3" => Ok("esp32c3"),
        "esp32s3" => Ok("esp32s3"),
        other => Err(IotEsp32Error::UnverifiedEspHalBoard {
            board: other.to_owned(),
        }
        .into()),
    }
}
