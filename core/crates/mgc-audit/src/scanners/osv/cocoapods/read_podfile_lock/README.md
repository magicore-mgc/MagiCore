# Read CocoaPods lockfile

Owns pods section parsing into pins and explicit skipped entries for the OSV audit lane.

## Phạm vi

Folder này sở hữu pods section parsing into pins and explicit skipped entries trong lane audit OSV. File `read_podfile_lock.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
