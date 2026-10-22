# Native NuGet install

Owns NuGet package verification and global-packages materialization.

## Chủ sở hữu

Sở hữu xác minh package NuGet và materialize vào global-packages.

`error.rs` owns the lane-local error boundary and converts to the existing shared `MgError` categories. NuGet currently has no owner-local behavior tests; add them under `test/` when its test cases are introduced.

`error.rs` giữ ranh giới lỗi cục bộ và chuyển về các loại `MgError` dùng chung hiện tại. NuGet chưa có test hành vi riêng; khi bổ sung test, đặt chúng trong `test/`.
