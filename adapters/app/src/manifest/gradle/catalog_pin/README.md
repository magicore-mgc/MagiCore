# Version-catalog pin updater

This owner rewrites one matched catalog version, preserves existing TOML decoration through `toml_edit`, refuses shared references, and verifies the saved value by reading it back.
Owner này sửa một version catalog phù hợp, giữ decoration TOML qua `toml_edit`, từ chối reference dùng chung và xác minh giá trị đã lưu bằng cách đọc lại.

The matching rule, first-hit selection, `false` results, and existing error variants/messages remain unchanged in this structural-only move.
Quy tắc tìm entry, chọn kết quả đầu tiên, kết quả `false` và variant/nội dung lỗi hiện hành được giữ nguyên trong lần di chuyển cấu trúc này.
