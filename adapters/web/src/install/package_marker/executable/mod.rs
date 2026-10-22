//! Platform projection of executable metadata into marker signatures.
//! Chuyển metadata executable theo nền tảng thành dữ liệu chữ ký marker.

mod executable;

pub(crate) use executable::{metadata_is_executable, mode_is_executable};
