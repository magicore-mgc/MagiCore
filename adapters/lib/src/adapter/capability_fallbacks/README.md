# capability_fallbacks

## Responsibility

Unclaimed capabilities and TypeScript-only materializer/lifecycle claims.

## Ghi chú tiếng Việt

Folder này chỉ sở hữu phần unclaimed capabilities and typescript-only materializer/lifecycle claims. trong adapter đa ngôn ngữ. Mọi lỗi tiếp tục dùng `MgError`/`MgResult` chung; không tạo lớp lỗi song song. Các test hiện có giữ nguyên trong `src/test/` và sẽ được sắp xếp ở phase test sau khi refactor source toàn repo hoàn tất.
