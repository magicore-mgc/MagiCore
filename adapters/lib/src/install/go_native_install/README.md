# Native Go install

Downloads, verifies, stores, and materializes Go modules in the managed module cache.

## Chủ sở hữu

Tải, xác minh, lưu và materialize module Go trong module cache do MGC quản lý.

`error.rs` owns the lane-local error boundary and converts to the existing shared `MgError` categories. Go currently has no owner-local behavior tests; add them under `test/` when its test cases are introduced.

`error.rs` giữ ranh giới lỗi cục bộ và chuyển về các loại `MgError` dùng chung hiện tại. Go chưa có test hành vi riêng; khi bổ sung test, đặt chúng trong `test/`.
