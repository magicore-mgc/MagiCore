# Build Swift OSV pin

Owns version/location validation and honest skip classification for one swift pin for the OSV audit lane.

## Phạm vi

Folder này sở hữu version/location validation and honest skip classification for one swift pin trong lane audit OSV. File `push_swift_pin.rs` chứa phần thực thi; `mod.rs` chỉ khai báo module và export API cần thiết.

The owner uses the shared `MgError` / `MgResult` contract; it does not duplicate the crate-wide error type. Existing error class and diagnostic construction stay at the call site.
