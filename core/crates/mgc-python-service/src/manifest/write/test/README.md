# Project writer tests

`write.rs` tests preservation of inactive project metadata and verifies that unsupported dependency groups fail before the original manifest is modified.

## Phạm vi test

`write.rs` kiểm tra writer giữ metadata project chưa được chọn để cài và xác nhận group ngoài phạm vi bị từ chối trước khi sửa file gốc.
