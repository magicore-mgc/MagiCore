# Read Swift resolved lockfile

Owns v1/v2 package.resolved parsing and pin/skipped classification for the OSV audit lane.

## Phạm vi

Folder này sở hữu v1/v2 package.resolved parsing and pin/skipped classification trong lane audit OSV. File `read_swift_resolved.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
