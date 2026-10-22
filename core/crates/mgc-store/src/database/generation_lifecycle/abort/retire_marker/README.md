# Staging marker retirement

Owns the final staging-marker deletion and affected-row guard for an abort.

## Trách nhiệm

Quản lý thao tác xóa staging marker cuối cùng và guard số row bị tác động của abort.

The caller has already deleted this generation's claims inside the same immediate transaction. Preserve the conditional delete and diagnostic.

Caller đã xóa claim generation này trong cùng transaction immediate. Giữ nguyên điều kiện delete và thông báo lỗi.
