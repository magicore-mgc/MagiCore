# SafeTensors scanner

This owner reads the eight-byte header length, rejects the existing 100 MiB metadata threshold, and reports truncated headers. It does not load tensor payloads or validate the complete JSON/tensor semantic structure. File I/O failures propagate through MgResult.

Owner này đọc độ dài header 8 byte, áp dụng ngưỡng metadata 100 MiB hiện có và báo header bị cắt. Nó không nạp payload tensor hoặc kiểm tra đầy đủ cấu trúc JSON/ngữ nghĩa tensor. Lỗi I/O truyền qua MgResult.

Existing tests are registered in test/ and remain runtime-unverified in this source-only phase.

Test hiện có được đăng ký trong test/ và chưa xác minh runtime trong phase chỉ sửa source này.
