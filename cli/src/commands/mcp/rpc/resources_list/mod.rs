//! resources_list owner wiring.
//! File này chỉ nối module resources_list và export hàm xử lý.

mod resources_list;
pub(in crate::commands::mcp) use resources_list::resources_list;
