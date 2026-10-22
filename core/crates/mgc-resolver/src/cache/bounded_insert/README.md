# Bounded cache insertion / Ghi cache có giới hạn

Owns admission for one concurrent registry map: prune expired entries, replace the same key, then evict the oldest fetched entry before adding a new key at capacity. Each map has an independent insertion gate, so version and dependency writes do not block each other.

Sở hữu admission cho một registry map đồng thời: dọn entry hết hạn, thay key trùng, rồi xóa entry được tải lâu nhất trước khi thêm key mới khi map đầy. Mỗi map có insertion gate riêng nên ghi version và dependency không chặn lẫn nhau.
