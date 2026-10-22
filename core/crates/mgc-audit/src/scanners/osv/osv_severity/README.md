# OSV severity mapping

Owns severity precedence and cvss fallback for the OSV audit lane.

## Phạm vi

Folder này sở hữu severity precedence and cvss fallback trong lane audit OSV. File `osv_severity.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
