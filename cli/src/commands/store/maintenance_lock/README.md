# Store maintenance lock

This owner acquires the same fail-fast project file lock as Web installation. It uses `.magicore/cache/web/locks` as the lock directory and `.magicore/cache/web` (`Layout::root()`) as the hashed project key. Prune and restore must hold the returned RAII guard across their entire read/mutation sequence. The underlying path-based lock setup can create its directory and is not protected against a hostile concurrent path swap.

# Khóa bảo trì store

Owner này lấy đúng project file lock fail-fast mà Web installation dùng. Thư mục lock là `.magicore/cache/web/locks`; project key được hash là `.magicore/cache/web` (`Layout::root()`). Prune và restore phải giữ RAII guard suốt chuỗi đọc/thay đổi. Cơ chế path-based có thể tạo thư mục lock và chưa chống được path bị thay đồng thời có chủ đích.
