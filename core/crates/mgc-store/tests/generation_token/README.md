# Generation-token test owners

This directory holds behavior implementations and shared fixtures for the stable Cargo target `test_generation_token`. The parent `tests/generation_token.rs` owns all 24 harness registrations and delegates each test once.

Thư mục này chứa implementation theo hành vi và fixture dùng chung cho Cargo target ổn định `test_generation_token`. File cha `tests/generation_token.rs` giữ đủ 22 đăng ký harness và chuyển mỗi test đúng một lần.

S328 adds direct SQL checks for orphan-claim rejection on fresh and migrated schemas; S329 fixes migration detection so a generation-key table missing its FK is rebuilt. S330 removes the public raw-connection bypass, keeps fixture SQL behind `test-util`, and routes the doctor through a read-only integrity count API. The previous S327 wording is historical. This source-first batch does not run tests, compiler, formatter, CI, or OS verification.

S328 thêm kiểm tra SQL trực tiếp việc từ chối claim mồ côi trên schema mới và schema sau migration; S329 sửa điều kiện migration để rebuild bảng có khóa generation nhưng thiếu FK. S330 bỏ raw-connection bypass công khai, chỉ mở SQL fixture sau `test-util` và chuyển doctor sang API đếm integrity chỉ đọc. Nội dung S327 là ghi nhận lịch sử. Batch source-first chưa chạy test, compiler, formatter, CI hay kiểm chứng OS.
