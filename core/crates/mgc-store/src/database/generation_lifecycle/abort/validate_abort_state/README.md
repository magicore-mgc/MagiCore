# Abort state validation

Owns the token-state gate before an abort can delete claims or its staging marker.

## Trách nhiệm

Quản lý cổng kiểm tra state trước khi abort được phép xóa claim hoặc staging marker.

Missing tokens remain idempotent no-ops; promoted tokens remain typed errors. Keep this check inside the caller's immediate transaction.

Token không còn tiếp tục là no-op idempotent; token promoted tiếp tục là lỗi có type. Giữ bước kiểm tra này trong transaction immediate của caller.
