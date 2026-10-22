# Archive entry scan / Quét entry archive

Owns gzip/tar entry classification and bounded-memory file hashing. It skips directory and GNU metadata records, rejects links/unsupported records, sanitizes each regular-file path, checks bytes read against the declared size, and returns the same ordered input records as the former inline implementation.

Sở hữu phân loại entry gzip/tar và hash file với bộ nhớ hữu hạn. Bỏ directory/metadata GNU, từ chối link/record không hỗ trợ, làm sạch path file thường, đối chiếu bytes đã đọc với size khai báo và trả cùng dữ liệu đầu vào như implementation cũ.

`test/entry_scan.rs` exercises a real regular-file tar record. Diagnostic text remains `MgError::Other`; malformed/link entries must continue to fail closed.

`test/entry_scan.rs` chạy qua một tar record file thường thật. Diagnostic giữ `MgError::Other`; entry lỗi/link tiếp tục phải fail-closed.
