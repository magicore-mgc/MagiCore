//! resource_read owner wiring.
//! File này chỉ nối module resource_read và export hàm xử lý.

mod resource_read;
pub(in crate::commands::mcp) use resource_read::resource_read;
