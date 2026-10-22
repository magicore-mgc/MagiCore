//! Typed error for an unverified ESP-HAL board.
//! Lỗi typed cho board ESP-HAL chưa xác minh.

#[derive(Debug, thiserror::Error)]
pub(in crate::scaffold::processors::iot::implementation::esp32_rust::implementation) enum IotEsp32Error
{
    #[error("No verified esp-hal feature for board '{board}'")]
    UnverifiedEspHalBoard { board: String },
}
