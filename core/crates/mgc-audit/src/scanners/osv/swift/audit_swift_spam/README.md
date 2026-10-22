# Audit Swift Package Manager

Owns package.resolved discovery, bounded input, audit status and skipped-pin aggregation for the OSV audit lane.

## Phạm vi

Folder này sở hữu package.resolved discovery, bounded input, audit status and skipped-pin aggregation trong lane audit OSV. File `audit_swift_spam.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
