# Rust build lanes / Lane build Rust

Owns ordinary offline Cargo build and the MGC-managed Cargo source override. Both use the shared BuildRunner; dependency resolution/network access is not added here.

Sở hữu Cargo build offline thông thường và override source Cargo do MGC quản lý. Cả hai dùng BuildRunner chung; không thêm resolve dependency/truy cập mạng ở đây.

`RustBuildError` owns the invalid managed-source path diagnostic; Cargo/executor failures preserve shared source chains. Build tests remain deferred.

`RustBuildError` sở hữu diagnostic managed-source path không hợp lệ; lỗi Cargo/executor giữ source chain dùng chung. Test build còn chờ.
