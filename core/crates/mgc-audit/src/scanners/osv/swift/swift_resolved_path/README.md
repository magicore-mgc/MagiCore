# Locate Swift resolved lockfile

Owns bounded package.resolved lookup at project root, ios, and one parent level for the OSV audit lane.

## Phạm vi

Folder này sở hữu bounded package.resolved lookup at project root, ios, and one parent level trong lane audit OSV. File `swift_resolved_path.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
