# Cache integrity / Integrity cache

Owns streaming BLAKE3 integrity calculation and uses the platform no-follow regular-file opener.
Sở hữu việc tính integrity BLAKE3 theo luồng và dùng hàm platform mở file thường không-follow.

The established `blake3:<hex>` output and fixed 64 KiB working buffer remain unchanged.
Định dạng `blake3:<hex>` và buffer làm việc cố định 64 KiB được giữ nguyên.
