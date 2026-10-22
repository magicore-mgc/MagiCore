//! Core-hint resolution wiring — nối chuẩn hóa, kiểm tra và lỗi cục bộ.
//! Wiring giải quyết core hint — nối chuẩn hóa, kiểm tra và lỗi owner.

#[path = "core_hints.rs"]
mod core_hints;

pub(in crate::dispatch) use core_hints::resolve;
