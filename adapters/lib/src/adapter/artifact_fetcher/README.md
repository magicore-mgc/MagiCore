# artifact_fetcher

## Responsibility

Standalone fetch capability and explicit non-native fail-closed behavior.

## Ghi chú tiếng Việt

Folder này chỉ sở hữu phần standalone fetch capability and explicit non-native fail-closed behavior. trong adapter đa ngôn ngữ. Mọi lỗi tiếp tục dùng `MgError`/`MgResult` chung; không tạo lớp lỗi song song. Các test hiện có giữ nguyên trong `src/test/` và sẽ được sắp xếp ở phase test sau khi refactor source toàn repo hoàn tất.
