# External Python manifest guard

`external_manifest.rs` lists dependency/lock files owned by other Python workflows and returns the first matching filename, case-normalized. It checks presence only, does not parse or execute the file, and preserves the caller's fail-closed choice before ambient-runtime fallback. Read-directory and entry errors have a local type and retain the previous public `MgError::Other` message.

## Guard manifest Python bên ngoài

`external_manifest.rs` nhận diện file dependency/lock thuộc workflow Python khác và trả filename khớp đầu tiên ở dạng chữ thường. Module chỉ kiểm tra sự hiện diện, không parse/chạy file; caller tiếp tục fail-closed trước ambient-runtime fallback. Lỗi đọc directory/entry có type cục bộ và giữ nguyên thông báo `MgError::Other` công khai.
