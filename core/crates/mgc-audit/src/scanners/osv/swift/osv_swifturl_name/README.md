# Normalize SwiftURL package name

Owns normalization of a git location to the osv swifturl identity for the OSV audit lane.

## Phạm vi

Folder này sở hữu normalization of a git location to the osv swifturl identity trong lane audit OSV. File `osv_swifturl_name.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
