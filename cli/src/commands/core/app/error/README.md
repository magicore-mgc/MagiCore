# App-local errors

Owns `AppCoreError`, its stable English display text, and the crate-visible `project_not_detected_error` and `native_lifecycle_error` builders. Keep these diagnostics here because both project helpers and App lifecycle commands use them; do not duplicate variants in command folders.

# Lỗi cục bộ của App

Sở hữu `AppCoreError`, nội dung English ổn định và hai hàm crate-visible `project_not_detected_error`, `native_lifecycle_error`. Giữ diagnostic tại đây vì helper project và lệnh lifecycle App cùng dùng; không nhân bản variant ở folder lệnh.
