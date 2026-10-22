# Open Regular File Owner

**Purpose:** Provide the shared, typed OS primitive for opening a regular file without following a final symlink or reparse point.

**Mục đích:** Cung cấp primitive OS dùng chung, có lỗi typed, để mở file thường mà không đi theo symlink/reparse cuối.

`open_regular_file_no_follow` returns the open handle after validating the opened object. Unix uses `O_NOFOLLOW | O_NONBLOCK`; Windows uses `FILE_FLAG_OPEN_REPARSE_POINT` and checks handle attributes. Unsupported OS targets fail closed. Parent path components are outside this helper's guarantee and must be confined by the caller.

`error.rs` preserves typed symlink, regular-file, reparse and I/O failure categories. `test/` covers regular reads, directory rejection, final symlink rejection and opened-handle stability. Rust code is authored but NOT_RUN until the repository-wide source pass closes.

`open_regular_file_no_follow` trả handle sau khi xác minh object đã mở. Unix dùng `O_NOFOLLOW | O_NONBLOCK`; Windows dùng `FILE_FLAG_OPEN_REPARSE_POINT` và kiểm tra thuộc tính handle. OS chưa hỗ trợ sẽ fail-closed. Helper không bảo vệ các thành phần cha; caller phải tự giới hạn chúng.

`error.rs` giữ nhóm lỗi typed cho symlink, file thường, reparse và I/O. `test/` kiểm tra đọc file thường, từ chối thư mục/link cuối và độ ổn định handle sau khi đổi tên path. Code đã viết nhưng NOT_RUN đến khi hoàn tất source pass toàn repo.
