//! method_not_found owner wiring.
//! File này chỉ nối module method_not_found và export hàm xử lý.

mod method_not_found;
pub(in crate::commands::mcp) use method_not_found::method_not_found;
