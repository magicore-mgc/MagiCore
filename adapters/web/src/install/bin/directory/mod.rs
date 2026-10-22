//! Wiring for `.bin` directory preparation and stale-link cleanup.
//! Wiring cho chuẩn bị thư mục `.bin` và dọn link cũ.

mod directory;
pub(super) use directory::prepare_bin_directory;
