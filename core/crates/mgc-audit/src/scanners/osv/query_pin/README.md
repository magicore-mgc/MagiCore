# OSV pin query

Owns one pinned package request and vulnerability detail materialization for the OSV audit lane.

## Phạm vi

Folder này sở hữu one pinned package request and vulnerability detail materialization trong lane audit OSV. File `query_pin.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
