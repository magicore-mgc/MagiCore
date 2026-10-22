//! Wires portable atomic replacement and the Windows-specific implementation.
//! Nối thay file atomic portable và implementation riêng cho Windows.

mod replace_file;
pub use self::replace_file::atomic_replace_file;
