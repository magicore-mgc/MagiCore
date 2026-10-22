# Resolve-first tests

`pinning.rs` checks that an unpinned Python request becomes an exact PEP 508 pin, other native lanes retain the bare resolved version, and explicit requested ranges stay unchanged. Registry resolution integration remains in the existing Lib native-engine tests.

## Phạm vi

`pinning.rs` kiểm tra request Python chưa pin được chuyển thành exact PEP 508, các native lane khác giữ version resolve dạng trần, và range người dùng nhập được bảo toàn. Integration resolve registry vẫn nằm trong Lib native-engine tests hiện tại.
