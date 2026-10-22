//! Memory detection owner exports.
//! (Xuất owner phát hiện bộ nhớ.)
mod memory;
mod os;
mod parse;
pub(in crate::commands::optimizer) use memory::detect_memory_gb;
