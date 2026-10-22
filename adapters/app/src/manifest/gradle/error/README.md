# Gradle manifest error owner

This private error type classifies locally constructed Gradle policy/update failures and unwraps original `MgError` values at the public boundary.
Lỗi private này phân loại lỗi policy/update do owner tạo ra và trả lại `MgError` gốc ở ranh giới công khai.

It keeps the existing display text, `MgError` variant, and underlying source for read/write failures.
Nó giữ nguyên nội dung hiển thị, variant `MgError` và source gốc của lỗi đọc/ghi.
