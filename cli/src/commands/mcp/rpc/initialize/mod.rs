//! initialize owner wiring.
//! File này chỉ nối module initialize và export hàm xử lý.

mod initialize;
pub(in crate::commands::mcp) use initialize::initialize;
