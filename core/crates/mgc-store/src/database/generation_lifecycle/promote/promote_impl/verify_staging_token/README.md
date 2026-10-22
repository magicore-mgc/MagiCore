# Staging token verification

Owns the read-only state check before CAS generation promotion mutates rows.

## Trách nhiệm

Quản lý bước đọc và kiểm tra state trước khi promote generation CAS thay đổi dữ liệu.

`false` means the token is already promoted and the caller must return without mutation; an unknown token remains a typed error. Keep the exact SQL and error mapping unchanged.

`false` nghĩa là token đã promoted và caller phải dừng trước mutation; token không tồn tại tiếp tục là lỗi có type. Giữ nguyên SQL và ánh xạ lỗi.
