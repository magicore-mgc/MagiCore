# Audit CocoaPods inventory

Owns bounded podfile.lock input and honest unsupported-ecosystem reporting for the OSV audit lane.

## Phạm vi

Folder này sở hữu bounded podfile.lock input and honest unsupported-ecosystem reporting trong lane audit OSV. File `audit_cocoapods_osv.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
