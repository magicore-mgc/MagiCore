//! Tarball entry DTO owner wiring. / Wiring owner DTO tarball.
//! Re-exports the unchanged data type from its same-name implementation.
//! Export lại kiểu dữ liệu không đổi từ implementation cùng tên.

#[path = "tarball_entry.rs"]
mod implementation;
pub use self::implementation::TarballEntry;
