# Journal parser / Parser journal

This folder validates the existing versioned journal shape. Its local error preserves `serde_json::Error` as the source and renders the existing diagnostic text. Version rejection remains before typed field parsing.

Folder này xác thực cấu trúc journal có version. Lỗi cục bộ giữ `serde_json::Error` làm source và hiển thị diagnostic hiện có. Kiểm tra version vẫn chạy trước parse field typed.
