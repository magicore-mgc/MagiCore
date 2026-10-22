# Tests for `process_isolation`

These tests validate that the selected platform configuration can be prepared without starting a child process. Actual process-tree kill/cancellation behavior remains covered by the crate's OS-specific integration tests.

Các test này xác nhận cấu hình nền tảng được chuẩn bị mà không chạy process con. Hành vi kill/cancel cây process thật tiếp tục do integration test theo OS của crate kiểm tra.

`mod.rs` registers `process_isolation.rs` as a named child module and forwards the owner-private contract names it uses.
`mod.rs` đăng ký `process_isolation.rs` thành module con có tên và nối các tên contract riêng của owner mà test dùng.
