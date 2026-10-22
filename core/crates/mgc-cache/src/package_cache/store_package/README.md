# Store package / Lưu package

Copies input bytes into a uniquely named `create_new` staging file, syncs it, verifies streaming BLAKE3, and atomically replaces the v2 entry.
Chép dữ liệu vào staging có tên riêng bằng `create_new`, sync, xác minh BLAKE3 theo luồng rồi thay entry v2 theo cách atomic.

An existing or planted staging symlink cannot be followed because the staging file must be created exclusively. Failed operations remove their staging path on drop.
Symlink staging có sẵn hoặc bị cài không thể bị đi theo vì file staging phải được tạo độc quyền. Khi lỗi, đường dẫn staging được dọn lúc drop.
