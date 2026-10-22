//! Wire search-only package classification and its nearest tests.
//! Nối logic phân loại package riêng cho search với test gần nhất.

#[path = "classification.rs"]
mod classification;

pub(super) use classification::detect_core_label;
