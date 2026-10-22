# Cache-presence errors

This owner preserves filesystem inspection failures as typed errors with their original I/O source. A missing path remains the normal `false` result.

Owner này giữ lỗi inspect filesystem thành lỗi typed có source I/O gốc. Path chưa tồn tại vẫn trả kết quả `false` bình thường.

Nearest error-chain coverage is in `test/error.rs`.

Test error-chain gần nhất ở `test/error.rs`.
