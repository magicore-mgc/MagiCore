# error

Owns typed retrieval failures and the compatibility conversion to the existing `MgError` categories. Cache reads/writes remain store errors, registry failures remain network errors, semaphore closure remains an other error, and integrity failures pass through unchanged.

# error / lỗi fetch

Quản lý lỗi fetch có kiểu và chuyển đổi tương thích về các nhóm `MgError` hiện có. Lỗi đọc/ghi cache vẫn là store, lỗi registry vẫn là network, semaphore đóng vẫn là other, và lỗi integrity được giữ nguyên.
