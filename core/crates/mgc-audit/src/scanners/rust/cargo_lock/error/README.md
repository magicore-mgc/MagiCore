# Cargo.lock parser errors

EN: Defines typed failures for malformed Cargo.lock structure and preserves each legacy display message at the shared `MgError` boundary. I/O and OSV errors remain owned elsewhere.

VI: Định nghĩa lỗi typed cho cấu trúc Cargo.lock malformed và giữ từng thông báo cũ tại ranh giới `MgError` dùng chung. Lỗi I/O và OSV vẫn do owner khác quản lý.
