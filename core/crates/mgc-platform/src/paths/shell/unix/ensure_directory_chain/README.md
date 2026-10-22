# Safe Unix shell config directory

Owns creation of a Fish configuration directory chain under the selected home, rejecting symlink and non-directory components. Public behavior is exercised through `core/crates/mgc-platform/tests/paths.rs`.

## Vietnamese

Owner này tạo chuỗi thư mục cấu hình Fish bên trong home đã chọn, từ chối thành phần là symlink hoặc không phải thư mục. Hành vi public được kiểm tra qua `core/crates/mgc-platform/tests/paths.rs`.
