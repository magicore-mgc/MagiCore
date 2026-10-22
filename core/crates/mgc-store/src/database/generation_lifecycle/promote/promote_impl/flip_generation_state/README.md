# CAS generation state flip

Owns the final staging-to-promoted update and its affected-row integrity guard.

## Trách nhiệm

Quản lý câu update cuối để đổi staging thành promoted cùng kiểm tra tính toàn vẹn số row.

This owner runs inside the transaction established by `promote/`. Preserve the exact update, one-row condition, and diagnostic.

Owner này chạy bên trong transaction do `promote/` mở. Giữ nguyên câu update, điều kiện đúng một row và thông báo lỗi.
