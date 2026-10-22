# record_verification / xác minh RECORD

Checks that each installed member matches the authenticated archive, then validates the declared SHA-256 and size. Reads through an opened regular-file handle with a fixed 64-KiB buffer; it does not allocate another `Vec` sized to the member.

Kiểm tra member đã cài khớp archive xác thực, sau đó xác minh SHA-256 và size đã khai báo. Đọc qua handle file thường với buffer cố định 64 KiB, không cấp phát thêm `Vec` bằng kích thước member.
