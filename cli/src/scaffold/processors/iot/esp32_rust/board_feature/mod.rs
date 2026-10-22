//! ESP-HAL board feature owner wiring.
//! Wiring owner feature board ESP-HAL.

#[path = "board_feature.rs"]
mod implementation;
pub(super) use implementation::for_board;
