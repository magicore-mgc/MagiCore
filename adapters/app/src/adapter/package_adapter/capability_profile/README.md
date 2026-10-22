# capability_profile

## Responsibility

Choose the runtime capability slice by App language. The public core-level arrays remain declared on `AppAdapter`; this selector must not claim a resolver for lanes that do not own native resolution.

## Ghi chú tiếng Việt

Folder này chọn tập capability runtime theo ngôn ngữ App. Các mảng công khai cấp core vẫn khai báo trên `AppAdapter`; selector không được claim resolver cho lane chưa sở hữu resolve native.
