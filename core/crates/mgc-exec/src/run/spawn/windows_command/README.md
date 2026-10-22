# Windows spawn command

Owns in-process Windows command lookup and safe `.cmd`/`.bat` dispatch. `mod.rs` exposes helpers to the runner and scanner tests; `windows_command.rs` resolves caller PATH, normalizes UTF-16 paths, validates shell syntax, and chooses only `System32` tools; `error.rs` defines redacted failure categories; `test/` covers precedence, path handling and rejection; `docs/` tracks the owner.

# Chuẩn bị lệnh spawn Windows

Sở hữu việc dò lệnh Windows trong process và gọi `.cmd`/`.bat` an toàn. `mod.rs` export helper cho runner và test scanner; `windows_command.rs` dò PATH do caller truyền, chuẩn hóa path UTF-16, kiểm tra cú pháp shell và chỉ chọn tool trong `System32`; `error.rs` định nghĩa nhóm lỗi đã lược dữ liệu; `test/` kiểm tra thứ tự ưu tiên, path và từ chối cú pháp; `docs/` theo dõi owner.
