# Get package / Lấy package

Selects the v2 entry first, then a safe legacy entry, and verifies BLAKE3 before returning either path.
Ưu tiên entry v2, sau đó mới đến entry cũ an toàn, và xác minh BLAKE3 trước khi trả path.

`has_package` checks for a regular file without claiming that its integrity has been verified; callers that consume bytes must use `get_package`.
`has_package` chỉ kiểm file thường, không xác nhận integrity; caller dùng dữ liệu phải gọi `get_package`.
