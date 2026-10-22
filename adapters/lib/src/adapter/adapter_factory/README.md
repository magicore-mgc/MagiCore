# adapter_factory

## Responsibility

Public language-detecting and explicit-language adapter constructors.

## Ghi chú tiếng Việt

Folder này chỉ sở hữu phần public language-detecting and explicit-language adapter constructors. trong adapter đa ngôn ngữ. Mọi lỗi tiếp tục dùng `MgError`/`MgResult` chung; không tạo lớp lỗi song song. Các test hiện có giữ nguyên trong `src/test/` và sẽ được sắp xếp ở phase test sau khi refactor source toàn repo hoàn tất.
