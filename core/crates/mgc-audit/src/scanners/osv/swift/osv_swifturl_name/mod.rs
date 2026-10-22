//! Normalize SwiftURL package name owner wiring.
//! File này chỉ nối module chuẩn hóa tên package SwiftURL.

mod osv_swifturl_name;
pub(in crate::scanners::osv) use osv_swifturl_name::osv_swifturl_name;
