//! Audit CocoaPods inventory owner wiring.
//! File này chỉ nối module audit CocoaPods và export API công khai.

mod audit_cocoapods_osv;
pub use audit_cocoapods_osv::audit_cocoapods_osv;
