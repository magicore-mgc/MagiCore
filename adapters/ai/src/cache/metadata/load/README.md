# Metadata loading

This owner distinguishes an absent sidecar from inspection/read/parse errors. Existing sidecars are opened through the shared no-follow regular-file helper; only a missing sidecar uses the legacy `CacheEntry::from_path("unknown", path)` fallback.

Owner này phân biệt sidecar thiếu với lỗi inspect/read/parse. Sidecar tồn tại được mở qua helper shared no-follow cho file thường; chỉ khi thiếu mới dùng fallback cũ `CacheEntry::from_path("unknown", path)`.

Nearest fallback, parse and symlink cases are in `test/load.rs`.

Test fallback, parse và symlink gần nhất nằm ở `test/load.rs`.
