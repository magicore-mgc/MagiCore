# Python lock owner

Reads the configured MGC lock schema, returns an empty result only when the project has no locked lane, and rejects missing locks when the Python manifest declares dependencies.
`error.rs` owns lock schema, inspection, and missing-lock failures; the owner preserves existing shared error categories and diagnostic text.

## Lock owner Python

Đọc schema lock MGC đã cấu hình, chỉ trả kết quả rỗng khi project chưa có lane lock; từ chối thiếu lock nếu manifest Python khai báo dependency.
`error.rs` sở hữu lỗi schema, đọc lock và thiếu lock; owner giữ nguyên nhóm lỗi chung cùng nội dung chẩn đoán hiện tại.
