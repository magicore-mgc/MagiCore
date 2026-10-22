# native_policy

## Responsibility

Language ownership and fail-closed native resolve eligibility.

## Ghi chú tiếng Việt

Folder này chỉ sở hữu phần language ownership and fail-closed native resolve eligibility. trong adapter đa ngôn ngữ. Mọi lỗi tiếp tục dùng `MgError`/`MgResult` chung; không tạo lớp lỗi song song. Các test hiện có giữ nguyên trong `src/test/` và sẽ được sắp xếp ở phase test sau khi refactor source toàn repo hoàn tất.
