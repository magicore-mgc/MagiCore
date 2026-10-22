//! tools_list owner wiring.
//! File này chỉ nối module tools_list và export hàm xử lý.

mod tools_list;
pub(in crate::commands::mcp) use tools_list::tools_list;
