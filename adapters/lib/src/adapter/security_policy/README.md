# security_policy

## Responsibility

Per-core release-age policy parsing and unsupported-lane gates.

## Ghi chú tiếng Việt

Folder này chỉ sở hữu phần per-core release-age policy parsing and unsupported-lane gates. trong adapter đa ngôn ngữ. Mọi lỗi tiếp tục dùng `MgError`/`MgResult` chung; không tạo lớp lỗi song song. Các test hiện có giữ nguyên trong `src/test/` và sẽ được sắp xếp ở phase test sau khi refactor source toàn repo hoàn tất.
