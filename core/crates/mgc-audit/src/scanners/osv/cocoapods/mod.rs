//! CocoaPods inventory and OSV capability owners.
//! Các owner kiểm kê CocoaPods và khả năng OSV.

mod audit_cocoapods_osv;
mod cocoapods;
mod read_podfile_lock;

pub use audit_cocoapods_osv::audit_cocoapods_osv;
pub(super) use cocoapods::report_cocoapods_inventory;
pub use read_podfile_lock::read_podfile_lock;
