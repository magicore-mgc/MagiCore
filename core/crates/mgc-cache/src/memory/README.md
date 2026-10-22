# Cache memory utilities / Tiện ích bộ nhớ cache

This module exports memory-oriented helpers used by cache code.
Module này xuất các tiện ích bộ nhớ dành cho code cache.

`StringPool` retains entries until `clear` is called. No in-repository production caller currently uses it; changing its retention policy needs a separately defined capacity contract.
`StringPool` giữ entry đến khi gọi `clear`. Hiện chưa có production caller trong repo; thay đổi chính sách giữ dữ liệu cần có hợp đồng giới hạn dung lượng riêng.
