# macOS clonefile tests

Checks that an embedded NUL in a path is rejected as `InvalidInput` before invoking the native syscall. Test body: `macos_clone.rs`.

# Test clonefile macOS

Kiểm tra NUL giữa chuỗi trong path bị từ chối với `InvalidInput` trước khi gọi syscall native. Nội dung test: `macos_clone.rs`.
