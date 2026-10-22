# Windows reparse inspection / Kiểm tra reparse Windows

This cfg-gated owner checks the Windows reparse attribute for junctions and mount points. It uses the same `GetFileAttributesW` primitive and error-to-false behavior as the previous implementation.

Owner cfg-gated này kiểm tra reparse attribute Windows cho junction và mount point. Owner giữ primitive `GetFileAttributesW` và hành vi lỗi-thành-false như implementation cũ.
