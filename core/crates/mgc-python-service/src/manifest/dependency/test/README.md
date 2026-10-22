# Dependency conversion tests

`dependency.rs` tests the currently modeled operators, rejects requirement syntax that cannot be represented without loss, and verifies exact-pin/star serialization. This is a source-owner test suite; it is not a claim that Python package resolution has been exercised.

## Phạm vi test

`dependency.rs` kiểm tra toán tử đang mô hình hóa, từ chối cú pháp không thể biểu diễn nguyên vẹn, và xác nhận serialize pin chính xác/range `*`. Đây là test tại source owner; chưa chứng minh resolver Python chạy thực tế.
