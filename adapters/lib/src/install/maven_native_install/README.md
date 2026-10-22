# Native Maven install

Owns Maven artifact and POM verification and materialization.

## Chủ sở hữu

Sở hữu xác minh và materialize artifact Maven cùng POM.

`error.rs` owns the lane-local error boundary and converts to the existing shared `MgError` categories. Maven currently has no owner-local behavior tests; add them under `test/` when its test cases are introduced.

`error.rs` giữ ranh giới lỗi cục bộ và chuyển về các loại `MgError` dùng chung hiện tại. Maven chưa có test hành vi riêng; khi bổ sung test, đặt chúng trong `test/`.
