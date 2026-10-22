# Lock contention / Tranh chấp lock

This owner recognizes `WouldBlock` on all platforms and Windows `ERROR_LOCK_VIOLATION`. Permission, filesystem, and unrelated errors remain immediate failures.

Owner này nhận diện `WouldBlock` trên mọi platform và `ERROR_LOCK_VIOLATION` trên Windows. Lỗi quyền, filesystem hoặc lỗi khác vẫn thất bại ngay.
