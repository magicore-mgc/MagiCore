# Executable permissions / Quyền executable

Preserves the existing Unix mode-bit inspection and `0755`/`0644` update. Non-Unix continues to report `false` and makes permission updates a no-op.

Giữ cách kiểm tra mode bit trên Unix và cập nhật `0755`/`0644` hiện tại. Non-Unix tiếp tục trả `false` và bỏ qua cập nhật permission.

Filesystem errors retain the original `MgError::Io` type. No direct tests are added before source decomposition closes; test mode behavior on Unix and no-op behavior on Windows in the later verification phase. `cargo test -p mgc-web-adapter` was not run.

Lỗi filesystem giữ nguyên loại `MgError::Io`. Chưa thêm test trực tiếp trước khi source decomposition đóng; phase verify sau cần test mode trên Unix và no-op trên Windows. Chưa chạy `cargo test -p mgc-web-adapter`.
